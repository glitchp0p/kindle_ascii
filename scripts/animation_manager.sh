#!/bin/sh
# animation_manager.sh - Runs for set time then exits

# Animation timeout (in seconds) - adjust as needed
TIMEOUT=300  # 5 minutes

# Record start time
START_TIME=$(date +%s)

echo "Animation started, will run for ${TIMEOUT} seconds"

# List of animations
ANIMATIONS="
/var/local/hacks/videos/rude_dance
/var/local/hacks/videos/em_dance
"

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

# Run animation with timeout check
cd /var/local/hacks/bin/
while true; do
    # Check if we've exceeded timeout
    CURRENT_TIME=$(date +%s)
    ELAPSED=$((CURRENT_TIME - START_TIME))
    
    if [ $ELAPSED -ge $TIMEOUT ]; then
        echo "Animation timeout reached (${ELAPSED}s), exiting"
        break
    fi
    
    ./kindle_ascii "$SELECTED" 1
    sleep 0.5
done

echo "Animation manager exiting cleanly"
