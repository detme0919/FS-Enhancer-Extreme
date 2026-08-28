import type {
    bool,
    PagesKey
} from '../define';

import {
    PAGE_STRUCT,
    PAGE_LIST
} from '../define';

const body: HTMLBodyElement = document.querySelector('body')!;
const navButtons: HTMLCollection = body.nth(2).children;
const swipeTrack: HTMLElement = body.nth(1).nth(1) as HTMLElement;

const loadedPages: Set<PagesKey> = new Set<PagesKey>();
let currentPage: PagesKey;

function switchPage(pageName: PagesKey) {
    if (pageName === currentPage) return

    const pageElement: HTMLElement = document.getElementById(`${pageName}_page`)!;

    currentPage = pageName
    pageElement.scrollTop = 0;

    if (!loadedPages.has(pageName)) {
        pageElement.innerHTML = PAGE_STRUCT[pageName].layout.html;
        PAGE_STRUCT[pageName].layout.entry(pageElement);
        loadedPages.add(pageName)
    }

    for (const navButton of navButtons) {
        const active: bool = (navButton.id === pageName);

        (navButton as HTMLElement).style.color = active
        ?
            'var(--var-theme-color)'
        :
            'var(--var-not-selected-color)'
        ;

        navButton.nth(1).innerHTML = active
        ?
            PAGE_STRUCT[navButton.id as PagesKey].nav.icon.filled
        :
            PAGE_STRUCT[navButton.id as PagesKey].nav.icon.regular
    }

    (swipeTrack).style.setProperty(
        '--var-displacement',
        `-${PAGE_LIST.indexOf(pageName) * 25}%`
    );
}

export function entry() {
    document.documentElement.style.setProperty(
        '--var-page-count',
        PAGE_LIST.length.toString()
    );

    for (const navButton of navButtons) {
        const buttonId: PagesKey = navButton.id as PagesKey;

        navButton.nth(2).textContent = PAGE_STRUCT[buttonId].nav.text;

        (navButton as HTMLElement).addEventListener('click', _ => switchPage(buttonId))
    }

    switchPage('scope');
}