/**
 * A function signature used to represent tags in 
 * my fucked up framework. This allows users to
 * get HTML elements lazily to ensure that they get
 * the latest instance of that tag if it has been 
 * re-instantiated in the DOM.
 */
export type TagGetter = () => HTMLElement;

/**
 * The different error messages that should be shown by an `ElementRagistrationError`
 */
const ElementRegistrationErrorKind =
{
    IdCollision: 'the desired element id already exists and is registered with the application',
    UnregisteredIdExists: 'the desired element id already exists in the DOM, but is not registered',
};
type ElementRegistrationErrorKind = typeof ElementRegistrationErrorKind[keyof typeof ElementRegistrationErrorKind];

/**
 * An error type returned when registering a tag with the application
 */
class ElementRegistrationError implements Error
{
    name: string;
    message: string;

    constructor(variety: ElementRegistrationErrorKind, id: string)
    {
        this.name = 'ElementRegistrationError';
        this.message = `${variety} (${id})`;
    }
}

/**
 * An error type returned when a `TagGetter` function 
 * fails to find the element in the DOM.
 */
class TagGetterError implements Error
{
    name: string;
    message: string;

    constructor(id: string)
    {
        this.name = 'TagGetterError';
        this.message = `failed to locate the application-registered id in the DOM (${id})`;
    }
}

/**
 * A 'singleton' object containing all the unique HTML tag
 * ids that the application is aware of.
 */
const UNIQUE_ELEMENT_IDS: Set<string> = new Set();

/**
 * Saves the id number used by the last generated unique 
 * id to save time while trying to generate a new id.
 */
let LastGeneratedHtmlIdNumber = 0;

/**
 * Represents a standard variety of Html Tag
 */
export type HtmlTagKind =  'a' | 'b' | 'div' | 'h1' | 'h2' | 'h3' | 'i' | 'span' | 'strong';

/**
 * 
 */
export interface TagCallbacks
{
    onclick:        ((e: Event) => void) | null;
    onmouseover:    ((e: Event) => void) | null;
    onmouseenter:   ((e: Event) => void) | null;
    onmouseleave:   ((e: Event) => void) | null;
    onmouseup:      ((e: Event) => void) | null;
    onmousedown:    ((e: Event) => void) | null;
};

type TagCallbackKind = keyof TagCallbacks;

export interface DynTag
{
    getter: TagGetter,
    children: TagGetter[],
}

/**
 * 
 */
export class TagBuilder
{
    tagType:        HtmlTagKind;
    tagClass:       string;
    tagClassText:   string;
    tagId:          string;
    tagIdText:      string;
    tagContent:     string;
    tagCallbacks:   TagCallbacks;

    /**
     * Creates a new HTML tag builder object.
     * @param tagType The variety of HTML tag to create
     */
    constructor(tagType: HtmlTagKind)
    {
        this.tagType = tagType;
        this.tagClass = '';
        this.tagClassText = '';
        this.tagId = '';
        this.tagIdText = '';
        this.tagContent = '';
        this.tagCallbacks = {
            onclick: null,
            onmouseover: null,
            onmouseenter: null,
            onmouseleave: null,
            onmousedown: null,
            onmouseup: null,
        };
    }

    setTagClass(tagClass: string): TagBuilder
    {
        this.tagClass = tagClass;
        this.tagClassText = `class='${tagClass}'`;
        return this;
    }

    setTagId(id: string)
    {
        this.tagId = id;
        this.tagIdText = `id='${id}'`;
        return this;
    }

    setTagContent(content: string)
    {
        this.tagContent = content;
        return this;
    }

    setTagCallback(callbackKind: TagCallbackKind, callbackFn: (e: Event) => void): TagBuilder
    {
        this.tagCallbacks[callbackKind] = callbackFn;
        return this;
    }

    /**
     * Builds the HTML tag into the DOM in the parent tag specified, registering it with the
     * application in the process.
     * @param parentTag the TagGetter function of the element to inject this element to
     * @param append a boolean flag that indicates whether the content should be added to the end 
     *               of the content already in parentTag or overwrite it.
     * @returns a new TagGetter function for the injected HTML element.
     */
    buildInto(parentTag: DynTag, append?: boolean): DynTag
    {
        const children: TagGetter[] = [];
        let preview = `<${this.tagType} ${this.tagIdText}${this.tagClassText}>${this.tagContent}</${this.tagType}>`;
        let getter = () => {
            let maybeElem = document.getElementById(this.tagId);
            if (maybeElem === null)
            {
                dropTagFromApplication(this.tagId);
                console.warn(`dropped ${this.tagId} from the application register`);
                throw new TagGetterError(this.tagId);
            }
            else
            {
                for (let callback in this.tagCallbacks)
                {
                    let cb = callback as keyof TagCallbacks;
                    maybeElem[cb] = this.tagCallbacks[cb];
                }
            }
            return maybeElem;
        };

        UNIQUE_ELEMENT_IDS.add(this.tagId);

        if (append)
        {
            const el = parentTag.getter();
            console.info(`refreshed element and children for: ${el.id}`);
            el.innerHTML += preview;
            parentTag.children.push(getter);
            for (let childGetter of parentTag.children)
            { 
                childGetter(); // Re-initialize old tags
            }
        }
        else
        {
            parentTag.getter().innerHTML = preview;
        }

        // Run once to initialize element properties
        getter();

        return { getter, children };
    }
};

/**
 * Registers a given HTML tag id with the application, allowing it to  
 * @param elementId 
 */
export function registerElementId(elementId: string, parent?: DynTag): DynTag
{
    const children: TagGetter[] = [];

    // Guard clause against duplicate ids
    if (UNIQUE_ELEMENT_IDS.has(elementId))
    {
        throw new ElementRegistrationError(
            ElementRegistrationErrorKind.IdCollision, 
            elementId,
        );
    }
    else
    {
        // Add element to set for quicker lookup
        UNIQUE_ELEMENT_IDS.add(elementId);
    }

    const getter = () => {
        let maybeTag = document.getElementById(elementId);
        if (maybeTag === null)
        {
            throw new TagGetterError(elementId);
        }
        else
        {
            return maybeTag;
        }
    };

    return { getter, children };
}

/**
 * 
 * @returns a unique id for a new HTML element.
 */
export function generateUniqueElementId(): string
{
    const REGISTERED_HTML_TAG = 'registered-html-tag-'
    while (UNIQUE_ELEMENT_IDS.has(REGISTERED_HTML_TAG + LastGeneratedHtmlIdNumber))
    {
        LastGeneratedHtmlIdNumber++;
    }
    UNIQUE_ELEMENT_IDS.add(REGISTERED_HTML_TAG + LastGeneratedHtmlIdNumber);
    return REGISTERED_HTML_TAG + LastGeneratedHtmlIdNumber;
}

export function dropTagFromApplication(tagId: string): boolean
{
    let status: boolean = false;

    if (UNIQUE_ELEMENT_IDS.has(tagId) && 
        (document.getElementById(tagId) === null))
    {
        status = UNIQUE_ELEMENT_IDS.delete(tagId);
    }

    return status;
}