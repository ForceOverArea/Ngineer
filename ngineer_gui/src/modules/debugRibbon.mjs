/*************************************************************
 * 
 *        File: debugRibbon.mjs
 *
 * Description: Contains functions and constants used for
 *              adding dynamic behavior to the debug ribbon.
 * 
 *************************************************************/

// ========================= Constants ========================= //
/**
 * The ID attribute of the Debug Ribbon HTML element.
 */
export const DEBUG_RIBBON  = "debugRibbon";

// ========================= Functions ========================= // 
/**
 * Writes a message to the debug ribbon
 * @param {String} msg 
 */
export function displayOnDebugRibbon(msg)
{
    let debugRibbon = document.getElementById(DEBUG_RIBBON);

    if (null === debugRibbon)
    {
        throw "failed to locate debug ribbon element!";
    }

    debugRibbon.innerText = msg;
}