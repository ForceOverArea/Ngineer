import { DEBUG_RIBBON } from './common.js';
function writeToDebugRibbon(msg) {
    DEBUG_RIBBON.getter().innerText = msg;
}
