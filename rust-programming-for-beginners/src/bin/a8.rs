// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Sweet,
    Sour,
}


struct Drink {
    flavor: Flavor,
    ounces: i32,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sweet => println!("my drink -> Sweet {}", drink.ounces),
        Flavor::Sour => println!("my drink -> Sour {}", drink.ounces),
    }

}

fn main() {
    let example_drink = Drink {
        flavor: Flavor::Sweet,
        ounces: 4,
    };

    print_drink(example_drink);
}


