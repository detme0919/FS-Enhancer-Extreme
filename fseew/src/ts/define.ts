export type str = string;
export type f64 = number;
export type bool = boolean;

export interface SubPage {
    put(root: HTMLElement): void
}

import {homePage}     from './pages/home';
import {listPage}     from './pages/list';
import {invokePage}   from './pages/invoke';
import {settingsPage} from './pages/settings';

import HOME_REGULAR     from '@fluentui/svg-icons/icons/home_28_regular.svg?raw';
import HOME_FILLED      from '@fluentui/svg-icons/icons/home_28_filled.svg?raw';
import LIST_REGULAR     from '@fluentui/svg-icons/icons/text_bullet_list_square_28_regular.svg?raw';
import LIST_FILLED      from '@fluentui/svg-icons/icons/text_bullet_list_square_28_filled.svg?raw';
import INVOKE_REGULAR   from '@fluentui/svg-icons/icons/puzzle_piece_28_regular.svg?raw';
import INVOKE_FILLED    from '@fluentui/svg-icons/icons/puzzle_piece_28_filled.svg?raw';
import SETTINGS_REGULAR from '@fluentui/svg-icons/icons/settings_28_regular.svg?raw';
import SETTINGS_FILLED  from '@fluentui/svg-icons/icons/settings_28_filled.svg?raw';

type Icon = {
    regular: str;
    filled:  str;
}
type Page = {
    page: SubPage;
    icon: Icon;
    text: str;
}
type Pages = {
    home:     Page,
    list:     Page,
    invoke:   Page,
    settings: Page,
}
export const PAGE_STRUCT: Pages = {
    home: {
        page: homePage,
        icon: {
            regular: HOME_REGULAR,
            filled:  HOME_FILLED
        },
        text: '首页'
    },
    list: {
        page: listPage,
        icon: {
            regular: LIST_REGULAR,
            filled:  LIST_FILLED
        },
        text: '列表'
    },
    invoke: {
        page: invokePage,
        icon: {
            regular: INVOKE_REGULAR,
            filled:  INVOKE_FILLED
        },
        text: '调用'
    },
    settings: {
        page: settingsPage,
        icon: {
            regular: SETTINGS_REGULAR,
            filled:  SETTINGS_FILLED
        },
        text: '设置'
    }
}

export type PagesKey = keyof Pages;

export const PAGE_LIST: PagesKey[] = Object.keys(PAGE_STRUCT) as PagesKey[];