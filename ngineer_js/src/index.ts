import { FILES_TAB, MODEL_TAB } from './common.js'
import { LEFT_PANEL_MANAGER } from './panelManager.js'

FILES_TAB.getter().onclick = LEFT_PANEL_MANAGER.focusFilesTab;
MODEL_TAB.getter().onclick = LEFT_PANEL_MANAGER.focusModelTab;