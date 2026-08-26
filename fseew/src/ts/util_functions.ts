import type {
    str,
    f64,
    bool
} from './define';

import {
    FSEEMODDIR
} from './define'

import {exec} from 'kernelsu';

interface Output {
    code: number,
    stdout: string,
    stderr: string
}

export async function exe(command: str): Promise<Output> {
    const {errno, stdout, stderr} = await exec(`${command}`);
    return {code: errno, stdout, stderr}
}

export function fseec(args: str[]): Promise<Output> {
    return exe(`${FSEEMODDIR}/bin/fseec ${args.join(' ')}`)
}