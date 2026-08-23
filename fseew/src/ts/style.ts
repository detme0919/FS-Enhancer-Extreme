import {
    ProgressBar,
    ProgressBarDefinition,
    Switch,
    SwitchDefinition,
    setTheme
} from '@fluentui/web-components';

import {
    webLightTheme
} from '@fluentui/tokens';

export function entry() {
    setTheme(webLightTheme);

    Switch.define(SwitchDefinition)
    ProgressBar.define(ProgressBarDefinition)
}