// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase


struct Customer {
    age: i32,
    name: String,
}

fn purchase_approved(cust: &Customer) -> Result<String, String> {
    if cust.age > 21 {
        Ok("cutomer approved !".to_owned())
    } else {
        Err("cutomer resiticted ! customer age is below 21".to_owned())
    }
}

fn main() {
    let customer1: Customer = Customer {age: 33, name: "chris".to_owned()};
    let cust_res = purchase_approved(&customer1);

    println!("{:?}",cust_res)
}
