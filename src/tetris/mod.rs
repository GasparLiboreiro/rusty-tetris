mod utils {
    pub mod vector2;
    pub mod matrices;
}
mod pieza;
mod displayManager;
use displayManager::DisplayManager;
use pieza::Pieza;
use std::default;

use rand::Rng;

use utils::vector2::Vec2i;

pub struct Tetris{
    escena:Vec<Pieza>,
    puntos:i64,
    nivel:i64,
    piezaActual:Pieza,
    piezaSiguiente:Pieza,
    display:DisplayManager
}

impl Tetris{
    pub fn new()->Tetris{
        Tetris{
            escena:Vec::new(),
            puntos:0,
            nivel:1,
            piezaActual:Pieza::new(rand::thread_rng().gen_range(0..7)),
            piezaSiguiente:Pieza::new(rand::thread_rng().gen_range(0..7)),
            display:DisplayManager {}
        }
    }

    pub fn run(&mut self){
        self.paint();
        
        self.paint();
        self.paint();
        self.paint();
    }

    pub fn paint(&self){
        print!("{esc}c", esc = 27 as char); //cls
        DisplayManager::paint(&self);
    }
}