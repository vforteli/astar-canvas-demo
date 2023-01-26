import { Board, Point } from "astar-wasm";
import { memory } from "astar-wasm/astar_rust_wasm_bg.wasm";

const CELL_SIZE = 5 * devicePixelRatio

const board = Board.new()
const width = board.width()
const height = board.height()

const canvas = document.getElementById("board-canvas")



canvas.height = height * CELL_SIZE
canvas.width = width * CELL_SIZE
canvas.style.width = width * (CELL_SIZE / devicePixelRatio) + "px";
canvas.style.height = height * (CELL_SIZE / devicePixelRatio) + "px";

const pointInfoSpan = document.getElementById("point-info")

const context = canvas.getContext('2d');


canvas.addEventListener('click', e => {
    const x = Math.floor(e.layerX / (CELL_SIZE / devicePixelRatio))
    const y = Math.floor(e.layerY / (CELL_SIZE / devicePixelRatio))

    const from = Point.new(0, 0)
    const to = Point.new(x, y)

    const distance = board.calculate_path(from, to, 1)

    console.debug(`length: ${distance}`)
    board.click_cell(x, y)
    renderImage(context)
}, false);

canvas.addEventListener('mousemove', e => {
    const x = Math.floor(e.layerX / 5)
    const y = Math.floor(e.layerY / 5)
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

                context.fillStyle = `rgba(${r},${g},${b},1.0)`

                context.fillRect(
                    col * CELL_SIZE,
                    row * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }

        for (let col = 0; col < width; col++) {
            context.beginPath();
            context.fillStyle = `rgba(255,255,255,0.2)`
            context.lineWidth = 0.1;
            context.moveTo(col * CELL_SIZE, 0);
            context.lineTo(col * CELL_SIZE, height * CELL_SIZE);
            context.stroke();
        }

        for (let row = 0; row < height; row++) {
            context.beginPath();
            context.fillStyle = `rgba(255,255,255,0.2)`
            context.lineWidth = 0.1;
            context.moveTo(0, row * CELL_SIZE);
            context.lineTo(width * CELL_SIZE, row * CELL_SIZE);
            context.stroke();
        }
    }
}


if (context) {
    renderImage(context)
}