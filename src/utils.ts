export function splitLines(input: string): string[] {
    return input.split(/\r\n|\r|\n/);
}

export function readInput(): string {
    return document.body.innerText;
}
