#!/bin/sh
# animation.sh - Single-instance animation screensaver

# Prevent multiple instances
PIDFILE="/tmp/animation.pid"
if [ -f "$PIDFILE" ]; then
    if ps -p $(cat "$PIDFILE") > /dev/null 2>&1; then
        echo "Animation already running, exiting"
        exit 0
    fi
fi
echo $$ > "$PIDFILE"

# Clean up on exit
trap 'rm -f "$PIDFILE"; exit' INT TERM EXIT

# Define your animation directories
ANIMATIONS="
/var/local/hacks/videos/rude_dance
"

cd /var/local/hacks/bin/

# Run animation only ONCE (not infinite loop)
for animation_dir in $ANIMATIONS; do
    if [ ! -d "$animation_dir" ]; then
        echo "Skipping missing directory: $animation_dir"
        continue
    fi
    
    if ! ls "$animation_dir"/test_frame_* 1> /dev/null 2>&1; then
        echo "Skipping $animation_dir - no frame files found"
        continue
    fi
    
    echo "Playing: $(basename "$animation_dir")"
    
    # Run the animation once
    if ! ./kindle_ascii "$animation_dir" 1; then
        echo "Error playing $animation_dir"
        continue
    fi
    
    break  # Only play first valid animation
done

# Clean up
rm -f "$PIDFILE"
