use super::{Tetris, Pieza, utils::vector2::Vec2i};
pub struct DisplayManager{
}

impl DisplayManager{
    pub const DISPLAY:Vec2i = Vec2i::new(30,30);
    pub const BOXES:[[i64;4];4] = [ // son los limites de cada caja con informacion en la pantalla, las coordenadas estan duplicadas en X porque cada "pixel" son 2 caracteres
        [ // tablero
            DisplayManager::DISPLAY.x/2-6, 
            DisplayManager::DISPLAY.y/2-11, 
            DisplayManager::DISPLAY.x/2+5, 
            DisplayManager::DISPLAY.y/2+10
        ],[// score
            DisplayManager::DISPLAY.x/2-15,
            DisplayManager::DISPLAY.y/2+8,
            DisplayManager::DISPLAY.x/2-7,
            DisplayManager::DISPLAY.y/2+10
        ],[// level
            DisplayManager::DISPLAY.x/2-13,
            DisplayManager::DISPLAY.y/2+5,
            DisplayManager::DISPLAY.x/2-7,
            DisplayManager::DISPLAY.y/2+7
        ],[// prox pieza
            DisplayManager::DISPLAY.x/2+6,
            DisplayManager::DISPLAY.y/2+4,
            DisplayManager::DISPLAY.x/2+12,
            DisplayManager::DISPLAY.y/2+10
        ]
    ];


    pub fn paint(game:&Tetris){
        const DISPLAY:&Vec2i = &DisplayManager::DISPLAY;
        const BOXES:&[[i64;4];4] = &DisplayManager::BOXES;

        let mut output:[[char; DISPLAY.y as usize]; (DISPLAY.x*2) as usize] = [[' '; DISPLAY.y as usize]; (DISPLAY.x*2) as usize];
        let mut tablero:[[char; 20];20] = [[' ';20];20];

        for p in game.escena.iter(){
            for v in Pieza::PIEZAS[ p.t_pieza ][ p.rotacion ].iter() {
                tablero[((v.x + p.pos.x)*2) as usize][(v.y + p.pos.y) as usize] = '█';
                tablero[((v.x + p.pos.x)*2 + 1) as usize][(v.y + p.pos.y) as usize] = '█';
            }
        }
        for v in Pieza::PIEZAS[game.piezaActual.t_pieza][game.piezaActual.rotacion].iter() {
            tablero[((v.x + game.piezaActual.pos.x)*2) as usize][(v.y + game.piezaActual.pos.y) as usize] = '▓';
            tablero[((v.x + game.piezaActual.pos.x)*2 + 1) as usize][(v.y + game.piezaActual.pos.y) as usize] = '▓';
        }



        for i in 0..DISPLAY.x {
            for j in 0..DISPLAY.y {                                                                        // itero por cada x y en la pantalla
                for k in 0..BOXES.len() {                                                                // por cada uno reviso cada box
                    if i>=BOXES[k][0] && j>=BOXES[k][1] && i<=BOXES[k][2] && j<=BOXES[k][3] {                   // y me fijo si estoy tocando alguna
                        if i>BOXES[k][0] && j>BOXES[k][1] && i<BOXES[k][2] && j<BOXES[k][3] {                   // si estoy adentro ...
                            if k==0 { // paso el punto xy del tablero al output
                                let pos_tablero:Vec2i = Vec2i::new((i-1-BOXES[k][0])*2, j-1-BOXES[k][1]);       // consigo las coords del punto en el tablero

                                output[(i*2) as usize][j as usize] = tablero[pos_tablero.x as usize][pos_tablero.y as usize];//paso ambos chars del pixel
                                output[(i*2+1) as usize][j as usize] = tablero[(pos_tablero.x+1) as usize][pos_tablero.y as usize];
                            } else if k==1 { // escribo y pongo los chars correspondientes al display SCORE
                                let pos_displ_puntos:Vec2i = Vec2i::new((i-1-BOXES[k][0])*2, j-1-BOXES[k][1]);

                                let mut display_puntos:String = DisplayManager::space(&"SCORE".to_string(), -6);
                                display_puntos.push(':');
                                display_puntos.push_str(&DisplayManager::space(&game.puntos.to_string(), 7));


                                if let Some(c) = display_puntos.chars().nth(pos_displ_puntos.x as usize) {
                                    output[(i*2) as usize][j as usize] = c;
                                    if let Some(c) = display_puntos.chars().nth((pos_displ_puntos.x+1) as usize) {
                                        output[(i*2+1) as usize][j as usize] = c;
                                    } else {
                                        output[(i*2+1) as usize][j as usize] = 'X';
                                    }
                                } else {
                                    output[(i*2+1) as usize][j as usize] = 'X';
                                }
                            } else if k==2 {
                                let pos_displ_lvl:Vec2i = Vec2i::new((i-1-BOXES[k][0])*2, j-1-BOXES[k][1]);

                                let mut display_nivel:String = DisplayManager::space(&"LEVEL".to_string(), -6);
                                display_nivel.push(':');
                                display_nivel.push_str(&DisplayManager::space(&game.nivel.to_string(), 3));


                                if let Some(c) = display_nivel.chars().nth(pos_displ_lvl.x as usize) {
                                    output[(i*2) as usize][j as usize] = c;
                                    if let Some(c) = display_nivel.chars().nth((pos_displ_lvl.x+1) as usize) {
                                        output[(i*2+1) as usize][j as usize] = c;
                                    } else {
                                        output[(i*2+1) as usize][j as usize] = 'X';
                                    }
                                } else {
                                    output[(i*2+1) as usize][j as usize] = 'X';
                                }
                            } else if k==3 {
                                let pos_displ_prox_pieza:Vec2i = Vec2i::new(i-1-BOXES[k][0], j-1-BOXES[k][1]);

                                for v in Pieza::PIEZAS[game.piezaSiguiente.t_pieza][game.piezaSiguiente.rotacion].iter() {
                                    if v.x+2 == pos_displ_prox_pieza.x && v.y+2 == pos_displ_prox_pieza.y {
                                        output[(i*2) as usize][j as usize] = '█';
                                        output[(i*2+1) as usize][j as usize] = '█';
                                    }

                                }
                            }
                        } else {                                                                                // si estoy en el borde ...
                                 if i==BOXES[k][0] && j==BOXES[k][1] { output[(i*2) as usize][j as usize]=' '; output[(i*2+1) as usize][j as usize]='╚';}
                            else if i==BOXES[k][2] && j==BOXES[k][3] { output[(i*2) as usize][j as usize]='╗'; output[(i*2+1) as usize][j as usize]=' ';}
                            else if i==BOXES[k][0] && j==BOXES[k][3] { output[(i*2) as usize][j as usize]=' '; output[(i*2+1) as usize][j as usize]='╔';}
                            else if i==BOXES[k][2] && j==BOXES[k][1] { output[(i*2) as usize][j as usize]='╝'; output[(i*2+1) as usize][j as usize]=' ';}
                            else if i==BOXES[k][0]                   { output[(i*2) as usize][j as usize]=' '; output[(i*2+1) as usize][j as usize]='║';}
                            else if i==BOXES[k][2]                   { output[(i*2) as usize][j as usize]='║'; output[(i*2+1) as usize][j as usize]=' ';}
                            else if j==BOXES[k][1] || j==BOXES[k][3] { output[(i*2) as usize][j as usize]='═'; output[(i*2+1) as usize][j as usize]='═';}
                        }
                    }
                }
            }
        }


        for j in (0..output[0].len()).rev() {
            for i in 0..output.len() {
                print!("{}", output[i][j])
            }
            print!("\n")
        }
    }   
    
    fn space(texto:&String, espacio:i16) -> String{
        let mut ret:String = String::new();

        if espacio>0 {
        
            for i in 0..espacio {
                if let Some(c) = texto.chars().nth(i as usize) {
                    ret.push(c);
                } else {
                    ret.push(' ');
                }
            }
        } else {
        
            for i in espacio..0{
                if let Some(c) = texto.chars().nth((i+texto.len() as i16) as usize){
                    ret.push(c);
                } else {
                    ret.push(' ');
                }
            }
        }
        ret
    }
}