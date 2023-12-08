import {readInput} from '../utils.js';
import {solvers} from '../solvers.js';

chrome.tabs.query({active: true, currentWindow: true}, (tabs) => {
    const currentTab = tabs[0];
    const currentUrl = currentTab?.url || '';
    if (currentUrl.startsWith("https://adventofcode.com/2023")) {
        const currentDay = currentUrl
            .replace('https://adventofcode.com/2023/', '')
            .split('/')[1];
        document.querySelector('#day').innerText = `Day ${currentDay}`;
        document.querySelectorAll('.solver-btn').forEach((b) => {
            const part = b.getAttribute('data-part');
            if (solvers[currentDay]?.[part]) {
                b.addEventListener('click', () => {
                    chrome.scripting.executeScript({
                        target: {tabId: currentTab.id},
                        func: readInput,
                    }).then((response) => {
                        const result = solvers[currentDay][part](response[0].result);
                        document.getElementById(`result-${part}`).innerText = `${result}`;
                    });
                });
                b.removeAttribute('disabled');
            }
        });
    } else {
        document.querySelector('#day').innerText = 'You can use this extension to solve only Advent Of Code 2023';
    }
});
