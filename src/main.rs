mod tetris;

use tetris::Tetris;

fn main() {
    let mut game:Tetris = Tetris::new();
    game.run();
}
