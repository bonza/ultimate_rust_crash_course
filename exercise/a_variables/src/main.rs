const STARTING_MISSILES:i32 = 8;
const READY_AMOUNT:i32 = 2;

fn main() {
    // STARTING_MISSILES = 3;
    let (missiles, ready):(i32,i32) = (STARTING_MISSILES, READY_AMOUNT);
    let dummy = 5;

    println!("Firing {} of my {} missiles...", ready, missiles);

    // missiles = missiles - ready;

    println!("{} missiles left", missiles - ready);
}
