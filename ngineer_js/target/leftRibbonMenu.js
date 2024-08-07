import { dropTagFromApplication, generateUniqueElementId, TagBuilder } from './tagbuilder.js';
import { CommonTagClasses, LEFT_RIBBON, PROJ_TABS_BAR } from './common.js';
export function createLeftRibbonMenuFileItem(filename) {
    var uniqueId = generateUniqueElementId();
    var onclickHandler = generateFocusCallback(filename, uniqueId);
    return new TagBuilder('div')
        .setTagId(uniqueId)
        .setTagClass(CommonTagClasses.LeftRibbonMenuFileItem)
        .setTagContent(filename)
        .setTagCallback('onclick', onclickHandler)
        .buildInto(LEFT_RIBBON, true);
}
export function generateFocusCallback(filename, elementId) {
    var onclickFocusHandler = function () { return console.log("clicked the ".concat(filename, " tab!")); };
    var newChildIndex = PROJ_TABS_BAR.children.length;
    var onclickCloseHandler = function () {
        var ptb = PROJ_TABS_BAR.getter();
        var tagToRemove = ptb.children[newChildIndex];
        console.log('chugya');
        ptb.removeChild(tagToRemove);
        dropTagFromApplication(tagToRemove.id);
    };
    return function (event) {
        var tabElement = document.getElementById(filename + '-tab');
        if (tabElement === null) {
            var tab = new TagBuilder('span')
                .setTagId(filename + '-tab')
                .setTagClass(CommonTagClasses.TopRibbonMenuFileItem)
                .setTagCallback('onmouseup', onclickFocusHandler)
                .buildInto(PROJ_TABS_BAR, true);
            var _filenameTag = new TagBuilder('span')
                .setTagId(filename + '-filename-tag')
                .setTagClass(CommonTagClasses.FilenameTag)
                .setTagContent(filename)
                .buildInto(tab, false);
            var _closeButton = new TagBuilder('span')
                .setTagId(filename + '-close-button')
                .setTagClass(CommonTagClasses.CloseButton)
                .setTagCallback('onmousedown', onclickCloseHandler)
                .setTagContent('X')
                .buildInto(tab, true);
        }
        else {
        }
    };
}
