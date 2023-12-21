import {splitLines} from "./utils.js";
import init, { day_2_part_a, day_2_part_b } from "./wasm/adv2023_lib.js";

export const solvers = [
    {}, // Day 0
    { a: day1partA, b: day1partB },
    { a: day2partA, b: day2partB },
]

async function day1partA(input) {
    const splitted = splitLines(input);
    return splitted
        .map((e) => e.replaceAll(/[a-z]/g, ''))
        .map((e) => e ? `${e[0]}${e[e.length - 1]}` : '')
        .reduce((accumulator, currentValue) => currentValue ? accumulator + Number(currentValue) : accumulator, 0);
}

async function day1partB(input) {
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

async function day2partA(input) {
    return await init().then(() => day_2_part_a(input));
}

async function day2partB(input) {
    return await init().then(() => day_2_part_b(input));
}