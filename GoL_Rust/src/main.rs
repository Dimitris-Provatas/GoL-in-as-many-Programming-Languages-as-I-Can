use foreach::for_each;
use std::thread::sleep;
use std::time::{Duration, Instant};

const BOARD_SIZE: i32 = 17;

const STEP_INTERVAL: Duration = Duration::from_millis(750);

// Pulsar
const INITIAL_STATE: [[i8; BOARD_SIZE as usize]; BOARD_SIZE as usize] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

#[derive(Clone)]
#[derive(Copy)]
struct Cell {
    is_alive: bool,
    will_be_alive: bool,
    pos_x: i32,
    pos_y: i32,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            is_alive: false,
            will_be_alive: false,
            pos_x: 0,
            pos_y: 0,
        }
    }
}

#[derive(Clone)]
#[derive(Copy)]
struct Board {
    field: [[Cell; BOARD_SIZE as usize ]; BOARD_SIZE as usize],
}

impl Board {
    fn new() -> Board {
        Board { field: [[Cell::new(); BOARD_SIZE as usize]; BOARD_SIZE as usize] }
    }
}

fn init_board(initial_state: [[i8; BOARD_SIZE as usize]; BOARD_SIZE as usize]) -> Board {
    // Initial state of the board
    let mut board: Board = Board::new();

    for i in 0..initial_state.len() {
        for j in 0..initial_state[i].len() {
            board.field[i][j] = Cell{
                is_alive: initial_state[i][j] != 0,
                will_be_alive: false,
                pos_x: j as i32,
                pos_y: i as i32,
            };
        }
    }

    board
}

fn print_board(_board: Board) {
    print!("{}[2J", 27 as char);

    let mut field = _board.field.iter();

    // Prints the board on the screen
    for_each!(_cell_row in field {
        let mut cell_row = _cell_row.iter();
        
        for_each!(cell in cell_row {
            print!("{} ", get_cell_char(*cell));
        });
        print!("\n");
    });
}

fn get_cell_char(_cell: Cell) -> char {
    if _cell.is_alive {
        '●'
    } else {
        '○'
    }
}

fn update_cell(_cell: Cell, board: &mut Board) {
    board.field[_cell.pos_y as usize][_cell.pos_x as usize].is_alive = _cell.will_be_alive;
}

fn check_neighbours(_cell: Cell, board: &mut Board) {
    // tl tc tr
    //  l C  r
    // bl bc br

    // tl -> pos_x - 1, pos_y - 1
    // tc -> pos_x    , pos_y - 1
    // tr -> pos_x + 1, pos_y - 1
    // l  -> pos_x - 1, pos_y
    // r  -> pos_x + 1, pos_y
    // bl -> pos_x - 1, pos_y + 1
    // bc -> pos_x    , pos_y + 1
    // br -> pos_x + 1, pos_y + 1

    let mut alive_neighbours: i8 = 0;

    let is_upper_most: bool = _cell.pos_y == 0;
    let is_lower_most: bool = _cell.pos_y == BOARD_SIZE - 1;
    let is_left_most : bool = _cell.pos_x == 0;
    let is_right_most: bool = _cell.pos_x == BOARD_SIZE - 1;

    // tl
    if !is_upper_most && !is_left_most && board.field[(_cell.pos_y - 1) as usize][(_cell.pos_x - 1) as usize].is_alive {
        alive_neighbours += 1;
    }

    // tc
    if !is_upper_most && board.field[(_cell.pos_y - 1) as usize][(_cell.pos_x) as usize].is_alive {
        alive_neighbours += 1;
    }

    // tr
    if !is_upper_most && !is_right_most && board.field[(_cell.pos_y - 1) as usize][(_cell.pos_x + 1) as usize].is_alive {
        alive_neighbours += 1;
    }

    // l
    if !is_left_most && board.field[(_cell.pos_y) as usize][(_cell.pos_x - 1) as usize].is_alive {
        alive_neighbours += 1;
    }

    // r
    if !is_right_most && board.field[(_cell.pos_y) as usize][(_cell.pos_x + 1) as usize].is_alive {
        alive_neighbours += 1;
    }

    // bl
    if !is_lower_most && !is_left_most && board.field[(_cell.pos_y + 1) as usize][(_cell.pos_x - 1) as usize].is_alive {
        alive_neighbours += 1;
    }

    // bc
    if !is_lower_most && board.field[(_cell.pos_y + 1) as usize][(_cell.pos_x) as usize].is_alive {
        alive_neighbours += 1;
    }

    // br
    if !is_lower_most && !is_right_most && board.field[(_cell.pos_y + 1) as usize][(_cell.pos_x + 1) as usize].is_alive {
        alive_neighbours += 1;
    }

    if _cell.is_alive {
        board.field[_cell.pos_y as usize][_cell.pos_x as usize].will_be_alive = alive_neighbours == 3 || alive_neighbours == 2;
    } else {
        board.field[_cell.pos_y as usize][_cell.pos_x as usize].will_be_alive = alive_neighbours == 3;
    }
}

fn main() {
    let mut board = init_board(INITIAL_STATE);

    loop {
        print_board(board);

        let now = Instant::now();
        sleep(STEP_INTERVAL);
        assert!(now.elapsed() >= STEP_INTERVAL);

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                check_neighbours(board.field[i as usize][j as usize], &mut board)
            }
        }

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                update_cell(board.field[i as usize][j as usize], &mut board)
            }
        }
    }
}
