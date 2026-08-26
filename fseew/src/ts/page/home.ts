import type {
    str
} from '../define';

import {fseec} from '../util_functions';

import STATE_INIT from '@fluentui/svg-icons/icons/question_circle_32_color.svg?raw'
import STATE_ERROR from '@fluentui/svg-icons/icons/dismiss_circle_32_color.svg?raw'
import STATE_NORMAL from '@fluentui/svg-icons/icons/checkmark_circle_32_color.svg?raw'
// import STATE_WARNING from '@fluentui/svg-icons/icons/warning_32_color.svg?raw'

export async function entry(element: HTMLElement) {
    const unknown: str = '未知'

    const main: Element = element.children[1];

    const stateCard: Element = main.children[0];

    const stateCardIcon: Element = stateCard.children[0];
    stateCardIcon.innerHTML = STATE_INIT;

    const stateCardText: Element = stateCard.children[1];

    const stateCardFirst: Element = stateCardText.children[0];
    stateCardFirst.innerHTML = unknown;

    const stateCardLast: Element = stateCardText.children[1];
    stateCardLast.innerHTML = unknown;

    const {stdout} = await fseec(['fseectl', 'state']);
    if (stdout.includes('not running')) {
        stateCardIcon.innerHTML = STATE_ERROR
        stateCardFirst.innerHTML = '服务未运行';
    } else if (stdout.includes('running')) {
        stateCardIcon.innerHTML = STATE_NORMAL
        stateCardFirst.innerHTML = '服务运行中';
    }
    stateCardLast.innerHTML = '//1.0.0 (110-47970cb-release)';

    const mainCard: Element = main.children[1];

    const mainCardE1: Element = mainCard.children[0];
    mainCardE1.children[0].innerHTML = '主模块';
    mainCardE1.children[1].innerHTML = unknown;
    mainCardE1.children[1].innerHTML = '//ForgeStore';

    const mainCardE2: Element = mainCard.children[1];
    mainCardE2.children[0].innerHTML = '根实现';
    mainCardE2.children[1].innerHTML = unknown;
    mainCardE2.children[1].innerHTML = '//APatch (11224)';

    const mainCardE3: Element = mainCard.children[2];
    mainCardE3.children[0].innerHTML = '完整性';
    mainCardE3.children[1].innerHTML = unknown;
    mainCardE3.children[1].innerHTML = '//通过验证';

    const mainCardE4: Element = mainCard.children[3];
    mainCardE4.children[0].innerHTML = 'SELinux 状态';
    mainCardE4.children[1].innerHTML = unknown;
    mainCardE4.children[1].innerHTML = '//强制执行'; // getenforce

    const mainCardE5: Element = mainCard.children[4];
    mainCardE5.children[0].innerHTML = '安卓版本';
    mainCardE5.children[1].innerHTML = unknown;
    mainCardE5.children[1].innerHTML = '//14 (API 34)'; // `${getprop ro.build.version.release} (API ${getprop ro.build.version.sdk})`

    const mainCardE6: Element = mainCard.children[5];
    mainCardE6.children[0].innerHTML = '设备架构';
    mainCardE6.children[1].innerHTML = unknown;
    mainCardE6.children[1].innerHTML = '//arm64-v8a'; //getprop ro.product.cpu.abi

    const mainCardE7: Element = mainCard.children[6];
    mainCardE7.children[0].innerHTML = '设备指纹';
    mainCardE7.children[1].innerHTML = unknown;
    mainCardE7.children[1].innerHTML = '//Xiaomi/fuxi/fuxi:14/UKQ1.230804.001/V816.0.19.0.UMCCNXM:user/release-keys'; //getprop ro.build.fingerprint

    const mainCardE8: Element = mainCard.children[7];
    mainCardE8.children[0].innerHTML = '内核版本';
    mainCardE8.children[1].innerHTML = unknown;
    mainCardE8.children[1].innerHTML = '//5.15.123-android13-8-00008-g3ca6a2912c7e-ab11087001'; //uname -r
}