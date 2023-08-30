use super::utils::vector2::Vec2i;
use super::utils::matrices::rotar_vec2i_90_n_veces;


pub struct Pieza{
    pub t_pieza:u8,
    pub rotacion:u8,
    pub pos:Vec2i
}

impl Pieza{
    pub const OFFSETS:[[[Vec2i;4];5];3] = [
        [// offsets piezas 0 a 4 inclusive
            [Vec2i::new(0,0), Vec2i::new(0,0), Vec2i::new(0,0), Vec2i::new(0,0)], //offset para cada rotacion en el test 1
            [Vec2i::new(0,0), Vec2i::new(1,0), Vec2i::new(0,0), Vec2i::new(-1,0)],// ~~     ~~   ~~     ~~    ~~ ~~ test 2  
            [Vec2i::new(0,0), Vec2i::new(1,-1), Vec2i::new(0,0), Vec2i::new(-1,-1)],
            [Vec2i::new(0,0), Vec2i::new(0,2), Vec2i::new(0,0), Vec2i::new(0,2)],
            [Vec2i::new(0,0), Vec2i::new(1,2), Vec2i::new(0,0), Vec2i::new(-1,2)]
        ],[// offsets pieza 6
            [Vec2i::new(0,0), Vec2i::new(-1,0), Vec2i::new(-1,1), Vec2i::new(0,1)],
            [Vec2i::new(-1,0), Vec2i::new(0,0), Vec2i::new(1,1), Vec2i::new(0,1)],
            [Vec2i::new(2,0), Vec2i::new(0,0), Vec2i::new(-2,1), Vec2i::new(0,1)],
            [Vec2i::new(-1,0), Vec2i::new(0,1), Vec2i::new(1,0), Vec2i::new(0,-1)],
            [Vec2i::new(2,0), Vec2i::new(0,-2), Vec2i::new(-2,0), Vec2i::new(0,2)]
        ],[// offsets pieza 5
            [Vec2i::new(0,0), Vec2i::new(0,-1), Vec2i::new(-1,-1), Vec2i::new(-1,0)],
            [Vec2i::new(0,0), Vec2i::new(0,-1), Vec2i::new(-1,-1), Vec2i::new(-1,0)],
            [Vec2i::new(0,0), Vec2i::new(0,-1), Vec2i::new(-1,-1), Vec2i::new(-1,0)],
            [Vec2i::new(0,0), Vec2i::new(0,-1), Vec2i::new(-1,-1), Vec2i::new(-1,0)],
            [Vec2i::new(0,0), Vec2i::new(0,-1), Vec2i::new(-1,-1), Vec2i::new(-1,0)]
        ]
    ]; //uso OFFSETS[tipo][n*test][offset por rotacion]

    pub const PIEZAS:[[[Vec2i;4];4];7] = [
        {// basandome en la primera, consigo las demas haciendo las rotaciones
            let mut out:[[Vec2i;4];4]=[[Vec2i::CERO;4];4];
            out[0]=[Vec2i::new(-1,0), Vec2i::new(0,0), Vec2i::new(0,1), Vec2i::new(1,1)];
            out[1]=[rotar_vec2i_90_n_veces(out[0][0],1),rotar_vec2i_90_n_veces(out[0][1],1),rotar_vec2i_90_n_veces(out[0][2],1),rotar_vec2i_90_n_veces(out[0][3],1)];
            out[2]=[rotar_vec2i_90_n_veces(out[0][0],2),rotar_vec2i_90_n_veces(out[0][1],2),rotar_vec2i_90_n_veces(out[0][2],2),rotar_vec2i_90_n_veces(out[0][3],2)];
            out[3]=[rotar_vec2i_90_n_veces(out[0][0],3),rotar_vec2i_90_n_veces(out[0][1],3),rotar_vec2i_90_n_veces(out[0][2],3),rotar_vec2i_90_n_veces(out[0][3],3)];
            out
        },
        {
            let mut out:[[Vec2i;4];4]=[[Vec2i::CERO;4];4];
            out[0]=[Vec2i::new(-1,1), Vec2i::new(0,1), Vec2i::new(0,0), Vec2i::new(1,0)];
            out[1]=[rotar_vec2i_90_n_veces(out[0][0],1),rotar_vec2i_90_n_veces(out[0][1],1),rotar_vec2i_90_n_veces(out[0][2],1),rotar_vec2i_90_n_veces(out[0][3],1)];
            out[2]=[rotar_vec2i_90_n_veces(out[0][0],2),rotar_vec2i_90_n_veces(out[0][1],2),rotar_vec2i_90_n_veces(out[0][2],2),rotar_vec2i_90_n_veces(out[0][3],2)];
            out[3]=[rotar_vec2i_90_n_veces(out[0][0],3),rotar_vec2i_90_n_veces(out[0][1],3),rotar_vec2i_90_n_veces(out[0][2],3),rotar_vec2i_90_n_veces(out[0][3],3)];
            out
        },
        {
            let mut out:[[Vec2i;4];4]=[[Vec2i::CERO;4];4];
            out[0]=[Vec2i::new(-1,0), Vec2i::new(0,0), Vec2i::new(1,0), Vec2i::new(1,1)];
            out[1]=[rotar_vec2i_90_n_veces(out[0][0],1),rotar_vec2i_90_n_veces(out[0][1],1),rotar_vec2i_90_n_veces(out[0][2],1),rotar_vec2i_90_n_veces(out[0][3],1)];
            out[2]=[rotar_vec2i_90_n_veces(out[0][0],2),rotar_vec2i_90_n_veces(out[0][1],2),rotar_vec2i_90_n_veces(out[0][2],2),rotar_vec2i_90_n_veces(out[0][3],2)];
            out[3]=[rotar_vec2i_90_n_veces(out[0][0],3),rotar_vec2i_90_n_veces(out[0][1],3),rotar_vec2i_90_n_veces(out[0][2],3),rotar_vec2i_90_n_veces(out[0][3],3)];
            out
        },
        {
            let mut out:[[Vec2i;4];4]=[[Vec2i::CERO;4];4];
            out[0]=[Vec2i::new(-1,1), Vec2i::new(-1,0), Vec2i::new(0,0), Vec2i::new(1,0)];
            out[1]=[rotar_vec2i_90_n_veces(out[0][0],1),rotar_vec2i_90_n_veces(out[0][1],1),rotar_vec2i_90_n_veces(out[0][2],1),rotar_vec2i_90_n_veces(out[0][3],1)];
            out[2]=[rotar_vec2i_90_n_veces(out[0][0],2),rotar_vec2i_90_n_veces(out[0][1],2),rotar_vec2i_90_n_veces(out[0][2],2),rotar_vec2i_90_n_veces(out[0][3],2)];
            out[3]=[rotar_vec2i_90_n_veces(out[0][0],3),rotar_vec2i_90_n_veces(out[0][1],3),rotar_vec2i_90_n_veces(out[0][2],3),rotar_vec2i_90_n_veces(out[0][3],3)];
            out
        },
        {
            let mut out:[[Vec2i;4];4]=[[Vec2i::CERO;4];4];
            out[0]=[Vec2i::new(-1,0), Vec2i::new(0,0), Vec2i::new(0,1), Vec2i::new(1,0)];
            out[1]=[rotar_vec2i_90_n_veces(out[0][0],1),rotar_vec2i_90_n_veces(out[0][1],1),rotar_vec2i_90_n_veces(out[0][2],1),rotar_vec2i_90_n_veces(out[0][3],1)];
            out[2]=[rotar_vec2i_90_n_veces(out[0][0],2),rotar_vec2i_90_n_veces(out[0][1],2),rotar_vec2i_90_n_veces(out[0][2],2),rotar_vec2i_90_n_veces(out[0][3],2)];
            out[3]=[rotar_vec2i_90_n_veces(out[0][0],3),rotar_vec2i_90_n_veces(out[0][1],3),rotar_vec2i_90_n_veces(out[0][2],3),rotar_vec2i_90_n_veces(out[0][3],3)];
            out
        },
        {
            let mut out:[[Vec2i;4];4]=[[Vec2i::CERO;4];4];
            out[0]=[Vec2i::new(0,1), Vec2i::new(1,1), Vec2i::new(1,0), Vec2i::new(0,0)];
            out[1]=[rotar_vec2i_90_n_veces(out[0][0],1),rotar_vec2i_90_n_veces(out[0][1],1),rotar_vec2i_90_n_veces(out[0][2],1),rotar_vec2i_90_n_veces(out[0][3],1)];
            out[2]=[rotar_vec2i_90_n_veces(out[0][0],2),rotar_vec2i_90_n_veces(out[0][1],2),rotar_vec2i_90_n_veces(out[0][2],2),rotar_vec2i_90_n_veces(out[0][3],2)];
            out[3]=[rotar_vec2i_90_n_veces(out[0][0],3),rotar_vec2i_90_n_veces(out[0][1],3),rotar_vec2i_90_n_veces(out[0][2],3),rotar_vec2i_90_n_veces(out[0][3],3)];
            out
        },
        {
            let mut out:[[Vec2i;4];4]=[[Vec2i::CERO;4];4];
            out[0]=[Vec2i::new(-1,0), Vec2i::new(0,0), Vec2i::new(1,0), Vec2i::new(2,0)];
            out[1]=[rotar_vec2i_90_n_veces(out[0][0],1),rotar_vec2i_90_n_veces(out[0][1],1),rotar_vec2i_90_n_veces(out[0][2],1),rotar_vec2i_90_n_veces(out[0][3],1)];
            out[2]=[rotar_vec2i_90_n_veces(out[0][0],2),rotar_vec2i_90_n_veces(out[0][1],2),rotar_vec2i_90_n_veces(out[0][2],2),rotar_vec2i_90_n_veces(out[0][3],2)];
            out[3]=[rotar_vec2i_90_n_veces(out[0][0],3),rotar_vec2i_90_n_veces(out[0][1],3),rotar_vec2i_90_n_veces(out[0][2],3),rotar_vec2i_90_n_veces(out[0][3],3)];
            out
        }
    ];  //uso PIEZAS[tipo][rotacion][tile]

    pub fn new(t_pieza:u8)->Pieza{
        Pieza { 
            t_pieza, 
            rotacion: 0, 
            pos: Vec2i{x:4,y:18} 
        }
    }

    pub fn out_piezas(){
        let tam = 7;
        let piezas = 7;
        for _ in 0..piezas as i64*tam*2+piezas as i64+1{
            print!("-");
        }
        print!("\n");
        for rotacion in 0..4 {
            for i in (0..tam).rev() {
                print!("|");
                for pieza in 0..piezas {
                    for j in 0..tam {
                        let mut hay:bool = false;
                        for v in 0..4 {
                            if Pieza::PIEZAS[pieza][rotacion][v].x==j-(tam-tam%2)/2 && Pieza::PIEZAS[pieza][rotacion][v].y==i-(tam-tam%2)/2 {
                                hay=true;
                            }
                        }
                        if hay {
                            print!("[{}",pieza);
                        } else {
                            print!(" .");
                        }
                    }
                    print!("|");
                }
                print!("\n");
            }
            for _ in 0..piezas as i64*tam*2+piezas as i64+1{
                print!("-");
            }
            print!("\n");
        }
    }
}