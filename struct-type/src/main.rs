#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main() {
    let  rect1 = Rectangle{
        width:20,
        height:10
    };
    println!("rect1.area is {}", rect1.area());

}
