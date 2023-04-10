// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    red,
    blue,
    black,
}

fn main() {
    let my_col = Color::black;

    match my_col {
        Color::red => println!("it's red"),
        Color::blue => println!("it's blue"),
        Color::black => println!("it's black"),
    }

}
