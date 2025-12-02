mod capture;
mod output;
mod segmentation;

use anyhow::{Context, Result};
use capture::{CaptureSource, WebcamCapture};
use clap::Parser;
use output::{OutputSink, V4L2Output};
use segmentation::{Preprocessor, SegmentationModel};
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input webcam device index
    #[arg(short, long, default_value_t = 0)]
    input_device: u32,

    /// Output v4l2loopback device path
    #[arg(short, long, default_value = "/dev/video10")]
    output_device: String,

    /// Capture resolution width
    #[arg(long, default_value_t = 1920)]
    capture_width: u32,

    /// Capture resolution height
    #[arg(long, default_value_t = 1080)]
    capture_height: u32,

    /// Output resolution width
    #[arg(long, default_value_t = 1280)]
    output_width: u32,

    /// Output resolution height
    #[arg(long, default_value_t = 720)]
    output_height: u32,

    /// Target frames per second
    #[arg(long, default_value_t = 30)]
    fps: u32,

    /// Enable debug logging
    #[arg(long)]
    debug: bool,

    /// Path to segmentation model (ONNX file)
    /// If not provided, runs in passthrough mode without segmentation
    #[arg(long)]
    model: Option<String>,

    /// Show matte visualization (grayscale silhouette) instead of original video
    #[arg(long)]
    show_matte: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .init();

    tracing::info!("Camola starting");
    tracing::info!("Capture: {}x{}", args.capture_width, args.capture_height);
    tracing::info!("Output: {}x{}", args.output_width, args.output_height);
    tracing::info!("Target FPS: {}", args.fps);

    // Initialize capture
    let mut capture = WebcamCapture::new(
        args.input_device,
        args.capture_width,
        args.capture_height,
    )
    .context("Failed to initialize webcam capture")?;

    // Initialize output
    let mut output = V4L2Output::new(&args.output_device, args.output_width, args.output_height)
        .context("Failed to initialize v4l2loopback output")?;

    // Initialize segmentation model if provided
    let model: Option<Box<dyn SegmentationModel>> = if let Some(model_path) = &args.model {
        tracing::info!("Loading segmentation model from {}", model_path);
        let model = segmentation::create_default_model(model_path)
            .context("Failed to load segmentation model")?;
        tracing::info!("Segmentation model loaded successfully");
        Some(model)
    } else {
        tracing::info!("Running in passthrough mode (no segmentation)");
        None
    };

    // Main loop
    run_pipeline(&mut capture, &mut output, model, args.fps, args.show_matte)?;

    Ok(())
}

fn run_pipeline<C, O>(
    capture: &mut C,
    output: &mut O,
    mut model: Option<Box<dyn SegmentationModel>>,
    target_fps: u32,
    show_matte: bool,
) -> Result<()>
where
    C: CaptureSource,
    O: OutputSink,
{
    let frame_duration = Duration::from_secs_f32(1.0 / target_fps as f32);
    let mut frame_count = 0u64;
    let mut total_capture_time = Duration::ZERO;
    let mut total_segment_time = Duration::ZERO;
    let mut total_output_time = Duration::ZERO;

    tracing::info!("Starting main pipeline loop");
    if model.is_some() {
        tracing::info!(
            "Segmentation enabled, show_matte={}",
            show_matte
        );
    }
    tracing::info!("Press Ctrl+C to stop");

    loop {
        let loop_start = Instant::now();

        // Capture frame
        let capture_start = Instant::now();
        let frame = capture
            .capture_frame()
            .context("Failed to capture frame")?;
        let capture_time = capture_start.elapsed();
        total_capture_time += capture_time;

        // Segmentation (if model is loaded)
        let output_frame = if let Some(ref mut model) = model {
            let segment_start = Instant::now();
            let matte = model
                .segment(&frame)
                .context("Failed to segment frame")?;
            let segment_time = segment_start.elapsed();
            total_segment_time += segment_time;

            if show_matte {
                // Visualize matte as grayscale image
                let (width, height) = frame.dimensions();
                Preprocessor::matte_to_rgb(&matte, width, height)
            } else {
                // For now, just pass through the original frame
                // TODO: In Milestone 3, we'll composite foreground onto backgrounds
                frame
            }
        } else {
            // Passthrough mode
            frame
        };

        // Output frame
        let output_start = Instant::now();
        output
            .write_frame(&output_frame)
            .context("Failed to write frame")?;
        let output_time = output_start.elapsed();
        total_output_time += output_time;

        frame_count += 1;

        // Log stats every 30 frames
        if frame_count % 30 == 0 {
            let avg_capture_ms = total_capture_time.as_secs_f64() * 1000.0 / frame_count as f64;
            let avg_segment_ms = total_segment_time.as_secs_f64() * 1000.0 / frame_count as f64;
            let avg_output_ms = total_output_time.as_secs_f64() * 1000.0 / frame_count as f64;
            let total_ms = avg_capture_ms + avg_segment_ms + avg_output_ms;
            let actual_fps = 1000.0 / total_ms;

            if model.is_some() {
                tracing::info!(
                    "Frame {}: capture={:.1}ms, segment={:.1}ms, output={:.1}ms, total={:.1}ms, fps={:.1}",
                    frame_count,
                    avg_capture_ms,
                    avg_segment_ms,
                    avg_output_ms,
                    total_ms,
                    actual_fps
                );
            } else {
                tracing::info!(
                    "Frame {}: capture={:.1}ms, output={:.1}ms, total={:.1}ms, fps={:.1}",
                    frame_count,
                    avg_capture_ms,
                    avg_output_ms,
                    total_ms,
                    actual_fps
                );
            }
        }

        // Frame rate limiting
        let elapsed = loop_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
}
