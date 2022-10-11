const BOARD_SIZE: i32 = 10;

const INITIAL_STATE: [[i8; BOARD_SIZE as usize]; BOARD_SIZE as usize] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

struct Cell {
    is_alive: bool,
    pos_x: i32,
    pos_y: i32,
    neighbours: Neighbours,
}

struct Neighbours {
    top_left: Option<Box<Cell>>,
    top: Option<Box<Cell>>,
    top_right: Option<Box<Cell>>,
    left: Option<Box<Cell>>,
    right: Option<Box<Cell>>,
    bottom_left: Option<Box<Cell>>,
    bottom: Option<Box<Cell>>,
    bottom_right: Option<Box<Cell>>,
}

struct Board {
    field: [[Option<Box<Cell>>; BOARD_SIZE as usize]; BOARD_SIZE as usize],
}

fn get_cell_char(_cell: Cell) -> char {
    if _cell.is_alive {
        '●'
    } else {
        '○'
    }
}

fn print_board(_board: Board) {
    // Prints the board on the screen
}

fn init_board(initial_state: [[i8; BOARD_SIZE as usize]; BOARD_SIZE as usize]) -> Board {
    // Initial state of the board
    let mut board: Board;
    
    for i in 0..initial_state.len() {
        for j in 0..initial_state[i].len() {
            board[i][j] = new Cell({
                is_alive: initial_state[i][j] as bool,
                pos_x: j,
                pos_y: i,
            });
        }
    }
}

fn main() {
    let mut board = init_board(INITIAL_STATE);

    print_board(board);

    println!("Hello, world!");
}
