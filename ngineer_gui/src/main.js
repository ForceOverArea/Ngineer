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

// ========================= Constants ========================= //
const { invoke } = window.__TAURI__.tauri;

// ========================= Initialization ========================= //
document.getElementById(FILES_TAB).onclick = activateFilesTab;
document.getElementById(MODEL_TAB).onclick = activateModelTab;

document

window.addEventListener("DOMContentLoaded", () => {
    greetInputEl = document.querySelector("#greet-input");
    greetMsgEl = document.querySelector("#greet-msg");
    document.querySelector("#greet-form").addEventListener("submit", (e) => {
        e.preventDefault();
        greet();
    });
});
