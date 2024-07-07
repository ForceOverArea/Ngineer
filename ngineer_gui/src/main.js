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
import { addFileToFilesRibbonMenu } from "./modules/leftRibbon.mjs";
import { addFileToProjectRibbonMenu } from "./modules/projectRibbon.mjs";
// import { listen } from "@tauri-apps/api/event";

export { addFileToFilesRibbonMenu, addFileToProjectRibbonMenu };

// ========================= Constants ========================= //
const { invoke } = window.__TAURI__.tauri;

// ========================= Initialization ========================= //
document.getElementById(FILES_TAB).onclick = activateFilesTab;
document.getElementById(MODEL_TAB).onclick = activateModelTab;

// await listen("new-file-button-clicked", (event) => { displayOnDebugRibbon(event.payload.message); });

window.addEventListener(
    "DOMContentLoaded", 
    () => 
    {
        document.querySelector("#leftRibbon")
            .addEventListener(
                "new-file-button-clicked",
                (event) => 
                {
                    event.preventDefault();
                    displayOnDebugRibbon("ligma");
                }
            );
    }
);
