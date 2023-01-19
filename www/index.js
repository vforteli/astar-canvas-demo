import { Board, Cell } from "astar-wasm";
import { memory } from "astar-wasm/astar_rust_wasm_bg.wasm";

// wasm.greet();
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const board = Board.new()
const width = board.width()
const height = board.height()

const canvas = document.getElementById("board-canvas")
canvas.height = height
canvas.width = width

const getIndex = (row, column) => {
    return row * width + column;
}

const context = canvas.getContext('2d');
if (context) {


    const cellsPtr = board.cells()
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    context.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const index = getIndex(row, col);

            context.fillStyle = cells[index] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
            context.fillRect(col, row, 1, 1)
        }
    }

    context.stroke();
}