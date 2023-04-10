// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print


fn main() {
    let coord = (1,2);
    print_y_vs_5(coord);
}

fn print_y_vs_5(tuple_xy:(i32,i32)) {
    if tuple_xy.1 > 5 {
        println!("y = {} is greater than 5",tuple_xy.1)
    } else if tuple_xy.1 < 5 {
        println!("y = {} is less than 5",tuple_xy.1)
    } else {
        println!("y = {} is equal to 5",tuple_xy.1)
    }
    
}
