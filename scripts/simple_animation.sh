#!/bin/sh
# simple_animation.sh - Single, clean animation script

# Prevent multiple instances
if [ -f "/tmp/anim.lock" ]; then
    exit 0
fi
echo "$$" > /tmp/anim.lock
trap 'rm -f /tmp/anim.lock; exit' INT TERM EXIT

# List of animations
ANIMS="/var/local/hacks/videos/rude_dance /var/local/hacks/videos/em_dance"

# Get counter
CNT_FILE="/tmp/anim_cnt"
if [ -f "$CNT_FILE" ]; then
    CNT=$(cat "$CNT_FILE")
else
    CNT=0
fi

# Select animation (0=rude_dance, 1=em_dance)
if [ "$CNT" = "0" ]; then
    SELECTED="/var/local/hacks/videos/rude_dance"
    echo "1" > "$CNT_FILE"
else
    SELECTED="/var/local/hacks/videos/em_dance"  
    echo "0" > "$CNT_FILE"
fi

echo "Playing: $(basename "$SELECTED")"

# Run animation continuously 
cd /var/local/hacks/bin/
while true; do
    ./kindle_ascii "$SELECTED" 1
    sleep 0.5
done

rm -f /tmp/anim.lock
