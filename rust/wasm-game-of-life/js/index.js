import("../pkg/index.js").then(({GameOfLife})=>{

  const CELL_SIZE = 5; // px
  const WIDTH = 256;
  const HEIGHT = 128;
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

  const renderLoop = (() => {
    const gol = GameOfLife.new(WIDTH, HEIGHT);
    return () => {
      gol.tick();
      drawCells(gol.cells());
      requestAnimationFrame(renderLoop);
    }
  })();

  drawGrid();
  requestAnimationFrame(renderLoop);

}).catch(console.error);
