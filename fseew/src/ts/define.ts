declare global {
    interface Element {
        nth(index: number): Element;
    }
}

Element.prototype.nth = function(index: number): Element {
    return this.children[index - 1]
};

interface PageEntry {
    (element: HTMLElement): void | Promise<void>
}

type Page = {
    layout: {
        html: string,
        entry: PageEntry
    },
    nav: {
        icon: {
            regular: string,
            filled:  string
        },
        text: string
    }
}

type Pages = {
    home:     Page,
    scope:    Page,
    console:  Page,
    settings: Page
}

export type PagesKey = keyof Pages;

import * as homePage     from './page/home';
import * as scopePage    from './page/scope';
import * as consolePage  from './page/console';
import * as settingsPage from './page/settings';

import homeHtml     from '../html/home.html?raw';
import scopeHtml    from '../html/scope.html?raw';
import consoleHtml  from '../html/console.html?raw';
import settingsHtml from '../html/settings.html?raw';

import HOME_REGULAR     from '@fluentui/svg-icons/icons/home_28_regular.svg?raw';
import HOME_FILLED      from '@fluentui/svg-icons/icons/home_28_filled.svg?raw';
import SCOPE_REGULAR    from '@fluentui/svg-icons/icons/text_bullet_list_square_28_regular.svg?raw';
import SCOPE_FILLED     from '@fluentui/svg-icons/icons/text_bullet_list_square_28_filled.svg?raw';
import CONSOLE_REGULAR  from '@fluentui/svg-icons/icons/puzzle_piece_28_regular.svg?raw';
import CONSOLE_FILLED   from '@fluentui/svg-icons/icons/puzzle_piece_28_filled.svg?raw';
import SETTINGS_REGULAR from '@fluentui/svg-icons/icons/settings_28_regular.svg?raw';
import SETTINGS_FILLED  from '@fluentui/svg-icons/icons/settings_28_filled.svg?raw';

export const PAGE_STRUCT: Pages = {
    home: {
        layout: {
            html:  homeHtml,
            entry: homePage.entry
        },
        nav: {
            icon: {
                regular: HOME_REGULAR,
                filled:  HOME_FILLED
            },
            text: '首页'
        }
    },
    scope: {
        layout: {
            html:  scopeHtml,
            entry: scopePage.entry
        },
        nav: {
            icon: {
                regular: SCOPE_REGULAR,
                filled:  SCOPE_FILLED
            },
            text: '作用域'
        }
    },
    console: {
        layout: {
            html:  consoleHtml,
            entry: consolePage.entry
        },
        nav: {
            icon: {
                regular: CONSOLE_REGULAR,
                filled:  CONSOLE_FILLED
            },
            text: '控制台'
        }
    },
    settings: {
        layout: {
            html:  settingsHtml,
            entry: settingsPage.entry
        },
        nav: {
            icon: {
                regular: SETTINGS_REGULAR,
                filled:  SETTINGS_FILLED
            },
            text: '设置'
        }
    }
}

export const PAGE_LIST: PagesKey[] = Object.keys(PAGE_STRUCT) as PagesKey[];

export const FSEEMODDIR: string = '/data/adb/modules/fs_enhancer_extreme';
export const LINK_PREFIX: string = 'https://github.com/XtrLumen/FS-Enhancer-Extreme';