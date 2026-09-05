import {
    Output,
    info,
    fseectl
} from '../util_functions';

import STATE_ERROR   from '@fluentui/svg-icons/icons/dismiss_circle_32_color.svg?raw'
import STATE_NORMAL  from '@fluentui/svg-icons/icons/checkmark_circle_32_color.svg?raw'
import STATE_UNKNOWN from '@fluentui/svg-icons/icons/question_circle_32_color.svg?raw'

const unknown: string = '获取失败';

const serviceNotRunning: string = '服务未运行';
const serviceRunning: string = '服务运行中';

const mainModuleTitle: string = '主模块';
const rootImplementTitle: string = '根实现';
const integrityTitle: string = '完整性';
const selinuxStatusTitle: string = 'SELinux 状态';
const androidVersionTitle: string = '安卓版本';
const deviceArchTitle: string = '设备架构';
const kernelVersionTitle: string = '内核版本';
const systemFingerprintTitle: string = '系统指纹';

const multiple: string = '多重共存';
const integrityVerified: string = '通过验证';
const integrityTampered: string = '遭到篡改';
const integrityUnsigned: string = '未签名';
const selinuxEnforcing: string = '强制执行';
const selinuxPermissive: string = '宽容模式';

function extractStdout(promise: Promise<Output>): Promise<string> {
    return promise.then(
        result => (result.code === 0)?(
            result.stdout
        ):(
            unknown
        )
    )
}

const [cs, pv, mm, ri, is, ss, av, ac, kv, sf] = await Promise.all([
    extractStdout(
        fseectl('state')
    ),
    extractStdout(info('pv')),
    extractStdout(info('mm')),
    extractStdout(info('ri')),
    extractStdout(info('is')),
    extractStdout(info('ss')),
    extractStdout(info('av')),
    extractStdout(info('ac')),
    extractStdout(info('kv')),
    extractStdout(info('sf'))
]);

export function entry(element: HTMLElement) {
    const main: Element = element.nth(2);

    const stateCard: Element = main.nth(1);

    const stateCardText: Element = stateCard.nth(2);

    const stateCardE1: Element = stateCardText.nth(1);
    if (cs.includes('not running')) {
        stateCard.nth(1).outerHTML = STATE_ERROR
        stateCardE1.innerHTML = serviceNotRunning;
    } else if (cs.includes('running|')) {
        stateCard.nth(1).outerHTML = STATE_NORMAL
        stateCardE1.innerHTML = serviceRunning;
    } else {
        stateCard.nth(1).outerHTML = STATE_UNKNOWN;
        stateCardE1.innerHTML = cs;
    }
    stateCardText.nth(2).innerHTML = pv;

    const mainCard: Element = main.nth(2);

    const mainCardE1: Element = mainCard.nth(1);
    mainCardE1.nth(1).innerHTML = mainModuleTitle;
    mainCardE1.nth(2).innerHTML = mm.includes('|')?(
        `${multiple}: ${mm}`
    ):(
        mm
    );

    const mainCardE2: Element = mainCard.nth(2);
    mainCardE2.nth(1).innerHTML = rootImplementTitle;
    mainCardE2.nth(2).innerHTML = ri.includes('|')?(
        `${multiple}: ${ri}`
    ):(
        ri
    );

    const mainCardE3: Element = mainCard.nth(3);
    mainCardE3.nth(1).innerHTML = integrityTitle;
    mainCardE3.nth(2).innerHTML = (is === '2')?(
        integrityVerified
    ):(is === '1')?(
        integrityUnsigned
    ):(is === '0')?(
        integrityTampered
    ):(
        is
    );

    const mainCardE4: Element = mainCard.nth(4);
    mainCardE4.nth(1).innerHTML = selinuxStatusTitle;
    mainCardE4.nth(2).innerHTML = (ss === '0')?(
        selinuxPermissive
    ):(ss === '1')?(
        selinuxEnforcing
    ):(
        ss
    );

    const mainCardE5: Element = mainCard.nth(5);
    mainCardE5.nth(1).innerHTML = androidVersionTitle;
    mainCardE5.nth(2).innerHTML = av;

    const mainCardE6: Element = mainCard.nth(6);
    mainCardE6.nth(1).innerHTML = deviceArchTitle;
    mainCardE6.nth(2).innerHTML = ac;

    const mainCardE7: Element = mainCard.nth(7);
    mainCardE7.nth(1).innerHTML = kernelVersionTitle;
    mainCardE7.nth(2).innerHTML = kv;

    const mainCardE8: Element = mainCard.nth(8);
    mainCardE8.nth(1).innerHTML = systemFingerprintTitle;
    mainCardE8.nth(2).innerHTML = sf;
}