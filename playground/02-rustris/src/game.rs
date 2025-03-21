use crate::tile::{Tile, Typ, TETROMINO_SIZE, generate_random_tile};

pub type Board = Vec<Vec<Option<Typ>>>;

pub const BOARD_WIDTH: usize = 16;
pub const BOARD_HEIGHT: usize = 20;

pub struct Game {
    pub board: Board,
    pub current_tile: Tile,
    pub current_tile_position: (usize, usize),
    pub current_tile_rotation: usize,
    pub score: usize,
    pub level: usize,
    pub lines_cleared: usize,
    pub game_over: bool,

    // Other game state...
}

impl Game {
    pub fn new() -> Game {
        let random_tile = generate_random_tile();
        Game {
            board: vec![vec![None; BOARD_WIDTH]; BOARD_HEIGHT],
            current_tile: random_tile,
            current_tile_position: (0, 0),
            current_tile_rotation: 0,
            score: 0,
            level: 1,
            lines_cleared: 0,
            game_over: false,
        }
    }

    pub fn drop_current_tile(&mut self) {
        let shape = self.current_tile.shape(self.current_tile_rotation);
        for y in 0..TETROMINO_SIZE {
            for x in 0..TETROMINO_SIZE {
                if shape[y][x] {
                    let board_x = self.current_tile_position.0 + x;
                    let board_y = self.current_tile_position.1 + y;
                    self.board[board_y][board_x] = Some(self.current_tile.typ);
                }
            }
        }
    }
}
