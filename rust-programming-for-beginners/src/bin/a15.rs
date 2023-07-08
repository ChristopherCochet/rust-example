// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

#[derive(Debug)]
enum Ticket {
    Backstage(String, f64), 
    Vip(String, f64),
    Standard(f64),
} 

fn main() {

    let t1:Ticket = Ticket::Backstage("Chris".to_owned(),47.0);
    let t2:Ticket = Ticket::Vip("Lauren".to_owned(),45.0);
    let t3:Ticket = Ticket::Standard(14.0);

    let myvec = vec![t1,t2,t3];

    for v in myvec {
        match v {
            Ticket::Backstage(name, price) => println!("this ticket -> Ticket.Backstage {:?} {:?}", name, price),
            Ticket::Vip(name, price) => println!("this ticket-> Ticket.Vip {:?} {:?}", name, price),
            Ticket::Standard(price) => println!("this ticket -> Ticket.Standard {:?}", price),
        }
    }
}
