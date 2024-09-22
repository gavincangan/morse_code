# Morse Code Converter

I have long wanted to try out Rust + WebAssembly (WASM). So here is a simple project to try it out. This is a simple Morse code converter and allows users to convert text to Morse code and vice versa through a web interface. It is hosted on GitHub Pages and can be accessed [here](https://bgav.in/morse_code). 

The primary goal of this project was to practice my Rust, and gain some hands-on experience with Rust and WebAssembly, so the implementation is likely quite inefficient.

## Technologies Used
- Rust
- WebAssembly (WASM)
- HTML/CSS/JavaScript
- GitHub Actions (for deployment)

## Features
- Convert text to Morse code
- Convert Morse code to text
- Real-time conversion in the browser

## Setup and Installation
1. Clone the repository
2. Ensure you have Rust and `wasm-pack` installed
3. Build the project: `wasm-pack build --target web`
4. Serve the `www` directory using a local web server

## Usage
Visit the [live demo](https://bgav.in/morse_code) to try it out.

## License
[MIT](https://choosealicense.com/licenses/mit/)

## Acknowledgements
- Claude 3.5 Sonnet for assistance in code generation
- The Rustaceans for their excellent intro documentation and for a compiler that wants to helps you (??!)