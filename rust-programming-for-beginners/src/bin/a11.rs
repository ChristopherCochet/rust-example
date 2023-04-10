// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_quantity(g_item: &GroceryItem) {
    println!("quantity is {}", g_item.quantity);
}

fn display_id(g_item: &GroceryItem) {
    println!("id is {}", g_item.id);
}

fn main() {
    let my_item : GroceryItem = GroceryItem {
        quantity: 10,
        id: 123456,
    } ;

    display_quantity(&my_item);
    display_id(&my_item);

}
