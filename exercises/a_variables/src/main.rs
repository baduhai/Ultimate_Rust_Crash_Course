const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // let mut missiles: i32 = STARTING_MISSILES;
    // let ready: i32 = READY_AMOUNT;
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);

    // let rockets = 7; // Warning: unused variable `rockets`

    // let READY_AMOUNT = 1; // error [E0005]: refutable pattern in local binding
    
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles -= ready;

    println!("{} missiles left", missiles);
    // println!("{} missiles left", missiles - ready);
}
