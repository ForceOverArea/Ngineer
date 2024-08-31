import { DynTag } from "./tagbuilder";

/**
 * The interface used to store data and behavior pertinent to 
 * managing different screens in the main panel of the application.
 */
export interface ProjectTab
{
    filename: string,
    dynTag: DynTag,
}

/**
 * The set of unique windows open in the main window of the application.
 */
let currentProjectTabs = new Set<ProjectTab>();

