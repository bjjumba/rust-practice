struct rectangle{
    width:f64,
    length:f64
}

impl rectangle{
    fn get_area(&self)->f64{
        self.width * self.length
    }

    //scale function
    fn scale(&mut self ,x:f64){
        self.width*=x;
        self.length*=x;
    }

    //our constructor
    fn new(x:f64,y:f64)->rectangle{
        rectangle{
            width:x,
            length:y
        }
    }
}

fn main() {
    let mut rect=rectangle::new(1.2,3.4);
    let area:f64=rect.get_area();
    println!("Area is {}",area);
    rect.scale(0.5);
    let area:f64=rect.get_area();
    println!("Area with scale is {}",area);
}
