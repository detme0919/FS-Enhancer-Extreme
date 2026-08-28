export function entry(element: HTMLElement) {
    const main: Element = element.nth(2);

    const logCard: Element = main.nth(1);
    logCard.nth(1).innerHTML = '日志管理'
    logCard.nth(2).innerHTML = '描述'
    logCard.nth(3).nth(1).innerHTML = '进入'

    const keyboxManagerCard: Element = main.nth(2);
    keyboxManagerCard.nth(1).innerHTML = 'Keybox 管理'
    keyboxManagerCard.nth(2).innerHTML = '描述'
    keyboxManagerCard.nth(3).nth(1).innerHTML = '进入'

    const securityPatchLevelCard: Element = main.nth(3);
    securityPatchLevelCard.nth(1).innerHTML = '安全补丁级别管理'
    securityPatchLevelCard.nth(2).innerHTML = '描述'
    securityPatchLevelCard.nth(3).nth(1).innerHTML = '进入'

    const fsCtlCard: Element = main.nth(4);
    fsCtlCard.nth(1).innerHTML = 'Forge Store 服务控制'
    fsCtlCard.nth(2).innerHTML = '描述'
    fsCtlCard.nth(3).nth(1).innerHTML = '进入'

    const fseeCtlCard: Element = main.nth(5);
    fseeCtlCard.nth(1).innerHTML = 'FS Enhancer Extreme 服务控制'
    fseeCtlCard.nth(2).innerHTML = '描述'
    fseeCtlCard.nth(3).nth(1).innerHTML = '进入'
}