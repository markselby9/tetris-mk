import("../pkg/index.js").then(t => {
    bootstrap(t)
}).catch(console.error);
const pre = document.getElementById('tetris-mk-canvas');
const score = document.getElementById('score');
const next_shape = document.getElementById('next_shape');

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
        score.textContent = board.get_score();
        next_shape.textContent = board.get_next_shape_type();

        if (!board.tick()) {
            alert("Game over! Your score: " + board.get_score());
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
            case "r":
                board.rotate();
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
    document.getElementById('rotate').onclick
        = () => board.rotate();

    requestAnimationFrame(renderLoop);
};