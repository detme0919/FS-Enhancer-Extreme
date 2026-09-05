import {
    Output,
    launchBrowser,
    setting_on,
    setting_off,
    setting_get
} from '../util_functions';

import {LINK_PREFIX} from "../define";

import GLOBAL     from '@fluentui/svg-icons/icons/globe_24_color.svg?raw';
import TRANSLATE  from '@fluentui/svg-icons/icons/text_edit_style_24_color.svg?raw';
import PVH        from '@fluentui/svg-icons/icons/arrow_clockwise_dashes_settings_32_color.svg?raw';
import SSP        from '@fluentui/svg-icons/icons/link_28_color.svg?raw';
import BLACK_LIST from '@fluentui/svg-icons/icons/text_bullet_list_square_sparkle_24_color.svg?raw';
import CHECK      from '@fluentui/svg-icons/icons/search_sparkle_28_color.svg?raw';
import FEEDBACK   from '@fluentui/svg-icons/icons/chat_bubbles_question_24_color.svg?raw';
import ABOUT      from '@fluentui/svg-icons/icons/people_community_48_color.svg?raw'

const setting: string = '设置';

const i18n: string = '国际化';
const language: string = '语言';
const thisLanguage: string = '简体中文';
const joinTranslate: string = '参与翻译';
const contributeTranslate: string = '为 WebUI 的翻译做出贡献';

const bootProcess: string = '启动流程';
const skipSpSync: string = '修正已验证启动哈希';
const skipSpSyncDesc: string = '异常状态时重设 prop 为正确值';
const skipVbhPass: string = '同步安全补丁级别';
const skipVbhPassDesc: string = '重设 prop 为配置';

const daemon: string = '守护进程';
const skipAppCheck: string = '冲突应用处理';
const skipAppCheckDesc: string = '关闭时跳过推入 appcheck 参数';
const skipModCheck: string = '冲突模块处理';
const skipModCheckDesc: string = '关闭时跳过启动 modcheck 线程';

const scope: string = '作用域';
const blackList: string = '黑名单';
const blackListDesc: string = '此选项不会影响应用安装时行为';

const other: string = '其他';
const feedback: string = '反馈';
const feedbackDesc: string = '前往 Github 创建 Issues';
const about: string = '关于';
const aboutDesc: string = '';

const end: string = '结束';

function parseCode(promise: Promise<Output>): Promise<boolean> {
    return promise.then(
        result => (result.code === 1)
    )
}

const [blacklist, vbhpass, spsync, appcheck, modcheck] = await Promise.all([
    parseCode(setting_get('blacklist')),
    parseCode(setting_get('vbhpass')),
    parseCode(setting_get('spsync')),
    parseCode(setting_get('appcheck')),
    parseCode(setting_get('modcheck')),
]);

export function entry(element: HTMLElement) {
    const tempCTCN: string = `${LINK_PREFIX}#翻译`;
    const tempCTOther: string = `${LINK_PREFIX}#Translation`;

    element.nth(1).nth(1).innerHTML = setting;

    const main: Element = element.nth(2);

    main.nth(1).nth(1).innerHTML = i18n;

    const langSetting: Element = main.nth(2);
    langSetting.nth(1).outerHTML = GLOBAL;
    const langSettingText: Element = langSetting.nth(2);
    langSettingText.nth(1).innerHTML = language;
    langSettingText.nth(2).innerHTML = thisLanguage;

    const translateLine: Element = main.nth(3);
    translateLine.nth(1).outerHTML = TRANSLATE;
    const translateLineText: Element = translateLine.nth(2);
    translateLineText.nth(1).innerHTML = joinTranslate;
    translateLineText.nth(2).innerHTML = contributeTranslate;

    translateLine.addEventListener('click', () => launchBrowser(tempCTCN));

    main.nth(4).nth(1).innerHTML = bootProcess;

    const passvbhashSetting: Element = main.nth(5);
    passvbhashSetting.nth(1).outerHTML = PVH;
    const passvbhashSettingText = passvbhashSetting.nth(2);
    passvbhashSettingText.nth(1).innerHTML = skipSpSync;
    passvbhashSettingText.nth(2).innerHTML = skipSpSyncDesc;
    const passvbhashSettingSwitch: Element = passvbhashSetting.nth(3);
    if (vbhpass) passvbhashSettingSwitch.setAttribute('checked', '');
    passvbhashSettingSwitch.addEventListener('change', () => passvbhashSettingSwitch.hasAttribute('state--checked')?(
        setting_on('vbhpass')
    ):(
        setting_off('vbhpass')
    ));

    const spsyncpropSetting: Element = main.nth(6);
    spsyncpropSetting.nth(1).outerHTML = SSP;
    const spsyncpropSettingText = spsyncpropSetting.nth(2);
    spsyncpropSettingText.nth(1).innerHTML = skipVbhPass;
    spsyncpropSettingText.nth(2).innerHTML = skipVbhPassDesc;
    const spsyncpropSettingSwitch: Element = spsyncpropSetting.nth(3);
    if (spsync) spsyncpropSettingSwitch.setAttribute('checked', '');
    spsyncpropSettingSwitch.addEventListener('change', () => spsyncpropSettingSwitch.hasAttribute('state--checked') ? (
        setting_on('spsync')
    ) : (
        setting_off('spsync')
    ));

    main.nth(7).nth(1).innerHTML = daemon;

    const appCheckSetting: Element = main.nth(8);
    appCheckSetting.nth(1).outerHTML = CHECK;
    const appCheckSettingText = appCheckSetting.nth(2);
    appCheckSettingText.nth(1).innerHTML = skipAppCheck;
    appCheckSettingText.nth(2).innerHTML = skipAppCheckDesc;
    const appCheckSettingSwitch: Element = appCheckSetting.nth(3);
    if (appcheck) appCheckSettingSwitch.setAttribute('checked', '');
    appCheckSettingSwitch.addEventListener('change', () => appCheckSettingSwitch.hasAttribute('state--checked') ? (
        setting_on('appcheck')
    ) : (
        setting_off('appcheck')
    ));

    const modCheckSetting: Element = main.nth(9);
    modCheckSetting.nth(1).outerHTML = CHECK;
    const modCheckSettingText = modCheckSetting.nth(2);
    modCheckSettingText.nth(1).innerHTML = skipModCheck;
    modCheckSettingText.nth(2).innerHTML = skipModCheckDesc;
    const modCheckSettingSwitch: Element = modCheckSetting.nth(3);
    if (modcheck) modCheckSettingSwitch.setAttribute('checked', '');
    modCheckSettingSwitch.addEventListener('change', () => modCheckSettingSwitch.hasAttribute('state--checked') ? (
        setting_on('modcheck')
    ) : (
        setting_off('modcheck')
    ));

    main.nth(10).nth(1).innerHTML = scope;

    const blackListSetting: Element = main.nth(11);
    blackListSetting.nth(1).outerHTML = BLACK_LIST;
    const blackListSettingText = blackListSetting.nth(2);
    blackListSettingText.nth(1).innerHTML = blackList;
    blackListSettingText.nth(2).innerHTML = blackListDesc;
    const blackListSettingSwitch: Element = blackListSetting.nth(3);
    if (blacklist) blackListSettingSwitch.setAttribute('checked', '');
    blackListSettingSwitch.addEventListener('change', () => blackListSettingSwitch.hasAttribute('state--checked') ? (
        setting_on('blacklist')
    ) : (
        setting_off('blacklist')
    ));

    main.nth(12).nth(1).innerHTML = other;

    const feedbackLine: Element = main.nth(13);
    feedbackLine.nth(1).outerHTML = FEEDBACK;
    const feedbackLineText = feedbackLine.nth(2);
    feedbackLineText.nth(1).innerHTML = feedback;
    feedbackLineText.nth(2).innerHTML = feedbackDesc;
    feedbackLine.addEventListener('click', () => launchBrowser(`${LINK_PREFIX}/issues/new`));

    const aboutLine: Element = main.nth(14);
    aboutLine.nth(1).outerHTML = ABOUT;
    const aboutLineText = aboutLine.nth(2);
    aboutLineText.nth(1).innerHTML = about;
    aboutLineText.nth(2).innerHTML = aboutDesc;

    main.nth(15).nth(1).innerHTML = end
}