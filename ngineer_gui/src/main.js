/*************************************************************
 * 
 *        File: main.js
 *
 * Description: Contains functions and constants used for
 *              adding dynamic behavior to the debug ribbon.
 * 
 *************************************************************/

// ========================= Imports ========================= //
import { activateFilesTab, activateModelTab, FILES_TAB, MODEL_TAB } from "./modules/leftRibbon.mjs";
import { displayOnDebugRibbon } from "./modules/debugRibbon.mjs";
// import { open } from "@tauri-apps/api/dialog";
import { appWindow } from "@tauri-apps/api/window";
displayOnDebugRibbon("imported modules successfully.");

// ========================= Constants ========================= //
const { invoke } = window.__TAURI__.tauri;

// ========================= Initialization ========================= //
document.getElementById(FILES_TAB).onclick = activateFilesTab;
document.getElementById(MODEL_TAB).onclick = activateModelTab;

window.addEventListener(
    "DOMContentLoaded", 
    () => 
    {
        window.listen("new-file-button-clicked",
        (event) => 
        {
            event.preventDefault();
            displayOnDebugRibbon("ligma");
        });
    }
);
