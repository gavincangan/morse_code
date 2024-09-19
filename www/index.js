import init, { MorseConverter } from './pkg/morse_code_converter.js';

async function run() {
    await init();
    const converter = new MorseConverter();

    const inputElement = document.getElementById('input');
    const outputElement = document.getElementById('output');
    const toMorseButton = document.getElementById('toMorse');
    const fromMorseButton = document.getElementById('fromMorse');

    toMorseButton.addEventListener('click', () => {
        const text = inputElement.value;
        const morse = converter.to_morse(text);
        outputElement.textContent = morse;
    });

    fromMorseButton.addEventListener('click', () => {
        const morse = inputElement.value;
        const text = converter.from_morse(morse);
        outputElement.textContent = text;
    });
}

run();