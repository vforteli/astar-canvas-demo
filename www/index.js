import { Board, Cell } from "astar-wasm";
import { memory, click } from "astar-wasm/astar_rust_wasm_bg.wasm";


// click(0, 10)
// wasm.greet();
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const CELL_SIZE = 4

const board = Board.new()
const width = board.width()
const height = board.height()

const canvas = document.getElementById("board-canvas")

const context = canvas.getContext('2d');


canvas.addEventListener('click', function (e) {
    board.click_cell(e.layerX, e.layerY)
    renderImage(context)
}, false);


canvas.height = height
canvas.width = width

const getIndex = (row, column) => {
    return row * width + column
}

const renderImage = (context) => {
    if (context) {
        const buffer = new Uint8Array(memory.buffer, board.image_data(), width * height * 4)
        const imageDataRaw = new Uint8ClampedArray(buffer)

        const imageData = new ImageData(imageDataRaw, width, height)
        context.putImageData(imageData, 0, 0)
    }
}


if (context) {
    renderImage(context)

    // const cellsPtr = board.cells()
    // const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    // context.beginPath();

    // for (let row = 0; row < height; row++) {
    //     for (let col = 0; col < width; col++) {
    //         const index = getIndex(row, col);

    //         context.fillStyle = cells[index] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
    //         context.fillRect(col, row, 1, 1)
    //     }
    // }

    // context.stroke();
}