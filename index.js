import init, { MorseConverter } from './pkg/morse_code.js';

async function run() {
    await init();
    const converter = new MorseConverter();

    const inputElement = document.getElementById('input');
    const triggerConversionButton = document.getElementById('triggerConversion');

    triggerConversionButton.addEventListener('click', () => {
        const input = inputElement.value;
        const output = converter.convert(input);
        inputElement.value = output;
    });
}

run();