import init, { Board, Point } from 'astar-wasm/astar_rust_wasm'

const wasmInit = await init()
const memory = wasmInit.memory


const CELL_SIZE = 5 * devicePixelRatio

const board = Board.new()
const width = board.width()
const height = board.height()

// refactor...
const gridCanvas = document.getElementById("board-canvas-grid") as HTMLCanvasElement
gridCanvas.height = height * CELL_SIZE
gridCanvas.width = width * CELL_SIZE
gridCanvas.style.width = width * (CELL_SIZE / devicePixelRatio) + "px";
gridCanvas.style.height = height * (CELL_SIZE / devicePixelRatio) + "px";

const canvas = document.getElementById("board-canvas") as HTMLCanvasElement
canvas.height = height * CELL_SIZE
canvas.width = width * CELL_SIZE
canvas.style.width = width * (CELL_SIZE / devicePixelRatio) + "px";
canvas.style.height = height * (CELL_SIZE / devicePixelRatio) + "px";

const pointInfoSpan = document.getElementById("point-info") as HTMLElement
const pathInfoSpan = document.getElementById("path-info") as HTMLElement
const multiplierInput = document.getElementById("heuristical-multiplier") as HTMLInputElement
const ticksPerFrameRange = document.getElementById("ticks-per-frame") as HTMLInputElement

const context = canvas.getContext('2d');
const gridContext = gridCanvas.getContext('2d');

type Pointy = {
    x: Readonly<number>,
    y: Readonly<number>,
}

const coordinateToPointy = (x: number, y: number): Pointy => ({
    x: Math.floor(x / (CELL_SIZE / devicePixelRatio)),
    y: Math.floor(y / (CELL_SIZE / devicePixelRatio))
})


let from: Pointy | undefined = undefined;
let to: Pointy | undefined = undefined;


const renderImage = (context: CanvasRenderingContext2D) => {
    board.render()
    const imageDataRaw = new Uint8Array(memory.buffer, board.frame_data(), width * height * 4)
    // const imageDataRaw = new Uint8ClampedArray(buffer)

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
}

const drawGrid = (context: CanvasRenderingContext2D) => {
    context.beginPath();
    context.fillStyle = `rgba(255,255,255,0.2)`
    context.lineWidth = 0.1;

    // vertical lines
    for (let col = 0; col < width; col++) {
        context.moveTo(col * CELL_SIZE, 0);
        context.lineTo(col * CELL_SIZE, height * CELL_SIZE);
    }

    // horizontal lines
    for (let row = 0; row < height; row++) {
        context.moveTo(0, row * CELL_SIZE);
        context.lineTo(width * CELL_SIZE, row * CELL_SIZE);
    }

    context.stroke();
}


if (context) {
    const tick = (ticksPerFrame: number, currentTo: Pointy) => {
        const result = board.tick(ticksPerFrame)
        renderImage(context)

        if (result === undefined && currentTo === to) {
            requestAnimationFrame(() => tick(ticksPerFrame, currentTo));
        }
    };


    canvas.onclick = e => {
        const point = coordinateToPointy(e.offsetX, e.offsetY)

        if (!from) {
            from = point
            board.set_from(point.x, point.y)
            renderImage(context)
        }
        else {
            to = point
            board.start_path_find(Point.new(from.x, from.y), Point.new(to.x, to.y), Number.parseInt(multiplierInput.value) ?? 1)

            // 0 here works only because the u32 on rust side breaks down and happily wraps around when decreasing...
            // actually this should probably panic but i guess this isnt perfect :O
            tick(ticksPerFrameRange.valueAsNumber > 100 ? 0 : ticksPerFrameRange.valueAsNumber, to)
        }
    }

    canvas.oncontextmenu = e => {
        e.preventDefault()
        pathInfoSpan.innerText = `distance: `
        from = coordinateToPointy(e.offsetX, e.offsetY)
        board.set_from(from.x, from.y)
        renderImage(context)
    }

    canvas.onpointermove = e => {
        const point = coordinateToPointy(e.offsetX, e.offsetY)
        const cellInfo = board.get_cell_info(point.x, point.y)
        pointInfoSpan.innerText = `x: ${point.x}, y: ${point.y}, weight: ${cellInfo?.toFixed(2)}`
    }

    renderImage(context)
}

if (gridContext) {
    drawGrid(gridContext)
}