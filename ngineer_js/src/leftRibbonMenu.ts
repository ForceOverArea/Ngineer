import { dropTagFromApplication, DynTag, generateUniqueElementId, TagBuilder, TagGetter } from './tagbuilder.js'
import { CommonTagClasses, LEFT_RIBBON, PROJ_TABS_BAR, TOP_BAR } from './common.js'

export function createLeftRibbonMenuFileItem(filename: string): DynTag
{
    const uniqueId = generateUniqueElementId();
    const onclickHandler = generateFocusCallback(filename, uniqueId);
    return new TagBuilder('div')
        .setTagId(uniqueId)
        .setTagClass(CommonTagClasses.LeftRibbonMenuFileItem)
        .setTagContent(filename)
        .setTagCallback('onclick', onclickHandler)
        .buildInto(LEFT_RIBBON, true);
}

// TODO: make a function that returns a closure for focusing a specific instance of the top ribbon menu files
export function generateFocusCallback(filename: string, elementId?: string): (e: Event) => void 
{
    const onclickFocusHandler = () => console.log(`clicked the ${filename} tab!`);

    const newChildIndex = PROJ_TABS_BAR.children.length;

    const onclickCloseHandler = () => {
        const ptb = PROJ_TABS_BAR.getter();
        const tagToRemove = ptb.children[newChildIndex];
        console.log('chugya');
        ptb.removeChild(tagToRemove);
        dropTagFromApplication(tagToRemove.id);
    }

    return (event: Event) => {
        const tabElement = document.getElementById(filename + '-tab');

        if (tabElement === null)
        {
            // If the tab doesn't exist, make one:
            const tab = new TagBuilder('span')
                .setTagId(filename + '-tab')
                .setTagClass(CommonTagClasses.TopRibbonMenuFileItem)
                .setTagCallback('onmouseup', onclickFocusHandler)
                .buildInto(PROJ_TABS_BAR, true);
            const _filenameTag = new TagBuilder('span')
                .setTagId(filename+ '-filename-tag')
                .setTagClass(CommonTagClasses.FilenameTag)
                .setTagContent(filename)
                .buildInto(tab, false);
            const _closeButton = new TagBuilder('span')
                .setTagId(filename + '-close-button')
                .setTagClass(CommonTagClasses.CloseButton)
                .setTagCallback('onmousedown', onclickCloseHandler)
                .setTagContent('X')
                .buildInto(tab, true);

            // Then, make the window for the file:
            

            // Then focus on that window:
        }
        else
        {
            // Pretty sure I need to write a callback to deregister the element if it 
            // fails here? otherwise this branch might not be necessary.
        }
    };
}