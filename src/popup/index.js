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
                .map((e) => e ? `${e[0]}${e[e.length-1]}` : '')
                .reduce((accumulator, currentValue) => currentValue ? accumulator + Number(currentValue) : accumulator, 0);
            document.getElementById("resultB").innerText = `${result}`;
        });
    });
});
