export type str = string;
export type f64 = number;
export type bool = boolean;

interface PageEntry {
    (element: HTMLElement): void | Promise<void>
}

type Page = {
    layout: {
        html: str,
        entry: PageEntry
    },
    nav: {
        icon: {
            regular: str,
            filled:  str
        },
        text: str
    }
}

type Pages = {
    home:     Page,
    list:     Page,
    invoke:   Page,
    settings: Page
}

export type PagesKey = keyof Pages;

import * as homePage     from './page/home';
import * as listPage     from './page/list';
import * as invokePage   from './page/invoke';
import * as settingsPage from './page/settings';

import homeHtml     from '../html/home.html?raw';
import listHtml     from '../html/list.html?raw';
import invokeHtml   from '../html/invoke.html?raw';
import settingsHtml from '../html/settings.html?raw';

import HOME_REGULAR     from '@fluentui/svg-icons/icons/home_28_regular.svg?raw';
import HOME_FILLED      from '@fluentui/svg-icons/icons/home_28_filled.svg?raw';
import LIST_REGULAR     from '@fluentui/svg-icons/icons/text_bullet_list_square_28_regular.svg?raw';
import LIST_FILLED      from '@fluentui/svg-icons/icons/text_bullet_list_square_28_filled.svg?raw';
import INVOKE_REGULAR   from '@fluentui/svg-icons/icons/puzzle_piece_28_regular.svg?raw';
import INVOKE_FILLED    from '@fluentui/svg-icons/icons/puzzle_piece_28_filled.svg?raw';
import SETTINGS_REGULAR from '@fluentui/svg-icons/icons/settings_28_regular.svg?raw';
import SETTINGS_FILLED  from '@fluentui/svg-icons/icons/settings_28_filled.svg?raw';

export const PAGE_STRUCT: Pages = {
    home: {
        layout: {
            html: homeHtml,
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
    list: {
        layout: {
            html: listHtml,
            entry: listPage.entry
        },
        nav: {
            icon: {
                regular: LIST_REGULAR,
                filled:  LIST_FILLED
            },
            text: '列表'
        }
    },
    invoke: {
        layout: {
            html: invokeHtml,
            entry: invokePage.entry
        },
        nav: {
            icon: {
                regular: INVOKE_REGULAR,
                filled:  INVOKE_FILLED
            },
            text: '调用'
        }
    },
    settings: {
        layout: {
            html: settingsHtml,
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

export const FSEEMODDIR: str = "/data/adb/modules/fs_enhancer_extreme";