fn main() {
    let item_label = "NASDAQ";
    let mut item_qty: i64 = 13;
    println!("Stock {item_label} now at {item_qty}");
    item_qty += 22;
    println!("Stock {item_label} delivered! Now at {item_qty}");
    item_qty -= 5;
    println!("Stock {item_label} sold! Now at {item_qty}");
}
