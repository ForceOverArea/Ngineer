import { DynTag } from "./tagbuilder";


export interface ProjectTab
{
    filename: string,
    dynTag: DynTag,
}

let currentProjectTabs = new Set<ProjectTab>();