/*************************************************************
 * 
 *        File: formatting.mjs
 *
 * Description: Contains functions and constants used for
 *              adding dynamic behavior to various parts of
 *              the Ngineer GUI. 
 * 
 *************************************************************/

// ========================= Imports ========================= // 
import { displayOnDebugRibbon } from "./debugRibbon.mjs";

// ========================= Constants ========================= // 

// ========================= Classes ========================= // 

// ========================= Functions ========================= // 
/**
 * A template for creating a valid HTML tag as a string.
 * @param {String} tagType
 * @param {String} tagContent
 * @param {String} tagClass 
 * @param {String} onclickFn 
 */
export function newTag(tagType, tagContent, tagClass, onclickFn)
{
    let classString = "";
    let onclickString = "";

    if (tagType === undefined || tagContent === undefined)
    {
        displayOnDebugRibbon("error in function newTag : tag type or contents were not specified.");
        return "";
    }

    if (tagClass !== undefined)
    {
        classString = ` class="${tagClass}"`;
    }

    if (onclickFn !== undefined)
    {
        onclickString = ` onclick="${onclickFn}"`;
    }

    return `<${tagType}${classString}${onclickString}></${tagType}>`
}
