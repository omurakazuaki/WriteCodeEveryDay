import JsGameOfLife from "./game_of_life";
import("../pkg/index.js").then(({GameOfLife})=>{

  const params = new URL(window.location.href).searchParams;
  const isJs = params.get('l') == 'js';
  document.getElementById('title').innerText = `Game of Life with ${isJs ? 'Javascript': 'Rust'}`;

  const CELL_SIZE = 5; // px
  const WIDTH = Number(params.get('w')) || 400;
  const HEIGHT = Number(params.get('h')) || 200;
  const GRID_COLOR = "#282c34";
  const DEAD_COLOR = "#000000";
  const ALIVE_COLOR = "#61dafb";

  const canvas = document.getElementById("game-of-life-canvas");
  canvas.width = (CELL_SIZE + 1) * WIDTH + 1;
  canvas.height = (CELL_SIZE + 1) * HEIGHT + 1;

  const ctx = canvas.getContext("2d");

  const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= WIDTH; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * HEIGHT + 1);
    }

    for (let j = 0; j <= HEIGHT; j++) {
      ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * WIDTH + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
  };

  const drawCells = (() => {
    let prevCells = [];
    const getIndex = (row, column) => {
      return row * WIDTH + column;
    };
    return (cells) => {

      ctx.beginPath();

      for (let row = 0; row < HEIGHT; row++) {
        for (let col = 0; col < WIDTH; col++) {
          const idx = getIndex(row, col);
          if (cells[idx] == prevCells[idx]) {
            continue;
          }

          ctx.fillStyle = cells[idx] ? ALIVE_COLOR : DEAD_COLOR;
          ctx.fillRect(
            col * (CELL_SIZE + 1) + 1,
            row * (CELL_SIZE + 1) + 1,
            CELL_SIZE,
            CELL_SIZE
          );
        }
      }

      ctx.stroke();

      prevCells = cells;
    }
  })();

  const FPS = 60;
  const renderLoop = ((gol) => {
    return () => {
      gol.tick();
      drawCells(gol.cells());
      setTimeout(renderLoop, 1000 / FPS);
    }
  })(isJs ? JsGameOfLife.new(WIDTH, HEIGHT) : GameOfLife.new(WIDTH, HEIGHT));

  drawGrid();
  renderLoop();

}).catch(console.error);
