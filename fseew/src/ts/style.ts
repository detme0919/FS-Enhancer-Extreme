import {
    Switch,
    SwitchDefinition,
    TextInput,
    TextInputDefinition,
    Badge,
    BadgeDefinition,
    Button,
    ButtonDefinition,
    Divider,
    DividerDefinition,
    setTheme
} from '@fluentui/web-components';

import {webLightTheme} from '@fluentui/tokens';

export async function entry() {
    setTheme(webLightTheme);
    await Switch.define(SwitchDefinition);
    await TextInput.define(TextInputDefinition);
    await Badge.define(BadgeDefinition);
    await Button.define(ButtonDefinition);
    await Divider.define(DividerDefinition);
}