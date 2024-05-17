 struct Fields {
        width: i32,
        height: i32,
    }
fn main() {
   let area_1 = Fields{
       width: 45,
       height: 56,
   };
   println!("The area of the rectangle is {}",area(area_1));
}

fn area(rectangle: Fields) -> i32 {
    rectangle.width * rectangle.height
}