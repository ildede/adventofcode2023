import {splitLines} from "./utils";
import init, { day_2_part_a, day_2_part_b, day_3_part_a } from "./wasm/adv2023_lib";

interface Solver {
    a?: (input: string) => Promise<string|number>,
    b?: (input: string) => Promise<string|number>;
}

export const solvers: Solver[]  = [
    {}, // Day 0
    { a: day1partA, b: day1partB },
    { a: day2partA, b: day2partB },
    { a: day3partA },
]

async function day1partA(input: string): Promise<number> {
    const splitted = splitLines(input);
    return splitted
        .map((e: string) => e.replaceAll(/[a-z]/g, ''))
        .map((e: string) => e ? `${e[0]}${e[e.length - 1]}` : '')
        .reduce((accumulator, currentValue) => currentValue ? accumulator + Number(currentValue) : accumulator, 0);
}

async function day1partB(input: string): Promise<number> {
    const splitted = splitLines(input);
    return splitted
                .map((e) => e.replaceAll(/one/g, 'o1e'))
                .map((e) => e.replaceAll(/two/g, 't2o'))
                .map((e) => e.replaceAll(/three/g, 't3e'))
                .map((e) => e.replaceAll(/four/g, 'f4r'))
                .map((e) => e.replaceAll(/five/g, 'f5e'))
                .map((e) => e.replaceAll(/six/g, 's6x'))
                .map((e) => e.replaceAll(/seven/g, 's7n'))
                .map((e) => e.replaceAll(/eight/g, 'e8t'))
                .map((e) => e.replaceAll(/nine/g, 'n9e'))
                .map((e) => e.replaceAll(/[a-z]/g, ''))
                .map((e) => e ? `${e[0]}${e[e.length - 1]}` : '')
                .reduce((accumulator, currentValue) => currentValue ? accumulator + Number(currentValue) : accumulator, 0);
}

async function day2partA(input: string): Promise<string> {
    return await init().then(() => day_2_part_a(input));
}

async function day2partB(input: string): Promise<string> {
    return await init().then(() => day_2_part_b(input));
}

async function day3partA(input: string): Promise<string> {
    return await init().then(() => day_3_part_a(input));
}