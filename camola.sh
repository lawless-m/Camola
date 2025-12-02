#!/bin/bash
# Camola - GPU-accelerated webcam effects for virtual camera
# Usage:
#   camola on   - Start virtual camera with default effects
#   camola off  - Stop virtual camera
#   camola      - Run in foreground (Ctrl+C to stop)

CAMOLA_DIR="/home/matt/Git/Camola"
PYTHON="$CAMOLA_DIR/venv/bin/python3"
SCRIPT="$CAMOLA_DIR/camola.py"
MODEL="$CAMOLA_DIR/webcam-fx/models/modnet-webcam.onnx"

case "$1" in
    help|--help|-h)
        cat << 'EOF'
CAMOLA - GPU-Accelerated Virtual Webcam Effects

USAGE:
    camola on              Start virtual camera in background
    camola off             Stop virtual camera
    camola status          Check if virtual camera is running
    camola                 Run in foreground (Ctrl+C to stop)
    camola [OPTIONS]       Run with custom options

COMMANDS:
    on          Start in background with default effects
    off         Stop the virtual camera
    status      Check running status
    help        Show this help message

DEFAULT EFFECTS:
    --plasma                           Animated plasma background
    --foreground-opacity 0.85          85% opacity (slightly transparent)
    --foreground-saturation 1.3        30% saturation boost

BACKGROUND OPTIONS:
    --plasma                           Plasma effect background (procedural animation)
    --plasma-speed FLOAT               Animation speed multiplier (default: 1.0)
    --plasma-scale FLOAT               Pattern scale (default: 0.02, smaller = tighter)
    --plasma-palette PALETTE           Color palette: classic, fire, ocean, neon, monochrome

    --pixelate-background              Pixelate the background
    --pixel-size SIZE                  Pixel block size (default: 16)
    --invert-background                Invert background colors

    --background-color RRGGBB          Solid color background (hex, e.g., 00FF00)
    --background-image PATH            Static image as background
    --background-video PATH            Video file as animated background

FOREGROUND OPTIONS:
    --foreground-opacity FLOAT         Your opacity, 0.0-1.0 (default: 1.0, fully opaque)
    --foreground-saturation FLOAT      Color saturation multiplier (default: 1.0)
    --foreground-effect EFFECT         Apply effect: blur, cartoon, sketch

TRAILS/GHOSTING EFFECT:
    --trails                           Enable motion trails/ghosting
    --trails-interval FRAMES           Frames between trail captures (default: 5)
    --trails-count COUNT               Number of trail frames (default: 6)
    --trails-fade-start FLOAT          Oldest trail opacity (default: 0.1)
    --trails-fade-end FLOAT            Newest trail opacity (default: 0.5)
    --trails-pixelate                  Apply pixelation to trails
    --trails-hue-shift DEGREES         Hue shift per trail, 0-180 (default: 0)

CAMERA OPTIONS:
    --input-device INDEX               Webcam device number (default: 0)
    --output-device PATH               Virtual camera device (default: /dev/video10)
    --capture-width WIDTH              Input resolution width (default: 1920)
    --capture-height HEIGHT            Input resolution height (default: 1080)
    --output-width WIDTH               Output resolution width (default: 1280)
    --output-height HEIGHT             Output resolution height (default: 720)
    --fps FPS                          Target frame rate (default: 30)

MODEL OPTIONS:
    --model PATH                       ONNX model path (required)

EXAMPLES:
    # Default plasma effect with transparency and saturation
    camola on

    # Fire palette plasma with full opacity
    camola --plasma --plasma-palette fire --foreground-opacity 1.0

    # Pixelated inverted background
    camola --pixelate-background --pixel-size 32 --invert-background

    # Motion trails with rainbow hue shift
    camola --trails --trails-pixelate --trails-hue-shift 30

    # Solid green screen background
    camola --background-color 00FF00

    # Semi-transparent with cartoon effect
    camola --foreground-opacity 0.7 --foreground-effect cartoon

FILES:
    /dev/video10                       Virtual camera output device
    ~/Git/Camola/                      Installation directory
    ~/Git/Camola/webcam-fx/models/     Model directory

NOTES:
    - Requires v4l2loopback kernel module loaded
    - Uses GPU acceleration via CUDA when available
    - Model must be provided (default: modnet-webcam.onnx)
    - All effects can be combined

EOF
        exit 0
        ;;

    on)
        # Check if already running
        if pgrep -f "camola.py" > /dev/null; then
            echo "Camola is already running"
            exit 0
        fi

        echo "Starting Camola virtual camera..."
        cd "$CAMOLA_DIR"
        nohup stdbuf -oL "$PYTHON" -u "$SCRIPT" \
            --model "$MODEL" \
            --plasma --foreground-opacity 0.85 --foreground-saturation 1.3 \
            > /dev/null 2>&1 &

        sleep 1
        if pgrep -f "camola.py" > /dev/null; then
            echo "Camola started successfully"
        else
            echo "Failed to start Camola"
            exit 1
        fi
        ;;

    off)
        if pgrep -f "camola.py" > /dev/null; then
            echo "Stopping Camola..."
            pkill -f "camola.py"
            sleep 1
            if pgrep -f "camola.py" > /dev/null; then
                echo "Force stopping..."
                pkill -9 -f "camola.py"
            fi
            echo "Camola stopped"
        else
            echo "Camola is not running"
        fi
        ;;

    status)
        if pgrep -f "camola.py" > /dev/null; then
            echo "Camola is running"
            exit 0
        else
            echo "Camola is not running"
            exit 1
        fi
        ;;

    *)
        # Run in foreground (no arguments or custom arguments)
        cd "$CAMOLA_DIR"
        if [ $# -eq 0 ]; then
            exec stdbuf -oL "$PYTHON" -u "$SCRIPT" \
                --model "$MODEL" \
                --plasma --foreground-opacity 0.85 --foreground-saturation 1.3
        else
            exec stdbuf -oL "$PYTHON" -u "$SCRIPT" \
                --model "$MODEL" \
                "$@"
        fi
        ;;
esac
