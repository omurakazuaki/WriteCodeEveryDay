export default class JsGameOfLife {
  constructor(width, height) {
    this.width = width;
    this._cells = new Int8Array(width * height).map(_=>Math.random() < 0.5 ? 1 : 0);
  }

  static new(width, height) {
    return new JsGameOfLife(width, height);
  }

  cells() {
    return this._cells;
  }

  tick() {
    const width = this.width;
    this._cells = this._cells.map((cell, currentIdx, cells) => {
      const sum =
        [ currentIdx-width-1, currentIdx-width, currentIdx-width+1,
          currentIdx-1, currentIdx+1,
          currentIdx+width-1, currentIdx+width, currentIdx+width+1
        ]
      .filter(idx =>
        -1 < idx && idx < cells.length &&
        !(currentIdx % width == 0 && idx % width == width - 1) &&
        !(currentIdx % width == width - 1 && idx % width == 0))
      .reduce((sum, idx) => sum + cells[idx], 0);
      if (cell == 0 && sum == 3) {
        return 1;
      } else if (cell == 1 && 1 < sum && sum < 4) {
        return 1;
      } else {
        return 0;
      }
    });
  }
}
