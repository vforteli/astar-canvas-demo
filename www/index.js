import { Board, Cell } from "astar-wasm";
import { memory, click } from "astar-wasm/astar_rust_wasm_bg.wasm";


// click(0, 10)
// wasm.greet();
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const CELL_SIZE = 5

const board = Board.new()
const width = board.width()
const height = board.height()

const canvas = document.getElementById("board-canvas")

const context = canvas.getContext('2d');


canvas.addEventListener('click', function (e) {
    console.debug(Math.floor(e.layerX / CELL_SIZE))
    console.debug(Math.floor(e.layerY / CELL_SIZE))
    board.click_cell(Math.floor(e.layerX / CELL_SIZE), Math.floor(e.layerY / CELL_SIZE))
    renderImage(context)
}, false);


canvas.height = height * CELL_SIZE
canvas.width = width * CELL_SIZE

const componentToHex = (c) => {
    var hex = c.toString(16)
    return hex.length == 1 ? "0" + hex : hex
}

const colorHex = (r, g, b) => {
    return `#${componentToHex(r)}${componentToHex(g)}${componentToHex(b)}`
}


const renderImage = (context) => {
    if (context) {
        const buffer = new Uint8Array(memory.buffer, board.image_data(), width * height * 4)
        const imageDataRaw = new Uint8ClampedArray(buffer)
        // console.debug(imageDataRaw)

        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const pixelIndex = row * (width * 4) + (col * 4)
                // console.debug(pixelIndex)

                // if (pixelIndex > 2000) return;

                const r = imageDataRaw[pixelIndex]
                const g = imageDataRaw[pixelIndex + 1]
                const b = imageDataRaw[pixelIndex + 2]
                const a = imageDataRaw[pixelIndex + 3]


                context.fillStyle = colorHex(r, g, b)

                context.fillRect(
                    col * CELL_SIZE,
                    row * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }

        // const imageData = new ImageData(imageDataRaw, width, height)
        // context.putImageData(imageData, 0, 0)
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