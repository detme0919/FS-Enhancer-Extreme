import type {SubPage} from '../define';

import homeHtml from '../../html/home.html?raw';

export const homePage: SubPage = {
    put: (element) => element.innerHTML = homeHtml
}