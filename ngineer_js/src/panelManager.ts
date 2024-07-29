import { CommonBGColors, FILES_TAB, LEFT_RIBBON, MODEL_TAB } from "./common.js";
import { DebugRibbon } from "./debugRibbon.js";

export const SCREEN_PANEL_MANAGER: string[] = [];

export const LEFT_PANEL_MANAGER = 
{
    filesTabState: '', // This is the default tab. It's text is determined by the hard-coded html value.
    modelTabState: 'view a file to see modelling tools',

    focusModelTab(): void
    {
        let lr = LEFT_RIBBON.getter();
        
        LEFT_PANEL_MANAGER.filesTabState = lr.innerHTML;
        lr.innerHTML = LEFT_PANEL_MANAGER.modelTabState;
        LEFT_RIBBON.getter();

        MODEL_TAB.getter().style.backgroundColor = CommonBGColors.Lightest;
        FILES_TAB.getter().style.backgroundColor = CommonBGColors.Dark;

        DebugRibbon.print("now showing modelling tab.");
    },

    focusFilesTab(): void
    {
        let lr = LEFT_RIBBON.getter();
        LEFT_PANEL_MANAGER.modelTabState = lr.innerHTML;
        lr.innerHTML = LEFT_PANEL_MANAGER.filesTabState;
        LEFT_RIBBON.getter();

        MODEL_TAB.getter().style.backgroundColor = CommonBGColors.Dark;
        FILES_TAB.getter().style.backgroundColor = CommonBGColors.Lightest;

        DebugRibbon.print("now showing the filetree tab.");
    },
};

export function panelManagerInit(): void
{

}