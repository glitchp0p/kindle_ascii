#!/bin/sh
#
# Improved screensaver-watchdog-helper with debugging
#

# Get hackname from the script's path
KH_HACKNAME="${0##/mnt/us/}"
KH_HACKNAME="${KH_HACKNAME%%/bin/*}"

# Try to pull our custom helper lib
_KH_FUNCS="/mnt/us/${KH_HACKNAME}/bin/libkh"
if [ -f ${_KH_FUNCS} ] ; then
   . ${_KH_FUNCS}
else
   # Pull default helper functions for logging
   _FUNCTIONS=/etc/rc.d/functions
   [ -f ${_FUNCTIONS} ] && . ${_FUNCTIONS}
   # We couldn't get our custom lib, abort
   msg "couldn't source libkh from '${KH_HACKNAME}'" W
   exit 0
fi

# Make sure shlock is exec'able
[ -x ${SSWD_LOCK_BIN} ] || chmod +x ${SSWD_LOCK_BIN}

# Make sure our lockfile has somewhere to live
[ -d ${SSWD_LOCK_DIR} ] || mkdir -p ${SSWD_LOCK_DIR}

# Add the PID of the lipc-wait-event(s) to the list of running daemons to kill
echo "$( pidof lipc-wait-event )" >> ${SS_WATCHDOG_PID}

# Add our PID to the list of running daemons to kill
echo "$$" >> ${SS_WATCHDOG_PID}

# The event we're hooking has no args on FW 2.x...
if [ "${IS_K2}" == "true" ] ; then
   goto_ss_event="goingToScreenSaver"
else
   goto_ss_event="goingToScreenSaver [23]"
fi

while read line ; do
   # Did we really go to screensaver?
   if echo "${line}" | grep -q "${goto_ss_event}" ; then
       echo "DEBUG: Sleep event detected" >> /tmp/watchdog_debug.log
       
       # Wait for Amazon's screensaver to load first
       sleep 2
       
       # Kill any existing animation processes
       pkill -f "kindle_ascii" >> /tmp/watchdog_debug.log 2>&1
       pkill -f "animation.sh" >> /tmp/watchdog_debug.log 2>&1
       
       # Clear the screen
       eips -c >> /tmp/watchdog_debug.log 2>&1
       
       # Start our animation with full path and logging
       echo "DEBUG: Starting animation at $(date)" >> /tmp/watchdog_debug.log
       cd /var/local/hacks/bin/
       ./kindle_ascii "/var/local/hacks/videos/rude_dance" 1 >> /tmp/watchdog_debug.log 2>&1 &
       
       echo "DEBUG: Animation started with PID $!" >> /tmp/watchdog_debug.log
   fi
done
