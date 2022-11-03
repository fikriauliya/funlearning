"use strict";
const SIZE = 20;
const N = 30;
let grid;
const initGrid = () => {
    //nxn boolean grid
    grid = new Array(N);
    for (let i = 0; i < N; i++) {
        grid[i] = new Array(N);
        for (let j = 0; j < N; j++) {
            grid[i][j] = false;
        }
    }
};
const drawBase = () => {
    let canvas = document.getElementById("canvas");
    let ctx = canvas.getContext("2d");
    if (ctx) {
        ctx.strokeStyle = "grey";
        ctx.lineWidth = 0.5;
        //create n x n rectangle with size SIZE without fill
        for (let i = 0; i < N; i++) {
            for (let j = 0; j < N; j++) {
                ctx.rect(i * SIZE, j * SIZE, SIZE, SIZE);
            }
        }
        ctx.stroke();
    }
};
function drawGrid() {
    let canvas = document.getElementById("canvas");
    let ctx = canvas.getContext("2d");
    if (ctx) {
        for (let i = 0; i < N; i++) {
            for (let j = 0; j < N; j++) {
                if (grid[i][j]) {
                    ctx.fillStyle = "black";
                    ctx.fillRect(i * SIZE, j * SIZE, SIZE, SIZE);
                }
            }
        }
    }
}
const handleCanvasClick = () => {
    let canvas = document.getElementById("canvas");
    canvas.addEventListener("click", (e) => {
        //get position of click, snap to grid
        let x = Math.floor(e.offsetX / SIZE);
        let y = Math.floor(e.offsetY / SIZE);
        grid[x][y] = !grid[x][y];
        drawGrid();
    });
};
const handleStart = () => {
    let startButton = document.getElementById("start");
    startButton.addEventListener("click", () => { });
};
//draw rectangle in canvas on document load
window.onload = () => {
    initGrid();
    drawBase();
    handleCanvasClick();
    handleStart();
};
