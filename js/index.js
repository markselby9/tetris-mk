import("../pkg/index.js").then(t => {
    bootstrap(t)
}).catch(console.error);
const pre = document.getElementById('tetris-mk-canvas');

const DIRECTIONS = {
    'LEFT': 0,
    'RIGHT': 1,
    'DOWN': 2,
};

const bootstrap = (modules) => {
    const {Board} = modules;
    let board = Board.new(10, 20);

    const renderLoop = () => {
        pre.textContent = board.render();
        if (!board.tick()) {
            board = Board.new(10, 20);
        }

        setTimeout(() => {
            requestAnimationFrame(renderLoop);
        }, 500);
    };


    document.onkeydown = (e) => {
        const event = e || window.event;
        switch (e.key) {
            case "Down": // IE/Edge specific value
            case "ArrowDown":
            case "j":
                board.move_shape(DIRECTIONS.DOWN);
                break;
            case "Left": // IE/Edge specific value
            case "ArrowLeft":
            case "h":
                board.move_shape(DIRECTIONS.LEFT);
                break;
            case "Right": // IE/Edge specific value
            case "ArrowRight":
            case "l":
                board.move_shape(DIRECTIONS.RIGHT);
                break;
            default:
                return;
        }
        pre.textContent = board.render();
    };

    document.getElementById('down').onclick
        = () => board.move_shape(DIRECTIONS.DOWN);
    document.getElementById('left').onclick
        = () => board.move_shape(DIRECTIONS.LEFT);
    document.getElementById('right').onclick
        = () => board.move_shape(DIRECTIONS.RIGHT);

    requestAnimationFrame(renderLoop);
};