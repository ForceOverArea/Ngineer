import { CommonBGColors, FILES_TAB, LEFT_RIBBON, MODEL_TAB } from "./common.js";
import { DebugRibbon } from "./debugRibbon.js";
export var SCREEN_PANEL_MANAGER = [];
export var LEFT_PANEL_MANAGER = {
    filesTabState: '',
    modelTabState: 'view a file to see modelling tools',
    focusModelTab: function () {
        var lr = LEFT_RIBBON.getter();
        LEFT_PANEL_MANAGER.filesTabState = lr.innerHTML;
        lr.innerHTML = LEFT_PANEL_MANAGER.modelTabState;
        LEFT_RIBBON.getter();
        MODEL_TAB.getter().style.backgroundColor = CommonBGColors.Lightest;
        FILES_TAB.getter().style.backgroundColor = CommonBGColors.Dark;
        DebugRibbon.print("now showing modelling tab.");
    },
    focusFilesTab: function () {
        var lr = LEFT_RIBBON.getter();
        LEFT_PANEL_MANAGER.modelTabState = lr.innerHTML;
        lr.innerHTML = LEFT_PANEL_MANAGER.filesTabState;
        LEFT_RIBBON.getter();
        MODEL_TAB.getter().style.backgroundColor = CommonBGColors.Dark;
        FILES_TAB.getter().style.backgroundColor = CommonBGColors.Lightest;
        DebugRibbon.print("now showing the filetree tab.");
    },
};
export function panelManagerInit() {
}
