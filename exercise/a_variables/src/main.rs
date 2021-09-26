
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    //let mut missiles = STARTING_MISSILES;
    //let ready = READY_AMOUNT;
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    let _notused = "notUsed";

    //READY_AMOUNT = 33;
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;

    println!("{} missiles left", missiles);
}
