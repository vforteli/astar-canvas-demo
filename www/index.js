import * as wasm from "hello-wasm-pack";

// wasm.greet();

const canvas = document.getElementById("board-canvas")
canvas.height = 500
canvas.width = 500

const context = canvas.getContext('2d');

if (context) {
    context.moveTo(0, 0)
    context.lineTo(250, 250)
    context.stroke()
}