#!/usr/bin/env python3
"""
Camola - GPU-accelerated webcam background replacement
Pure Python implementation for maximum performance
"""
import sys
import time
import argparse
import numpy as np
import cv2
import onnxruntime as ort
from PIL import Image
import fcntl

class CamolaGPU:
    def __init__(self, model_path, input_device=0, output_device="/dev/video10",
                 capture_width=1920, capture_height=1080,
                 output_width=1280, output_height=720,
                 background_color=None, background_image=None, background_video=None,
                 pixelate_background=False, pixel_size=16,
                 invert_background=False,
                 foreground_effect=None, fps=30):

        self.output_width = output_width
        self.output_height = output_height
        self.target_fps = fps
        self.frame_duration = 1.0 / fps

        # Effect settings
        self.pixelate_background = pixelate_background
        self.pixel_size = pixel_size
        self.invert_background = invert_background
        self.foreground_effect = foreground_effect

        # Temporal smoothing for matte stability
        self.prev_matte = None
        self.temporal_alpha = 0.7  # Blend factor: higher = more weight on current frame

        # Initialize webcam capture
        print(f"Initializing webcam {input_device} at {capture_width}x{capture_height}", flush=True)
        self.cap = cv2.VideoCapture(input_device)
        self.cap.set(cv2.CAP_PROP_FRAME_WIDTH, capture_width)
        self.cap.set(cv2.CAP_PROP_FRAME_HEIGHT, capture_height)
        self.cap.set(cv2.CAP_PROP_FPS, fps)

        if not self.cap.isOpened():
            raise RuntimeError("Failed to open webcam")

        actual_width = int(self.cap.get(cv2.CAP_PROP_FRAME_WIDTH))
        actual_height = int(self.cap.get(cv2.CAP_PROP_FRAME_HEIGHT))
        print(f"Webcam opened: {actual_width}x{actual_height}")

        # Initialize v4l2loopback output
        print(f"Opening v4l2loopback device at {output_device} ({output_width}x{output_height})")
        self.output_device = open(output_device, 'wb')

        # Set format using v4l2 ioctl
        import struct
        VIDIOC_S_FMT = 0xc0d05605
        V4L2_BUF_TYPE_VIDEO_OUTPUT = 2
        V4L2_PIX_FMT_YUYV = 0x56595559  # YUYV 4:2:2 format (standard webcam format)

        # Build v4l2_format structure (208 bytes)
        fmt = bytearray(208)
        struct.pack_into('I', fmt, 0, V4L2_BUF_TYPE_VIDEO_OUTPUT)  # type
        struct.pack_into('I', fmt, 8, output_width)  # width
        struct.pack_into('I', fmt, 12, output_height)  # height
        struct.pack_into('I', fmt, 16, V4L2_PIX_FMT_YUYV)  # pixelformat

        fcntl.ioctl(self.output_device, VIDIOC_S_FMT, fmt)
        print("v4l2loopback device configured")

        # Load segmentation model with CUDA
        print(f"Loading segmentation model: {model_path}")
        providers = ['CUDAExecutionProvider', 'CPUExecutionProvider']
        self.session = ort.InferenceSession(model_path, providers=providers)
        print(f"ONNX Runtime providers: {self.session.get_providers()}")
        self.input_name = self.session.get_inputs()[0].name

        # Prepare background
        self.background_video_cap = None
        if background_color:
            # Parse hex color
            color_hex = background_color.lstrip('#')
            b = int(color_hex[4:6], 16)
            g = int(color_hex[2:4], 16)
            r = int(color_hex[0:2], 16)
            self.background = np.full((output_height, output_width, 3), [b, g, r], dtype=np.uint8)
            print(f"Using solid color background: {background_color}")
        elif background_image:
            bg = cv2.imread(background_image)
            if bg is None:
                raise RuntimeError(f"Failed to load background image: {background_image}")
            self.background = cv2.resize(bg, (output_width, output_height))
            print(f"Using image background: {background_image}")
        elif background_video:
            self.background_video_cap = cv2.VideoCapture(background_video)
            if not self.background_video_cap.isOpened():
                raise RuntimeError(f"Failed to load background video: {background_video}")
            self.background = None  # Will be updated each frame
            print(f"Using video background: {background_video}")
        else:
            self.background = None
            print("No background replacement")

        print("Camola GPU initialized")

    def segment(self, frame):
        """Run segmentation on frame and return alpha matte"""
        h, w = frame.shape[:2]

        # Convert BGR to RGB
        frame_rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)

        # Resize to model input size (320x320)
        frame_resized = cv2.resize(frame_rgb, (320, 320))

        # Normalize and convert to NCHW
        frame_normalized = frame_resized.astype(np.float32) / 255.0
        frame_tensor = np.transpose(frame_normalized, (2, 0, 1))[np.newaxis, ...]

        # Run inference
        output = self.session.run(None, {self.input_name: frame_tensor})[0]

        # Resize matte back to original size
        matte = cv2.resize(output[0, 0], (w, h))

        # Apply Gaussian blur to soften edges
        matte = cv2.GaussianBlur(matte, (7, 7), 0)

        # Optional: Apply morphological operations to clean up the matte
        # Slight erosion followed by dilation to remove small noise
        kernel = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (3, 3))
        matte = cv2.erode(matte, kernel, iterations=1)
        matte = cv2.dilate(matte, kernel, iterations=1)

        # Apply another small blur after morphological ops
        matte = cv2.GaussianBlur(matte, (5, 5), 0)

        # Temporal smoothing to reduce flicker
        if self.prev_matte is not None:
            # Exponential moving average: blend current with previous
            matte = self.temporal_alpha * matte + (1 - self.temporal_alpha) * self.prev_matte

        # Store for next frame
        self.prev_matte = matte.copy()

        return matte

    def apply_foreground_effect(self, frame):
        """Apply artistic effect to foreground (person)"""
        if self.foreground_effect == 'cartoon':
            # Cartoon effect using stylization
            cartoon = cv2.stylization(frame, sigma_s=150, sigma_r=0.25)
            return cartoon
        elif self.foreground_effect == 'sketch':
            # Pencil sketch effect (grayscale)
            sketch_gray, sketch_color = cv2.pencilSketch(frame, sigma_s=60, sigma_r=0.07, shade_factor=0.05)
            return sketch_color
        elif self.foreground_effect == 'sketch_bw':
            # Black and white sketch
            sketch_gray, sketch_color = cv2.pencilSketch(frame, sigma_s=60, sigma_r=0.07, shade_factor=0.05)
            return cv2.cvtColor(sketch_gray, cv2.COLOR_GRAY2BGR)
        return frame

    def get_background_frame(self):
        """Get next frame from background video (with looping)"""
        if self.background_video_cap is None:
            return self.background

        ret, bg_frame = self.background_video_cap.read()
        if not ret:
            # End of video, loop back to start
            self.background_video_cap.set(cv2.CAP_PROP_POS_FRAMES, 0)
            ret, bg_frame = self.background_video_cap.read()
            if not ret:
                # Fallback to None if still failing
                return None

        # Resize to output size
        bg_frame = cv2.resize(bg_frame, (self.output_width, self.output_height))
        return bg_frame

    def composite(self, frame, matte):
        """Composite foreground onto background using matte"""
        # Resize frame to output size
        frame_resized = cv2.resize(frame, (self.output_width, self.output_height))
        matte_resized = cv2.resize(matte, (self.output_width, self.output_height))

        # Expand matte to 3 channels
        matte_3ch = np.stack([matte_resized] * 3, axis=-1)

        # Apply foreground effect if specified
        if self.foreground_effect:
            # Apply effect to the entire frame
            effected_frame = self.apply_foreground_effect(frame_resized)
            # Use effected version for foreground
            foreground = effected_frame
        else:
            foreground = frame_resized

        # Pixelate background effect
        if self.pixelate_background:
            # Create pixelated version of the frame
            h, w = frame_resized.shape[:2]
            # Downscale to pixelate
            small = cv2.resize(frame_resized, (w // self.pixel_size, h // self.pixel_size),
                             interpolation=cv2.INTER_LINEAR)
            # Upscale back using nearest neighbor to keep blocky pixels
            pixelated = cv2.resize(small, (w, h), interpolation=cv2.INTER_NEAREST)

            # Apply color inversion if requested
            if self.invert_background:
                pixelated = 255 - pixelated

            # Composite: effected foreground + pixelated background
            composited = (foreground * matte_3ch + pixelated * (1 - matte_3ch)).astype(np.uint8)
            return composited

        # Get background (static or video frame)
        background = self.get_background_frame()

        # Background replacement
        if background is None:
            # No background replacement, just return with foreground effect applied
            composited = (foreground * matte_3ch + frame_resized * (1 - matte_3ch)).astype(np.uint8)
            return composited

        # Composite: effected foreground + background
        composited = (foreground * matte_3ch + background * (1 - matte_3ch)).astype(np.uint8)

        return composited

    def write_frame(self, frame):
        """Write frame to v4l2loopback device"""
        # Ensure correct size
        if frame.shape[:2] != (self.output_height, self.output_width):
            frame = cv2.resize(frame, (self.output_width, self.output_height))

        # Convert BGR to YUYV (YUV 4:2:2 packed) format for browser compatibility
        frame_yuyv = cv2.cvtColor(frame, cv2.COLOR_BGR2YUV_YUY2)

        # Write to device
        self.output_device.write(frame_yuyv.tobytes())
        self.output_device.flush()

    def run(self):
        """Main processing loop"""
        print("Starting main pipeline loop")
        print(f"Segmentation enabled, background={'yes' if self.background is not None else 'no'}")
        print("Press Ctrl+C to stop")

        frame_count = 0
        total_capture_time = 0.0
        total_segment_time = 0.0
        total_composite_time = 0.0
        total_output_time = 0.0

        try:
            while True:
                loop_start = time.time()

                # Capture frame
                capture_start = time.time()
                ret, frame = self.cap.read()
                if not ret:
                    print("Failed to capture frame")
                    break
                capture_time = time.time() - capture_start
                total_capture_time += capture_time

                # Segmentation
                segment_start = time.time()
                matte = self.segment(frame)
                segment_time = time.time() - segment_start
                total_segment_time += segment_time

                # Compositing
                composite_start = time.time()
                output_frame = self.composite(frame, matte)
                composite_time = time.time() - composite_start
                total_composite_time += composite_time

                # Output
                output_start = time.time()
                self.write_frame(output_frame)
                output_time = time.time() - output_start
                total_output_time += output_time

                frame_count += 1

                # Log stats every 30 frames
                if frame_count % 30 == 0:
                    avg_capture_ms = (total_capture_time / frame_count) * 1000
                    avg_segment_ms = (total_segment_time / frame_count) * 1000
                    avg_composite_ms = (total_composite_time / frame_count) * 1000
                    avg_output_ms = (total_output_time / frame_count) * 1000
                    total_ms = avg_capture_ms + avg_segment_ms + avg_composite_ms + avg_output_ms
                    actual_fps = 1000.0 / total_ms

                    print(f"Frame {frame_count}: capture={avg_capture_ms:.1f}ms, "
                          f"segment={avg_segment_ms:.1f}ms, composite={avg_composite_ms:.1f}ms, "
                          f"output={avg_output_ms:.1f}ms, total={total_ms:.1f}ms, fps={actual_fps:.1f}")

                # Frame rate limiting
                elapsed = time.time() - loop_start
                if elapsed < self.frame_duration:
                    time.sleep(self.frame_duration - elapsed)

        except KeyboardInterrupt:
            print("\nStopping...")
        finally:
            self.cap.release()
            if self.background_video_cap is not None:
                self.background_video_cap.release()
            self.output_device.close()
            print("Camola stopped")

def main():
    parser = argparse.ArgumentParser(description="Camola - GPU-accelerated webcam background replacement")
    parser.add_argument("--model", required=True, help="Path to ONNX segmentation model")
    parser.add_argument("--input-device", type=int, default=0, help="Input webcam device index")
    parser.add_argument("--output-device", default="/dev/video10", help="Output v4l2loopback device path")
    parser.add_argument("--capture-width", type=int, default=1920, help="Capture resolution width")
    parser.add_argument("--capture-height", type=int, default=1080, help="Capture resolution height")
    parser.add_argument("--output-width", type=int, default=1280, help="Output resolution width")
    parser.add_argument("--output-height", type=int, default=720, help="Output resolution height")
    parser.add_argument("--fps", type=int, default=30, help="Target frames per second")
    parser.add_argument("--background-color", help="Solid color background in hex (e.g., 00FF00)")
    parser.add_argument("--background-image", help="Background image file path")
    parser.add_argument("--background-video", help="Background video file path (loops automatically)")
    parser.add_argument("--pixelate-background", action="store_true", help="Pixelate background instead of replacing")
    parser.add_argument("--pixel-size", type=int, default=16, help="Pixel block size for pixelation effect (default: 16)")
    parser.add_argument("--invert-background", action="store_true", help="Invert colors in background")
    parser.add_argument("--foreground-effect", choices=['cartoon', 'sketch', 'sketch_bw'], help="Apply artistic effect to foreground (you)")

    args = parser.parse_args()

    camola = CamolaGPU(
        model_path=args.model,
        input_device=args.input_device,
        output_device=args.output_device,
        capture_width=args.capture_width,
        capture_height=args.capture_height,
        output_width=args.output_width,
        output_height=args.output_height,
        background_color=args.background_color,
        background_image=args.background_image,
        background_video=args.background_video,
        pixelate_background=args.pixelate_background,
        pixel_size=args.pixel_size,
        invert_background=args.invert_background,
        foreground_effect=args.foreground_effect,
        fps=args.fps
    )

    camola.run()

if __name__ == "__main__":
    main()
