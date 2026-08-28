import {
    // TextArea,
    // TextAreaDefinition,
    TextInput,
    TextInputDefinition,
    Button,
    ButtonDefinition,
    Switch,
    SwitchDefinition,
    Divider,
    DividerDefinition,
    setTheme
} from '@fluentui/web-components';

import {webLightTheme} from '@fluentui/tokens';

export function entry() {
    setTheme(webLightTheme);
    // TextArea.define(TextAreaDefinition);
    TextInput.define(TextInputDefinition);
    Button.define(ButtonDefinition);
    Switch.define(SwitchDefinition);
    Divider.define(DividerDefinition);
}