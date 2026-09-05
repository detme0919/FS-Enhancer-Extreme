import type {PagesKey} from '../define';

import {
    PAGE_STRUCT,
    PAGE_LIST
} from '../define';

const body: HTMLBodyElement = document.querySelector('body')!;
const navButtons: HTMLCollection = body.nth(2).children;
const swipeTrack: HTMLElement = body.nth(1).nth(1) as HTMLElement;

let currentPage: PagesKey;

function switchPage(pageName: PagesKey) {
    if (pageName === currentPage) return

    currentPage = pageName

    for (const navButton of navButtons) {
        const active: boolean = navButton.id === pageName;

        (navButton as HTMLElement).style.color = (active)?(
            'var(--var-theme-color)'
        ):(
            'var(--var-not-selected-color)'
        );

        navButton.nth(1).outerHTML = (active)?(
            PAGE_STRUCT[navButton.id as PagesKey].nav.icon.filled
        ):(
            PAGE_STRUCT[navButton.id as PagesKey].nav.icon.regular
        )
    }

    swipeTrack.style.setProperty(
        '--var-displacement',
        `-${PAGE_LIST.indexOf(pageName) * 25}%`
    )
}

export function entry() {
    document.documentElement.style.setProperty(
        '--var-page-count',
        PAGE_LIST.length.toString()
    );

    for (const navButton of navButtons) {
        const pageName: PagesKey = navButton.id as PagesKey;

        const pageElement: HTMLElement = document.getElementById(`${pageName}_page`)!;
        pageElement.innerHTML = PAGE_STRUCT[pageName].layout.html;
        PAGE_STRUCT[pageName].layout.entry(pageElement);

        navButton.nth(2).textContent = PAGE_STRUCT[pageName].nav.text;

        (navButton as HTMLElement).addEventListener('click', () => switchPage(pageName))
    }

    switchPage('settings');
}