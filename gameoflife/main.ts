const SIZE = 20;
const N = 30;

//create Game type

type Game = {
  grid: boolean[][];
  movement: number;
};
let game: Game = {
  grid: [],
  movement: 0,
};

const initGrid = () => {
  //nxn boolean grid
  game.grid = new Array(N).fill(false).map(() => new Array(N).fill(false));
};

const drawBase = () => {
  let canvas = document.getElementById("canvas") as HTMLCanvasElement;
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
  let canvas = document.getElementById("canvas") as HTMLCanvasElement;
  let ctx = canvas.getContext("2d");
  if (ctx) {
    ctx?.clearRect(0, 0, canvas.width, canvas.height);
    for (let i = 0; i < N; i++) {
      for (let j = 0; j < N; j++) {
        if (game.grid[i][j]) {
          ctx.fillStyle = "black";
          ctx.fillRect(i * SIZE, j * SIZE, SIZE, SIZE);
        } else {
          ctx.strokeStyle = "grey";
          ctx.fillStyle = "white";
          ctx.lineWidth = 0.5;
          ctx.fillRect(i * SIZE, j * SIZE, SIZE, SIZE);
        }
      }
    }
    ctx.stroke();
  }
}

const handleCanvasClick = () => {
  let canvas = document.getElementById("canvas") as HTMLCanvasElement;
  canvas.addEventListener("click", (e) => {
    //get position of click, snap to grid
    let x = Math.floor(e.offsetX / SIZE);
    let y = Math.floor(e.offsetY / SIZE);
    game.grid[x][y] = !game.grid[x][y];
    drawGrid();
  });
};

const handleNext = () => {
  let nextButton = document.getElementById("next") as HTMLButtonElement;
  let countSurrounding: (x: number, y: number) => number = (x, y) => {
    let count = 0;
    for (let i = -1; i <= 1; i++) {
      for (let j = -1; j <= 1; j++) {
        if (i == 0 && j == 0) continue;
        if (x + i >= 0 && x + i < N && y + j >= 0 && y + j < N) {
          if (game.grid[x + i][y + j]) {
            count++;
          }
        }
      }
    }
    return count;
  };
  nextButton.addEventListener("click", () => {
    //simulate conway's game of life on grid
    let nextGrid = new Array(N).fill(false).map(() => new Array(N).fill(false));

    for (let i = 0; i < N; i++) {
      for (let j = 0; j < N; j++) {
        let count = countSurrounding(i, j);
        if (game.grid[i][j]) {
          if (count < 2 || count > 3) {
            nextGrid[i][j] = false;
          } else {
            nextGrid[i][j] = true;
          }
        } else {
          if (count === 3) {
            nextGrid[i][j] = true;
          }
        }
      }
    }
    game.grid = nextGrid;
    drawGrid();
  });
};

//draw rectangle in canvas on document load
window.onload = () => {
  initGrid();
  drawBase();
  handleCanvasClick();
  handleNext();
};
