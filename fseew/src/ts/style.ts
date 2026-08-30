import {
    Switch,
    SwitchDefinition,
    TextInput,
    TextInputDefinition,
    Badge,
    BadgeDefinition,
    Button,
    ButtonDefinition,
    // TextArea,
    // TextAreaDefinition,
    Divider,
    DividerDefinition,
    setTheme
} from '@fluentui/web-components';

import {webLightTheme} from '@fluentui/tokens';

export function entry() {
    setTheme(webLightTheme);
    Switch.define(SwitchDefinition);
    TextInput.define(TextInputDefinition);
    Badge.define(BadgeDefinition);
    Button.define(ButtonDefinition);
    // TextArea.define(TextAreaDefinition);
    Divider.define(DividerDefinition);
}