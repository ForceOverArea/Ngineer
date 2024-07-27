import { TagGetter, TagBuilder, registerElementId, generateUniqueElementId } from './tagbuilder.js'
import { DEBUG_RIBBON } from './common.js'
import { createLeftRibbonMenuFileItem } from './leftRibbonMenu.js'

/**
 * Prints a message to the debug ribbon at the 
 * bottom of the Ngineer UI.
 * @param msg the message to display on the debug ribbon.
 */
function writeToDebugRibbon(msg: string): void
{
    DEBUG_RIBBON.getter().innerText = msg;
}
