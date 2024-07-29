import { DEBUG_RIBBON } from "./common.js";

const DebugRibbonTextColors = 
{
    normal:  '#d0d0d0',
    warning: '#eeff00',
    error:   '#ff0000',
};
type DebugRibbonTextColors = typeof DebugRibbonTextColors[keyof typeof DebugRibbonTextColors]; 

const DebugRibbonTextPrefixes = 
{
    normal:  '',
    warning: 'WARNING: ',
    error:   'ERROR: ',
};
type DebugRibbonTextPrefixes = typeof DebugRibbonTextPrefixes[keyof typeof DebugRibbonTextPrefixes];

/**
 * An object with static methods for displaying information to the user.
 * 
 * Similar to the web dev console, this has options for printing warning 
 * and error messages with special formatting.
 */
export class DebugRibbon
{
    private static printInColorWithPrefix(msg: string, color: DebugRibbonTextColors, prefix: DebugRibbonTextPrefixes): void
    {
        let dr = DEBUG_RIBBON.getter();
        dr.style.color = color;
        dr.innerText = prefix + msg;
    }

    /**
     * Prints a normal message in the debug ribbon
     * @param msg the message to be displayed
     */
    static print(msg: string): void
    {
        this.printInColorWithPrefix(msg, 
            DebugRibbonTextColors.normal, 
            DebugRibbonTextPrefixes.normal);
    }

    /**
     * Prints a warning message in the debug ribbon in yellow with a WARNING prefix
     * @param msg the message to be displayed
     */
    static warn(msg: string): void
    {
        this.printInColorWithPrefix(msg, 
            DebugRibbonTextColors.warning, 
            DebugRibbonTextPrefixes.warning);
    }

    /**
     * Prints an error message in the debug ribbon in red with an ERROR prefix
     * @param msg the message to be displayed
     */
    static error(msg: string): void
    {
        this.printInColorWithPrefix(msg, 
            DebugRibbonTextColors.error, 
            DebugRibbonTextPrefixes.error);
    }
}