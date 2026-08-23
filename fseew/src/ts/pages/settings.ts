import type {SubPage} from '../define';

import settingsHtml from '../../html/settings.html?raw';

export const settingsPage: SubPage = {
    put: (element) => element.innerHTML = settingsHtml
}