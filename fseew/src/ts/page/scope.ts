import SEARCH from '../../svg/search_28_color.svg?raw'

export async function entry(element: HTMLElement) {
    const fluentTextInput: Element = element.nth(1).nth(1);
    fluentTextInput.setAttribute(
        'placeholder',
        '搜索应用'
    );
    fluentTextInput.nth(1).innerHTML = SEARCH;
    fluentTextInput.nth(2).innerHTML = '';

    const line: Element = element.nth(2).nth(1);

    const img: Element = line.nth(1);
    img.setAttribute(
        'src',
        "ksu://icon/com.android.settings"
    );

    const text: Element = line.nth(2);
    text.nth(1).innerHTML = '设置';
    text.nth(2).innerHTML = 'com.android.settings';
    text.nth(3).innerHTML = '系统应用';
}