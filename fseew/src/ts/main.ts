import * as style from './style';
import * as pages from './page/mod';
// import * as util_functions from './util_functions'
import {exit} from 'kernelsu'

async function main() {
    // const {code} = await util_functions.fseec(['api']);
    // if (code !== 0) {
    //     document.documentElement.remove();
    //     throw new Error('Abnormal Environment');
    //     exit()
    // }

    style.entry();
    pages.entry()
}

main()