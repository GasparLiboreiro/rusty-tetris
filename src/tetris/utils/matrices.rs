use super::vector2::Vec2i;

const MATRIZ_ROTACION_90:[[i8;2];2] = [
    [0,1],
    [-1,0]
];


pub const fn rotar_vec2i_90_n_veces(mut vec:Vec2i, n:i8)->Vec2i{
    let matriz = &MATRIZ_ROTACION_90;
    if n<1{
        vec = Vec2i::new(matriz[0][0] as i64*vec.x+matriz[1][0] as i64*vec.y,matriz[0][1] as i64*vec.x+matriz[1][1] as i64*vec.y);
    }
    if n<2{
        vec = Vec2i::new(matriz[0][0] as i64*vec.x+matriz[1][0] as i64*vec.y,matriz[0][1] as i64*vec.x+matriz[1][1] as i64*vec.y);
    }
    if n<3{
        vec = Vec2i::new(matriz[0][0] as i64*vec.x+matriz[1][0] as i64*vec.y,matriz[0][1] as i64*vec.x+matriz[1][1] as i64*vec.y);
    }
    if n<4{
        vec = Vec2i::new(matriz[0][0] as i64*vec.x+matriz[1][0] as i64*vec.y,matriz[0][1] as i64*vec.x+matriz[1][1] as i64*vec.y);
    }
    vec
}