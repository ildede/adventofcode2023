import { readInput, splitLines } from '../utils.js';

document.getElementById('read-content').addEventListener('click', () => {
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
            document.getElementById("result").innerText = `${result}`;
        });
    });
});
