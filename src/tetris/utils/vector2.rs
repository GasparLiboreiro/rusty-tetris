#[derive(Copy, Clone)]
pub struct Vec2i{
    pub x:i64,
    pub y:i64
}

impl Vec2i{
    pub const CERO:Vec2i = Vec2i{x:0,y:0};
    pub fn len(&self)->f64{
        let x = self.x as f64;
        let y = self.y as f64;
        f64::sqrt(x*x+y*y)
    }
    pub fn to_string(&self)->String{
        "(".to_string()+&self.x.to_string()+&";".to_string()+&self.y.to_string()+&")".to_string()
    }
    pub const fn new(x:i64, y:i64) -> Vec2i{
        Vec2i{x,y}
    }
}