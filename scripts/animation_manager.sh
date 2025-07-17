#!/bin/sh
# animation_manager.sh - Cycles through multiple animations

# List of available animations
ANIMATIONS="
/var/local/hacks/videos/rude_dance
/var/local/hacks/videos/em_dance
"

# Track which animation to play next
COUNTER_FILE="/tmp/animation_counter"

# Initialize counter if it doesn't exist
if [ ! -f "$COUNTER_FILE" ]; then
    echo "0" > "$COUNTER_FILE"
fi

# Read current counter
COUNTER=$(cat "$COUNTER_FILE")

# Convert animations list to array
ANIM_ARRAY=""
for anim in $ANIMATIONS; do
    ANIM_ARRAY="$ANIM_ARRAY $anim"
done

# Count total animations
TOTAL=0
for anim in $ANIM_ARRAY; do
    TOTAL=$((TOTAL + 1))
done

# Select animation based on counter
CURRENT=0
SELECTED=""
for anim in $ANIM_ARRAY; do
    if [ $CURRENT -eq $COUNTER ]; then
        SELECTED="$anim"
        break
    fi
    CURRENT=$((CURRENT + 1))
done

# Increment counter for next time (wrap around)
NEXT_COUNTER=$((COUNTER + 1))
if [ $NEXT_COUNTER -ge $TOTAL ]; then
    NEXT_COUNTER=0
fi
echo "$NEXT_COUNTER" > "$COUNTER_FILE"

echo "Playing animation: $(basename "$SELECTED")" >> /tmp/watchdog_debug.log

# Change to binary directory and run the selected animation
cd /var/local/hacks/bin/
while true; do
    ./kindle_ascii "$SELECTED" 1
    sleep 0.5
done
