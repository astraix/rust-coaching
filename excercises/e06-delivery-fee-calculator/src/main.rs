fn main() {
    let order_amount = 12;
    let delivery_fee = calculate_delivery_fee(order_amount);
    let total_amount = order_amount + delivery_fee;
    println!(
        "Order amount: {order_amount} - Delivery fee: {delivery_fee} -> Total: {total_amount}"
    );
}

fn calculate_delivery_fee(order_amount: i32) -> i32 {
    if order_amount < 50 { 6 } else { 0 }
}
