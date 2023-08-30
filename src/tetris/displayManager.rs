use super::{Tetris, utils::vector2::Vec2i};

pub struct DisplayManager{
}

impl DisplayManager{
    pub const DISPLAY:Vec2i = Vec2i::new(30,30);

    pub fn paint(game:&Tetris){
        for i in 0..DisplayManager::DISPLAY.y{
            for j in 0..DisplayManager::DISPLAY.x{
                //dibujar cada uno de los datablocks que contienen la data del juego
            }
            print!("\n")
        }
    }   
    pub fn data_block(data:&str, data_space:u8, name:&str, name_space:u8)->Vec<Vec<char>>{
        let mut out:Vec<Vec<char>> = vec![vec!['X';3]; (1+name_space+3+data_space+1) as usize];

        for i in 0..3 as usize {
            if i==1{
                out[0][1]='|';
                out[(1+name_space+3+data_space+1-1)as usize][1]='|';
                for j in 0..(name_space+3+data_space) as usize{
                    println!("{};{}",j,i);
                    if 0<=name_space as i16-name.len() as i16 && j<(name_space as i16-name.len() as i16) as usize {
                        out[1+j][i]=' ';
                    }else if j<name_space as usize {
                        println!("index:{}",(j as i16-(name_space as i16-name.len() as i16)));
                        out[1+j][i]=name.chars().collect::<Vec<char>>()[(j as i16-(name_space as i16-name.len() as i16)) as usize];
                    }else if j<(name_space+1) as usize {
                        out[1+j][i]=' ';
                    }else if j<(name_space+2) as usize {
                        out[1+j][i]=':';
                    }else if j<(name_space+3) as usize {
                        out[1+j][i]=' ';
                    }else if j<(name_space+3+data.len() as u8) as usize {
                        out[1+j][i]=data.chars().collect::<Vec<char>>()[j-(name_space as usize+3)];
                    }else if j<(name_space+3+data_space as u8) as usize {
                        out[1+j][i]=' ';
                    }
                }
            } else {
                for j in 0..(1+name_space+3+data_space+1) as usize{
                    out[j][i]='-';
                }
            }
        }

        out
    }
}