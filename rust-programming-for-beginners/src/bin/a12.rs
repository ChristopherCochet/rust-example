// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics


#[derive(Debug)]
enum Color {
    Red,
    Blue,
    White
}

struct ShippingBox {
    dimensions: (i32,i32,i32),
    color: Color,
    weight:i32,
}

impl ShippingBox {
    fn print_caracteristics(&self) {
        println!("Shipping bix caracteristics: dimension -> {:?}, color -> {:?},  weight-> {:?}", self.dimensions, self.color, self.weight)
    }
}

fn main() {

    let my_box:ShippingBox = ShippingBox { dimensions: (2,3,4), color: Color::Blue, weight:64 };
    my_box.print_caracteristics();

}
