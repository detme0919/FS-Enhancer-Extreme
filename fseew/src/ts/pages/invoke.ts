import type {SubPage} from '../define';

import invokeHtml from '../../html/invoke.html?raw';

export const invokePage: SubPage = {
    put: (element) => element.innerHTML = invokeHtml
}