import init, { Board, Point } from 'astar-wasm/astar_rust_wasm'

const wasmInit = await init()
const memory = wasmInit.memory


const CELL_SIZE = 5 * devicePixelRatio

const board = Board.new()
const width = board.width()
const height = board.height()

const canvas = document.getElementById("board-canvas") as HTMLCanvasElement
canvas.height = height * CELL_SIZE
canvas.width = width * CELL_SIZE
canvas.style.width = width * (CELL_SIZE / devicePixelRatio) + "px";
canvas.style.height = height * (CELL_SIZE / devicePixelRatio) + "px";

const pointInfoSpan = document.getElementById("point-info") as HTMLElement
const pathInfoSpan = document.getElementById("path-info") as HTMLElement
const multiplierInput = document.getElementById("heuristical-multiplier") as HTMLInputElement

const context = canvas.getContext('2d');

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
    const imageDataRaw = new Uint8Array(memory.buffer, board.render(), width * height * 4)
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

    drawGrid(context)
}

const drawGrid = (context: CanvasRenderingContext2D) => {
    context.beginPath();
    context.fillStyle = `rgba(255,255,255,0.2)`
    context.lineWidth = 0.1;

    // vertical grid
    for (let col = 0; col < width; col++) {
        context.moveTo(col * CELL_SIZE, 0);
        context.lineTo(col * CELL_SIZE, height * CELL_SIZE);
    }

    // horizontal grid
    for (let row = 0; row < height; row++) {
        context.moveTo(0, row * CELL_SIZE);
        context.lineTo(width * CELL_SIZE, row * CELL_SIZE);
    }

    context.stroke();
}


if (context) {
    const tick = () => {
        const result = board.tick(50)
        renderImage(context)

        if (result === undefined) {
            requestAnimationFrame(tick);
        }
    };


    canvas.onclick = e => {
        const point = coordinateToPointy(e.offsetX, e.offsetY)

        if (!from) {
            from = point
            board.click_cell(point.x, point.y)
            renderImage(context)
        }
        else {
            to = point
            const multiplier = Number.parseInt(multiplierInput.value) ?? 1

            board.start_path_find(Point.new(from.x, from.y), Point.new(to.x, to.y), multiplier)

            tick()



            // const pathStatistics = board.calculate_path(Point.new(from.x, from.y), Point.new(to.x, to.y), multiplier)
            // pathInfoSpan.innerText = `distance: ${pathStatistics?.total_distance.toFixed(2)}`
            // console.debug(pathStatistics?.path_nodes_count)
            // console.debug(pathStatistics?.nodes_visited_count)
        }
    }

    canvas.oncontextmenu = e => {
        e.preventDefault()
        pathInfoSpan.innerText = `distance: `
        from = coordinateToPointy(e.offsetX, e.offsetY)
        board.click_cell(from.x, from.y)
        renderImage(context)
    }

    canvas.onpointermove = e => {
        const point = coordinateToPointy(e.offsetX, e.offsetY)
        const cellInfo = board.get_cell_info(point.x, point.y)
        pointInfoSpan.innerText = `x: ${point.x}, y: ${point.y}, weight: ${cellInfo?.toFixed(2)}`
    }

    renderImage(context)
}
