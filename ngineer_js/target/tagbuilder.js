var ElementRegistrationErrorKind = {
    IdCollision: 'the desired element id already exists and is registered with the application',
    UnregisteredIdExists: 'the desired element id already exists in the DOM, but is not registered',
};
var ElementRegistrationError = (function () {
    function ElementRegistrationError(variety, id) {
        this.name = 'ElementRegistrationError';
        this.message = "".concat(variety, " (").concat(id, ")");
    }
    return ElementRegistrationError;
}());
var TagGetterError = (function () {
    function TagGetterError(id) {
        this.name = 'TagGetterError';
        this.message = "failed to locate the application-registered id in the DOM (".concat(id, ")");
    }
    return TagGetterError;
}());
var UNIQUE_ELEMENT_IDS = new Set();
var LastGeneratedHtmlIdNumber = 0;
;
var TagBuilder = (function () {
    function TagBuilder(tagType) {
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
    TagBuilder.prototype.setTagClass = function (tagClass) {
        this.tagClass = tagClass;
        this.tagClassText = "class='".concat(tagClass, "'");
        return this;
    };
    TagBuilder.prototype.setTagId = function (id) {
        this.tagId = id;
        this.tagIdText = "id='".concat(id, "'");
        return this;
    };
    TagBuilder.prototype.setTagContent = function (content) {
        this.tagContent = content;
        return this;
    };
    TagBuilder.prototype.setTagCallback = function (callbackKind, callbackFn) {
        this.tagCallbacks[callbackKind] = callbackFn;
        return this;
    };
    TagBuilder.prototype.buildInto = function (parentTag, append) {
        var _this = this;
        var children = [];
        var preview = "<".concat(this.tagType, " ").concat(this.tagIdText).concat(this.tagClassText, ">").concat(this.tagContent, "</").concat(this.tagType, ">");
        var getter = function () {
            var maybeElem = document.getElementById(_this.tagId);
            if (maybeElem === null) {
                dropTagFromApplication(_this.tagId);
                console.warn("dropped ".concat(_this.tagId, " from the application register"));
                throw new TagGetterError(_this.tagId);
            }
            else {
                for (var callback in _this.tagCallbacks) {
                    var cb = callback;
                    maybeElem[cb] = _this.tagCallbacks[cb];
                }
            }
            return maybeElem;
        };
        UNIQUE_ELEMENT_IDS.add(this.tagId);
        if (append) {
            var el = parentTag.getter();
            console.info("refreshed element and children for: ".concat(el.id));
            el.innerHTML += preview;
            parentTag.children.push(getter);
            for (var _i = 0, _a = parentTag.children; _i < _a.length; _i++) {
                var childGetter = _a[_i];
                childGetter();
            }
        }
        else {
            parentTag.getter().innerHTML = preview;
        }
        getter();
        return { getter: getter, children: children };
    };
    return TagBuilder;
}());
export { TagBuilder };
;
export function registerElementId(elementId, parent) {
    var children = [];
    if (UNIQUE_ELEMENT_IDS.has(elementId)) {
        throw new ElementRegistrationError(ElementRegistrationErrorKind.IdCollision, elementId);
    }
    else {
        UNIQUE_ELEMENT_IDS.add(elementId);
    }
    var getter = function () {
        var maybeTag = document.getElementById(elementId);
        if (maybeTag === null) {
            throw new TagGetterError(elementId);
        }
        else {
            return maybeTag;
        }
    };
    return { getter: getter, children: children };
}
export function generateUniqueElementId() {
    var REGISTERED_HTML_TAG = 'registered-html-tag-';
    while (UNIQUE_ELEMENT_IDS.has(REGISTERED_HTML_TAG + LastGeneratedHtmlIdNumber)) {
        LastGeneratedHtmlIdNumber++;
    }
    UNIQUE_ELEMENT_IDS.add(REGISTERED_HTML_TAG + LastGeneratedHtmlIdNumber);
    return REGISTERED_HTML_TAG + LastGeneratedHtmlIdNumber;
}
export function dropTagFromApplication(tagId) {
    var status = false;
    if (UNIQUE_ELEMENT_IDS.has(tagId) &&
        (document.getElementById(tagId) === null)) {
        status = UNIQUE_ELEMENT_IDS.delete(tagId);
    }
    return status;
}
