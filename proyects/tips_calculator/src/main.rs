use std::io;

fn main() {
    let mut bill = String::new();
    println!("Put the total of bill:");
    io::stdin().read_line(&mut bill).unwrap();

    let mut percent = String::new();
    println!("Put the percentage of the tip:");
    io::stdin().read_line(&mut percent).unwrap();

    let bill_int: f32 = bill.trim().parse().unwrap();
    let percent_int: f32 = percent.trim().parse().unwrap();

    let tip = bill_int * (percent_int / 100.0);
    let bill_total = bill_int + tip;

    println!(
        "The total of the bill is: {}$. {}$ Of propine",
        bill_total, tip
    )
}
