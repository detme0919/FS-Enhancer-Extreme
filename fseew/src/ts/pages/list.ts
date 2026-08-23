import type {SubPage} from '../define';

import listHtml from '../../html/list.html?raw';

export const listPage: SubPage = {
    put: (element) => element.innerHTML = listHtml
}