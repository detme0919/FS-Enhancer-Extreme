import type {
    bool,
    PagesKey
} from '../define';

import {
    PAGE_STRUCT,
    PAGE_LIST
} from '../define';

const body: HTMLBodyElement = document.querySelector('body')!;
const navButtons: HTMLCollection = body.children[1].children;
const swipeTrack: Element = body.children[0].children[0];

const loadedPages: Set<PagesKey> = new Set<PagesKey>();
let currentPage: PagesKey | null = null;

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
            'var(--var-nav-button-color)'
        ;

        navButton.children[0].innerHTML = active
        ?
            PAGE_STRUCT[navButton.id as PagesKey].nav.icon.filled
        :
            PAGE_STRUCT[navButton.id as PagesKey].nav.icon.regular
    }

    (swipeTrack as HTMLElement).style.setProperty(
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

        navButton.children[1].textContent = PAGE_STRUCT[buttonId].nav.text;

        (navButton as HTMLElement).addEventListener('click', _ => switchPage(buttonId))
    }

    switchPage('home');
}