use rand::Rng;

pub const TETROMINO_SIZE: usize = 4;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Typ {
    B,
    I,
    L,
    J,
    T,
    S,
    Z,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Tile {
    pub typ: Typ,
	// drawings defines how the tile is drawn using a small DSL:
	//  'x' = draw + move right
	//  'c' = draw + move right + use as center point
	//  ' ' = move right
	//  ',' = next row
    pub drawings: [&'static str; 4],
}

impl Tile {
    pub fn shape(&self, rotation: usize) -> [[bool; TETROMINO_SIZE]; TETROMINO_SIZE] {
        let mut shape = [[false; TETROMINO_SIZE]; TETROMINO_SIZE];
        let drawing = self.drawings[rotation];
        let mut x = 0;
        let mut y = 0;
        for c in drawing.chars() {
            match c {
                'x' => shape[y][x] = true, // draw + move right
                'c' => shape[y][x] = true, // draw + move right + use as center point
                ' ' => {} // move right
                ',' => {
                    y += 1; // next row
                    x = 0;  // reset x position
                    continue;
                }
                _ => panic!("Invalid character in drawing: {}", c),
            }
            if c != ',' {
                x += 1;
            }
        }
        shape
    }
}

pub const TILES: [Tile; 7] = [
    Tile { typ: Typ::B, drawings: ["xc,xx", "xc,xx", "xc,xx", "xc,xx"] },
    Tile { typ: Typ::I, drawings: ["x,c,x,x", "xcxx", "x,x,c,x", "xxcx"] },
    Tile { typ: Typ::L, drawings: ["x,c,xx", "xcx,x", "xx, c, x", "  x,xcx"] },
    Tile { typ: Typ::J, drawings: [" x, c,xx", "x,xcx", "xx,c,x", "xcx,  x"] },
    Tile { typ: Typ::T, drawings: ["xcx, x", " x,xc, x", " x,xcx", "x,cx,x"] },
    Tile { typ: Typ::S, drawings: [" cx,xx", "x,xc, x", " xx,xc", "x,cx, x"] },
    Tile { typ: Typ::Z, drawings: ["xc, xx", " x,xc,x", "xx, cx", " x,cx,x"] },
];

pub fn generate_random_tile() -> Tile {
    let mut rng = rand::thread_rng();
    let random_tile_type = rng.gen_range(0..7);
    TILES[random_tile_type]
}
