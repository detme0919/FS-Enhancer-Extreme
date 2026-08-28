import type {str} from '../define';

import {fseec} from '../util_functions';

import STATE_INIT from '@fluentui/svg-icons/icons/question_circle_32_color.svg?raw'
import STATE_ERROR from '@fluentui/svg-icons/icons/dismiss_circle_32_color.svg?raw'
import STATE_NORMAL from '@fluentui/svg-icons/icons/checkmark_circle_32_color.svg?raw'
// import STATE_WARNING from '@fluentui/svg-icons/icons/error_circle_24_color.svg?raw'

export async function entry(element: HTMLElement) {
    const unknown: str = '未知'

    const main: Element = element.nth(2);

    const stateCard: Element = main.nth(1);

    const stateCardIcon: Element = stateCard.nth(1);
    stateCardIcon.innerHTML = STATE_INIT;

    const stateCardText: Element = stateCard.nth(2);

    const stateCardE1: Element = stateCardText.nth(1);
    stateCardE1.innerHTML = unknown;

    const stateCardE2: Element = stateCardText.nth(2);
    stateCardE2.innerHTML = unknown;

    const {stdout} = await fseec(['fseectl', 'state']);
    if (stdout.includes('not running')) {
        stateCardIcon.innerHTML = STATE_ERROR
        stateCardE1.innerHTML = '服务未运行';
    } else if (stdout.includes('running')) {
        stateCardIcon.innerHTML = STATE_NORMAL
        stateCardE1.innerHTML = '服务运行中';
    }
    stateCardE2.innerHTML = '1.0.0 (110-47970cb-release)';

    const mainCard: Element = main.nth(2);

    const mainCardE1: Element = mainCard.nth(1);
    mainCardE1.nth(1).innerHTML = '主模块';
    mainCardE1.nth(2).innerHTML = unknown;
    mainCardE1.nth(2).innerHTML = 'ForgeStore';

    const mainCardE2: Element = mainCard.nth(2);
    mainCardE2.nth(1).innerHTML = '根实现';
    mainCardE2.nth(2).innerHTML = unknown;
    mainCardE2.nth(2).innerHTML = 'APatch (11224)';

    const mainCardE3: Element = mainCard.nth(3);
    mainCardE3.nth(1).innerHTML = '完整性';
    mainCardE3.nth(2).innerHTML = unknown;
    mainCardE3.nth(2).innerHTML = '通过验证';

    const mainCardE4: Element = mainCard.nth(4);
    mainCardE4.nth(1).innerHTML = 'SELinux 状态';
    mainCardE4.nth(2).innerHTML = unknown;
    mainCardE4.nth(2).innerHTML = '强制执行'; // `${getenforce}`

    const mainCardE5: Element = mainCard.nth(5);
    mainCardE5.nth(1).innerHTML = '安卓版本';
    mainCardE5.nth(2).innerHTML = unknown;
    mainCardE5.nth(2).innerHTML = '14 (API 34)'; // `${getprop ro.build.version.release} (API ${getprop ro.build.version.sdk})`

    const mainCardE6: Element = mainCard.nth(6);
    mainCardE6.nth(1).innerHTML = '设备架构';
    mainCardE6.nth(2).innerHTML = unknown;
    mainCardE6.nth(2).innerHTML = 'arm64-v8a'; // `${getprop ro.product.cpu.abi}`

    const mainCardE7: Element = mainCard.nth(7);
    mainCardE7.nth(1).innerHTML = '内核版本';
    mainCardE7.nth(2).innerHTML = unknown;
    mainCardE7.nth(2).innerHTML = '5.15.123-android13-8-00008-g3ca6a2912c7e-ab11087001'; // `${uname -r}`

    const mainCardE8: Element = mainCard.nth(8);
    mainCardE8.nth(1).innerHTML = '系统指纹';
    mainCardE8.nth(2).innerHTML = unknown;
    mainCardE8.nth(2).innerHTML = 'Xiaomi/fuxi/fuxi:14/UKQ1.230804.001/V816.0.19.0.UMCCNXM:user/release-keys'; // `${getprop ro.build.fingerprint}`
}