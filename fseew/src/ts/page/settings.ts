import GLOBAL     from '@fluentui/svg-icons/icons/globe_24_color.svg?raw';
import TRANSLATE  from '@fluentui/svg-icons/icons/text_edit_style_24_color.svg?raw';
import PVH        from '@fluentui/svg-icons/icons/arrow_clockwise_dashes_settings_32_color.svg?raw';
import SSP        from '@fluentui/svg-icons/icons/link_28_color.svg?raw';
import BLACK_LIST from '@fluentui/svg-icons/icons/text_bullet_list_square_sparkle_24_color.svg?raw';
import CHECK      from '@fluentui/svg-icons/icons/search_sparkle_28_color.svg?raw';
import FEEDBACK   from '@fluentui/svg-icons/icons/chat_bubbles_question_24_color.svg?raw';
import ABOUT      from '@fluentui/svg-icons/icons/people_community_48_color.svg?raw'

export function entry(element: HTMLElement) {
    element.nth(1).nth(1).innerHTML = '设置';

    const main: Element = element.nth(2);

    main.nth(1).nth(1).innerHTML = '语言';

    const langSetting: Element = main.nth(2);
    langSetting.nth(1).outerHTML = GLOBAL;
    const langSettingText: Element = langSetting.nth(2);
    langSettingText.nth(1).innerHTML = '语言';
    langSettingText.nth(2).innerHTML = '简体中文';

    const translateLine: Element = main.nth(3);
    translateLine.nth(1).outerHTML = TRANSLATE;
    const translateLineText: Element = translateLine.nth(2);
    translateLineText.nth(1).innerHTML = '参与翻译';
    translateLineText.nth(2).innerHTML = '为 WebUI 的翻译做出贡献';

    main.nth(4).nth(1).innerHTML = '启动流程';

    const passvbhashSetting: Element = main.nth(5);
    passvbhashSetting.nth(1).outerHTML = PVH;
    const passvbhashSettingText = passvbhashSetting.nth(2);
    passvbhashSettingText.nth(1).innerHTML = '修正已验证启动哈希';
    passvbhashSettingText.nth(2).innerHTML = '描述';

    const spsyncpropSetting: Element = main.nth(6);
    spsyncpropSetting.nth(1).outerHTML = SSP;
    const spsyncpropSettingText = spsyncpropSetting.nth(2);
    spsyncpropSettingText.nth(1).innerHTML = '同步安全补丁级别';
    spsyncpropSettingText.nth(2).innerHTML = '描述';

    main.nth(7).nth(1).innerHTML = '作用域';

    const blackListSetting: Element = main.nth(8);
    blackListSetting.nth(1).outerHTML = BLACK_LIST;
    const blackListSettingText = blackListSetting.nth(2);
    blackListSettingText.nth(1).innerHTML = '黑名单';
    blackListSettingText.nth(2).innerHTML = '此选项不会影响应用安装时行为';

    main.nth(9).nth(1).innerHTML = '服务';

    const appCheckSetting: Element = main.nth(10);
    appCheckSetting.nth(1).outerHTML = CHECK;
    const appCheckSettingText = appCheckSetting.nth(2);
    appCheckSettingText.nth(1).innerHTML = '冲突应用处理';
    appCheckSettingText.nth(2).innerHTML = '关闭时跳过 appcheck 调用';

    const modCheckSetting: Element = main.nth(11);
    modCheckSetting.nth(1).outerHTML = CHECK;
    const modCheckSettingText = modCheckSetting.nth(2);
    modCheckSettingText.nth(1).innerHTML = '冲突模块处理';
    modCheckSettingText.nth(2).innerHTML = '关闭时跳过对应线程启动';

    main.nth(12).nth(1).innerHTML = '其他';

    const feedbackLine: Element = main.nth(13);
    feedbackLine.nth(1).outerHTML = FEEDBACK;
    const feedbackLineText = feedbackLine.nth(2);
    feedbackLineText.nth(1).innerHTML = '反馈';
    feedbackLineText.nth(2).innerHTML = '描述';

    const aboutLine: Element = main.nth(14);
    aboutLine.nth(1).outerHTML = ABOUT;
    const aboutLineText = aboutLine.nth(2);
    aboutLineText.nth(1).innerHTML = '关于';
    aboutLineText.nth(2).innerHTML = '描述';

    main.nth(15).nth(1).innerHTML = '结束';
}