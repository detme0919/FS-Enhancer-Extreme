import {
    FSEEMODDIR
} from './define'

import {exec} from 'kernelsu';

export interface Output {
    code: number,
    stdout: string,
    stderr: string
}

export async function exe(command: string): Promise<Output> {
    const {errno, stdout, stderr} = await exec(`${command}`);
    return {code: errno, stdout, stderr}
}

export function launchBrowser(link: string) {
    exe(`am start -a android.intent.action.VIEW -d ${link}`)
}

export function fseec(args: string[]): Promise<Output> {
    //                    ${FSEEMODDIR}/bin/fseec
    return exe(`/data/local/tmp/fseec ${args.join(' ')}`)
}

export function info(arg: string): Promise<Output> {
    return fseec(['api', 'info', arg])
}

export function fseectl(arg: string): Promise<Output> {
    return fseec(['fseectl', arg])
}

function setting(args: string[]): Promise<Output> {
    return fseec(['api', 'setting', `${args.join(' ')}`])
}

export function setting_on(arg: string): Promise<Output> {
    return setting([arg, 'on'])
}

export function setting_off(arg: string): Promise<Output> {
    return setting([arg, 'off'])
}

export function setting_get(arg: string): Promise<Output> {
    return setting([arg, 'get'])
}