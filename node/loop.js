// node myFile.js

const pendingTimers = [];
const pendingOSTasks = [];
const pendingOperations = [];

// New timers, tasks & operations are recorded from myFile running
myFile.runContents();

function shouldContinue() {
    // check 1: Any pending setTimeout, setInterval, setImmediate?
    // check 2: Any pending OS task? (like server listening to port)?
    // check 3: Any pending long running operation? (like fs module)?
    return pendingTimers.length || pendingOSTasks.length || pendingOperations.length;

}

while (shouldContinue()) {
    // entire body executed in one tick
    // 1) Node looks at pendingTimers to see if any function is to run (setTimeout, setInterval)
    // 2) Node looks at pendingOSTask & pendingOperations and call relelvant callbacks
    // 3) Pause execution. Continue when..
    //      - a new pendingOSTask is done
    //      - a new pendingOperation is done
    //      - a timer is about to complete.
    // 4) Look for pendingTimers (call any setImmediate)
    // 5) Handle any 'close' events

}

// exit back to terminal