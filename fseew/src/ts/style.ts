import {
    ProgressBar,
    ProgressBarDefinition,
    Switch,
    SwitchDefinition,
    Button,
    ButtonDefinition,
    TextArea,
    TextAreaAppearancesForDisplayShadow,
    setTheme
} from '@fluentui/web-components';

import {
    webLightTheme
} from '@fluentui/tokens';

export function entry() {
    setTheme(webLightTheme);

    Switch.define(SwitchDefinition);
    Button.define(ButtonDefinition);
    ProgressBar.define(ProgressBarDefinition);
}