import {
    ProgressBar,
    ProgressBarDefinition,
    Switch,
    SwitchDefinition,
    Button,
    ButtonDefinition,
    setTheme
} from '@fluentui/web-components';

import {
    webLightTheme
} from '@fluentui/tokens';

export function entry() {
    setTheme(webLightTheme);

    Switch.define(SwitchDefinition);
    Button.define(ButtonDefinition);
    ProgressBar.define(ProgressBarDefinition)
}