import { registerElementId } from './tagbuilder.js'

export const CommonBGColors = 
{
    Lightest:   '#2e2e2e',
    Light:      '#161616',
    Dark:       '#1e1e1e',
    Darkest:    '#161616',
};
type CommonBGColors = typeof CommonBGColors[keyof typeof CommonBGColors]

export const CommonTagClasses =
{
    LeftRibbonMenuFileItem: 'left-ribbon-menu-file-item debug',
    TopRibbonMenuFileItem: 'top-ribbon-menu-file-item debug',
    FilenameTag: 'filename-tag debug',
    CloseButton: 'close-button debug',
};
type CommonTagClasses = typeof CommonTagClasses[keyof typeof CommonTagClasses];

// Register ids written in HTML file
export const TOP_BAR           = registerElementId('top-bar');
export const LEFT_RIBBON_TABS  = registerElementId('left-ribbon-tabs');
export const FILES_TAB         = registerElementId('files-tab');
export const MODEL_TAB         = registerElementId('model-tab');
export const PROJ_TABS_BAR     = registerElementId('proj-tabs-bar');
export const MID_CONTAINER     = registerElementId('mid-container');
export const LEFT_RIBBON       = registerElementId('left-ribbon');
export const PROJ_PANE         = registerElementId('proj-pane');
export const DEBUG_RIBBON      = registerElementId('debug-ribbon');
export const OPEN_PROJECT      = registerElementId('dropdown-file-open-project');
export const SAVE_PROJECT      = registerElementId('dropdown-file-save-project');