import("../pkg/index.js").then(t => {
    bootstrap(t)
}).catch(console.error);
const pre = document.getElementById('tetris-mk-canvas');

const DIRECTIONS = [0, 1, 2];

const bootstrap = (modules) => {
    const {Board} = modules;
    let board = Board.new(10, 20);

    const renderLoop = () => {
        pre.textContent = board.render();
        board.move_shape(DIRECTIONS[Math.floor(Math.random() * 3)]);
        if (!board.tick()) {
            board = Board.new(10, 20);
        }

        setTimeout(() => {
            requestAnimationFrame(renderLoop);
        }, 100);
    };

    requestAnimationFrame(renderLoop);
};