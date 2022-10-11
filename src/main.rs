use tcod::console::*;
use tcod::colors::*;

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

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground (self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum
struct Tcod {
    root: Root,
    con: Offscreen,
}

fn handle_keys(tcod: &mut Tcod, player: &mut Object) -> bool {

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

            Key { code: Up, .. } => player.move_by(0, -1),
            Key { code: Down, .. } => player.move_by(0, 1),
            Key { code: Left, .. } => player.move_by(-1, 0),
            Key { code: Right, .. } => player.move_by(1, 0),
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
        
    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut tcod = Tcod { root, con };
    tcod::system::set_fps(LIMIT_FPS);

    const START_X: i32 = SCREEN_WIDTH / 2;
    const START_Y: i32 = SCREEN_HEIGHT / 2;

    let player = Object::new(START_X, START_Y, '@', WHITE);
    let npc = Object::new(START_X - 5, START_Y, '@', YELLOW);

    let mut objects = [player, npc];

    while !tcod.root.window_closed() {
        tcod.con.clear();

        for object in &objects {
            object.draw(&mut tcod.con);
        }

        // blit the contents of "con" to the root console and present it
        blit(
            &tcod.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0,
        );

        tcod.root.flush();
        tcod.root.wait_for_keypress(true);

        let player = &mut objects[0];
        // handle keys and exit game if needed
        let exit = handle_keys(&mut tcod, player);
        if exit {
            break;
        }
    }
}
