<div>

  <h1><code>sudoku-rust-yew</code></h1>

  **A simple Sudoku game with Rust and <a href="https://yew.rs/">Yew</a>, using <a href="https://trunkrs.dev/">Trunk</a>.**


## About

This project is a simple yet challenging Sudoku game. It generates random Sudoku puzzles while ensuring they are always solvable, providing an engaging and varied experience for players.

## Features

- **Random Puzzle Generation:** Randomly generates Sudoku puzzles to ensure a unique and enjoyable experience every time.
- **Next step:** By clicking the "Next Step" button, users can reveal a cell that remains solvable by human logic, aiding progression through the Sudoku puzzle.
- **Check:** Clicking the "Check" button highlights incorrect fields in red, providing immediate visual feedback to the user about incorrect entries in the Sudoku puzzle.

## 🚴 Usage

This project uses WebAssembly to deliver an efficient and seamless Sudoku gaming experience directly within the browser.

### 🐑 Clone the repository

```
git clone https://github.com/HKarimiA/sudoku-rust-wasm.git
cd sudoku-rust-wasm
```

### 🛠️ Install Trunk

```
cargo install --locked trunk
```

**Other options:** There are options other than Trunk that may be used for bundling Yew applications. You might want to try one of these options:
- wasm-pack
- wasm-run
- xtask-wasm (still in early development)

## 🔋 Serving Locally

```
trunk serve --open
```
