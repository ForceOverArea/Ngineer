/*************************************************************
 * 
 *        File: projectRibbon.mjs
 *
 * Description: Contains functions and constants used for
 *              adding dynamic behavior to the project ribbon.
 * 
 *************************************************************/

// ========================= Imports ========================= //
import { BG_COLORS } from "./colors.mjs";
import { displayOnDebugRibbon } from "./debugRibbon.mjs";
import { newTag } from "./formatting.mjs";

// ========================= Constants ========================= //
/**
 * The ID attribute of the project ribbon.
 */
export const PROJECT_RIBBON = "projTabsBar";

// ========================= Functions ========================= // 
/**
 * Adds a file to the list of files in the project ribbon.
 * @param {String} file 
 */
export function addFileToProjectRibbonMenu(file)
{
    let tag = newTag("div", file, "projectRibbonFileItem");
    document.getElementById(PROJECT_RIBBON).innerHTML += tag;
    displayOnDebugRibbon(`focusing file: ${file}`);
}