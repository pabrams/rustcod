use tcod::console::*;
use tcod::colors::*;

#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    // move by the given amount, if the destination is not blocked
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {  
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground (self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map,
}

fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    map[30][22] = Tile::wall();
    map[50][24] = Tile::wall();
    map
}

fn render(tcod: &mut Tcod, game: &Game, objects: &[Object]) {
    for object in objects {
        object.draw(&mut tcod.con);
    }

    // go through all tiles, and set their background color
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    // blit the contents of "con" to the root console and present it
    blit(
        &tcod.con,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}
const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum
struct Tcod {
    root: Root,
    con: Offscreen,
}

fn handle_keys(tcod: &mut Tcod, player: &mut Object, game: &Game) -> bool {

    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    
    let key = tcod.root.wait_for_keypress(true);
    if key.pressed {
        match key {
            Key {
                code: Enter,
                alt: true,
                ..
            } => {
                // Alt+Enter: toggle fullscreen
                let fullscreen = tcod.root.is_fullscreen();
                tcod.root.set_fullscreen(!fullscreen);
            }
            Key { code: Escape, .. } => return true, // exit game

            Key { code: Up, .. } => player.move_by(0, -1, game),
            Key { code: Down, .. } => player.move_by(0, 1, game),
            Key { code: Left, .. } => player.move_by(-1, 0, game),
            Key { code: Right, .. } => player.move_by(1, 0, game),
            _ => {}
        }
    }
    false
}

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();
        
    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    let mut tcod = Tcod { root, con };
    tcod::system::set_fps(LIMIT_FPS);

    const START_X: i32 = SCREEN_WIDTH / 2;
    const START_Y: i32 = SCREEN_HEIGHT / 2;

    let player = Object::new(START_X, START_Y, '@', WHITE);
    let npc = Object::new(START_X - 5, START_Y, '@', YELLOW);

    let mut objects = [player, npc];

    let game = Game {
        map: make_map(),
    };

    while !tcod.root.window_closed() {
        tcod.con.clear();
        render(&mut tcod, &game, &objects);
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);

        let player = &mut objects[0];
        // handle keys and exit game if needed
        let exit = handle_keys(&mut tcod, player, &game);
        if exit {
            break;
        }
    }
}
