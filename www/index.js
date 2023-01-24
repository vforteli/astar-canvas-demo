import { Board } from "astar-wasm";
import { memory } from "astar-wasm/astar_rust_wasm_bg.wasm";
import { colorHex } from "./utils";

const CELL_SIZE = 5

const board = Board.new()
const width = board.width()
const height = board.height()

const canvas = document.getElementById("board-canvas")
canvas.height = height * CELL_SIZE
canvas.width = width * CELL_SIZE

const pointInfoSpan = document.getElementById("point-info")

const context = canvas.getContext('2d');


canvas.addEventListener('click', e => {
    board.click_cell(Math.floor(e.layerX / CELL_SIZE), Math.floor(e.layerY / CELL_SIZE))
    renderImage(context)
}, false);

canvas.addEventListener('mousemove', e => {
    const x = Math.floor(e.layerX / CELL_SIZE)
    const y = Math.floor(e.layerY / CELL_SIZE)
    const cellInfo = board.get_cell_info(x, y)
    pointInfoSpan.innerText = `x: ${x}, y: ${y}, weight: ${cellInfo}`
}, false);





const renderImage = (context) => {
    if (context) {
        const buffer = new Uint8Array(memory.buffer, board.render(), width * height * 4)
        const imageDataRaw = new Uint8ClampedArray(buffer)

        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const pixelIndex = row * (width * 4) + (col * 4)

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
    }
}


if (context) {
    renderImage(context)
}