// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock


use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    let mut total_stock = 0;

    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table", 2);
    stock.insert("Couch", 0);

    for (item, count) in stock.iter() {
        total_stock = total_stock + count;
        match count {
            0 =>  println!("{:?} -> out of stock", item),
            _ =>  println!("{:?} -> {:?}", item, count),
        }
    }
    
    println!("Total stock -> {:?}", total_stock);

}
