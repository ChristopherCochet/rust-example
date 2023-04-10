// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    color: String,
}

fn main() {

    let p1 = Person {age: 47, name: "chris".to_owned(), color: "red".to_owned()};
    let p2 = Person {age: 44, name: "lauren".to_owned(), color: "blue".to_owned()};
    let p3 = Person {age: 5, name: "saxby".to_owned(), color: "sun".to_owned()};
    let mut myvec = Vec::new();
    myvec.push(p1);
    myvec.push(p2);
    myvec.push(p3);

    for p in myvec {
        println!("age ->{:?},  name -> {:?},  color -> {:?},", p.age, p.name, p.color)
    }
    


}
