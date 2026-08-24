import type {
    f64,
    bool,
    PagesKey
} from '../define';

import {
    PAGE_STRUCT,
    PAGE_LIST
} from '../define';

const navButtons: NodeListOf<HTMLElement> = document.querySelectorAll<HTMLElement>('body > nav > button');
const swipeTrack: HTMLElement | null = document.querySelector<HTMLElement>('body > main > section');

let loadedPages = new Set<PagesKey>();
let currentPage: PagesKey | null = null;

function switchPage(pageName: PagesKey) {
    if (pageName === currentPage) return

    const pageElement: HTMLElement | null = document.getElementById(`${pageName}_page`);
    if (!pageElement) return

    currentPage = pageName
    pageElement.scrollTop = 0;

    if (!loadedPages.has(pageName)) {
        pageElement.innerHTML = PAGE_STRUCT[pageName].page;
        PAGE_STRUCT[pageName].entry(pageElement);
        loadedPages.add(pageName)
    }

    for (const navButton of navButtons) {
        const active: bool = (navButton.id === pageName);

        navButton.style.color = active
        ?
            'var(--var-theme-color)'
        :
            'var(--var-nav-button-color)'
        ;

        const iconSlot: HTMLElement | null = navButton.querySelector('span');
        if (iconSlot) iconSlot.innerHTML = active
        ?
            PAGE_STRUCT[navButton.id as PagesKey].icon.filled
        :
            PAGE_STRUCT[navButton.id as PagesKey].icon.regular
    }

    const toIndex: f64 = PAGE_LIST.indexOf(pageName);
    if (swipeTrack) swipeTrack.style.setProperty(
        '--var-displacement',
        `-${toIndex * 25}%`
    );
}

export function entry() {
    document.documentElement.style.setProperty(
        '--var-page-count',
        PAGE_LIST.length.toString()
    );

    for (const navButton of navButtons) {
        const labelSlot: HTMLElement | null = navButton.querySelector('div');
        if (labelSlot) labelSlot.textContent = PAGE_STRUCT[navButton.id as PagesKey].text
    }

    switchPage('home');

    document.querySelector('nav')?.addEventListener('click', (event) => {
        const button: HTMLElement | null = (event.target as HTMLElement).closest<HTMLElement>('button');
        if (button) switchPage(button.id as PagesKey);
    });
}