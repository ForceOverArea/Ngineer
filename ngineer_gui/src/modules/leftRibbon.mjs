/*************************************************************
 * 
 *        File: leftRibbon.mjs
 *
 * Description: Contains functions and constants used for
 *              adding dynamic behavior to the left ribbon 
 *              menu.
 * 
 *************************************************************/

// ========================= Imports ========================= //
import { displayOnDebugRibbon } from "./debugRibbon.mjs";
import { BG_COLORS } from "./colors.mjs";
import { addFileToProjectRibbonMenu } from "./projectRibbon.mjs";

// ========================= Constants ========================= //
/**
 * ID attribute of the left ribbon menu's "files" tab
 */
export const FILES_TAB = "filesTab";

/**
 * ID attribute of the left ribbon menu's "model" tab
 */
export const MODEL_TAB = "modelTab";

/**
 * ID attribute of the left ribbon menu
 */
export const LEFT_RIBBON_MENU = "leftRibbon";

// ========================= Classes ========================= // 

/**
 * 
 */
class LeftRibbonMenuState
{
    constructor()
    {
        if (undefined === this.instance)
        {

        }
    }
}

// ========================= Functions ========================= // 
/**
 * Activates either the "Files" tab or the "Model" tab based on the boolean flag passed in.
 * @param {boolean} activateFiles
 * @param {Event} event
 */
function activateTab(activateFiles, event)
{
    let filesTab = document.getElementById(FILES_TAB);
    let modelTab = document.getElementById(MODEL_TAB);

    if (null === filesTab || null === modelTab)
    {
        displayOnDebugRibbon("failed to locate the files and/or model tab(s).");
    }

    if (activateFiles)
    {
        displayOnDebugRibbon("showing files ribbon.");
        modelTab.style.backgroundColor = BG_COLORS.dark;
        filesTab.style.backgroundColor = BG_COLORS.lightest;
    }
    else
    {
        displayOnDebugRibbon("showing model ribbon.");
        modelTab.style.backgroundColor = BG_COLORS.lightest;
        filesTab.style.backgroundColor = BG_COLORS.dark;
    }
}

/**
 * Activates the "Files" tab in the left ribbon menu.
 * @param {Event} event 
 */
export function activateFilesTab(event)
{
    activateTab(true, event);
}

/**
 * Activates the "Model" tab in the left ribbon menu.
 * @param {Event} event 
 */
export function activateModelTab(event)
{
    activateTab(false, event);
}

/**
 * Adds a file to the list of files in the left ribbon
 * menu.
 * @param {String} file 
 */
export function addFileToFilesRibbonMenu(file)
{
    let tag = newTag("div", file,
        "leftRibbonMenuFileItem", 
        `addFileToProjectRibbonMenu('${file}')`
    );
    document.getElementById("leftRibbon").innerHTML = tag;
}