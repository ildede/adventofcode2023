import { readInput, splitLines } from '../utils.js';

document.getElementById('day1-A').addEventListener('click', () => {
    chrome.tabs.query({active: true, currentWindow: true}, (tabs) => {
        const tab = tabs[0];

        chrome.scripting.executeScript({
            target: {tabId: tab.id},
            func: readInput,
        }).then((response) => {
            const splitted = splitLines(response[0].result);
            const result = splitted
                .map((e) => e.replaceAll(/[a-z]/g, ''))
                .map((e) => e ? `${e[0]}${e[e.length-1]}` : '')
                .reduce((accumulator, currentValue) => currentValue ? accumulator + Number(currentValue) : accumulator, 0);
            document.getElementById("resultA").innerText = `${result}`;
        });
    });
});
document.getElementById('day1-B').addEventListener('click', () => {
    chrome.tabs.query({active: true, currentWindow: true}, (tabs) => {
        const tab = tabs[0];

        chrome.scripting.executeScript({
            target: {tabId: tab.id},
            func: readInput,
        }).then((response) => {
            const splitted = splitLines(response[0].result);
            const result = splitted
                .map((e) => e.replaceAll(/one/g, '1'))
                .map((e) => e.replaceAll(/two/g, '2'))
                .map((e) => e.replaceAll(/three/g, '3'))
                .map((e) => e.replaceAll(/four/g, '4'))
                .map((e) => e.replaceAll(/five/g, '5'))
                .map((e) => e.replaceAll(/six/g, '6'))
                .map((e) => e.replaceAll(/seven/g, '7'))
                .map((e) => e.replaceAll(/eight/g, '8'))
                .map((e) => e.replaceAll(/nine/g, '9'))
                .map((e) => e.replaceAll(/[a-z]/g, ''))
                .map((e) => e ? `${e[0]}${e[e.length-1]}` : '')
                .reduce((accumulator, currentValue) => {
                    console.log(currentValue);
                    return (currentValue ? accumulator + Number(currentValue) : accumulator);
                }, 0);
            document.getElementById("resultB").innerText = `${result}`;
        });
    });
});
