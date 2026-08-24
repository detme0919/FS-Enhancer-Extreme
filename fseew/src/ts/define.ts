export type str = string;
export type f64 = number;
export type bool = boolean;

interface PageEntry {
    (element: HTMLElement): void | Promise<void>;
}

type Icon = {
    regular: str;
    filled:  str;
}

type Page = {
    page: str;
    entry: PageEntry;
    icon: Icon;
    text: str;
}

type Pages = {
    home:     Page,
    list:     Page,
    invoke:   Page,
    settings: Page,
}

export type PagesKey = keyof Pages;

import * as homePage     from './pages/home';
import * as listPage     from './pages/list';
import * as invokePage   from './pages/invoke';
import * as settingsPage from './pages/settings';

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
        page: homeHtml,
        entry: homePage.entry,
        icon: {
            regular: HOME_REGULAR,
            filled:  HOME_FILLED
        },
        text: '首页'
    },
    list: {
        page: listHtml,
        entry: listPage.entry,
        icon: {
            regular: LIST_REGULAR,
            filled:  LIST_FILLED
        },
        text: '列表'
    },
    invoke: {
        page: invokeHtml,
        entry: invokePage.entry,
        icon: {
            regular: INVOKE_REGULAR,
            filled:  INVOKE_FILLED
        },
        text: '调用'
    },
    settings: {
        page: settingsHtml,
        entry: settingsPage.entry,
        icon: {
            regular: SETTINGS_REGULAR,
            filled:  SETTINGS_FILLED
        },
        text: '设置'
    }
}

export const PAGE_LIST: PagesKey[] = Object.keys(PAGE_STRUCT) as PagesKey[];