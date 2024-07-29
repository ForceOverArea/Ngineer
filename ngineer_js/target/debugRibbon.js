import { DEBUG_RIBBON } from "./common.js";
var DebugRibbonTextColors = {
    normal: '#d0d0d0',
    warning: '#eeff00',
    error: '#ff0000',
};
var DebugRibbonTextPrefixes = {
    normal: '',
    warning: 'WARNING: ',
    error: 'ERROR: ',
};
var DebugRibbon = (function () {
    function DebugRibbon() {
    }
    DebugRibbon.printInColorWithPrefix = function (msg, color, prefix) {
        var dr = DEBUG_RIBBON.getter();
        dr.style.color = color;
        dr.innerText = prefix + msg;
    };
    DebugRibbon.print = function (msg) {
        this.printInColorWithPrefix(msg, DebugRibbonTextColors.normal, DebugRibbonTextPrefixes.normal);
    };
    DebugRibbon.warn = function (msg) {
        this.printInColorWithPrefix(msg, DebugRibbonTextColors.warning, DebugRibbonTextPrefixes.warning);
    };
    DebugRibbon.error = function (msg) {
        this.printInColorWithPrefix(msg, DebugRibbonTextColors.error, DebugRibbonTextPrefixes.error);
    };
    return DebugRibbon;
}());
export { DebugRibbon };
