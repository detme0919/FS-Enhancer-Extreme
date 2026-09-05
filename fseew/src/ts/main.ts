import * as style from './style';
import * as pages from './page/mod';
import * as util_functions from './util_functions'

import {exit} from 'kernelsu'

async function main() {
    const {code} = await util_functions.fseec(['envcheck']);
    if (code !== 0) {
        document.documentElement.remove();
        exit()
    }

    await style.entry();
    pages.entry()
}

await main()