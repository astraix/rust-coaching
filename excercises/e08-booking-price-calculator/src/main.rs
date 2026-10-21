fn main() {
    let booking_service_open = true;
    println!(
        "Booking service open: {booking_service_open}. Here the seats price list from 1 to 8 seats."
    );
    for seats in 1..=8 {
        let seats_price = booking_price(seats, booking_service_open);
        println!(
            "Seats: {seats} - Booking service open: {booking_service_open} -> Seats price: {seats_price}"
        );
    }

    if !booking_service_open {
        println!("Unavailable");
    } else {
        let budget = 95;
        let customer = String::from("Jimbo Bimbo");
        customer_confirmation(&customer);
        customer_confirmation("Wambo Jambo");
        let customer_budget = budget;
        let (seats_can_afford, seats_price) = most_seats_affordable(budget, booking_service_open);
        println!(
            "With {customer_budget} {customer} can afford {seats_can_afford} seats at {seats_price}"
        );
    }
}

fn booking_price(seats: i32, booking_service_open: bool) -> i32 {
    if !booking_service_open || seats <= 0 {
        return 0;
    }
    match seats {
        1 => 15,
        2 => 28,
        _ => seats * 12,
    }
}

fn most_seats_affordable(budget_amount: i32, booking_service_open: bool) -> (i32, i32) {
    if !booking_service_open || budget_amount <= 0 {
        return (0, 0);
    }
    let mut seats = 0;
    while seats < 8 && booking_price(seats + 1, booking_service_open) <= budget_amount {
        seats += 1;
    }
    (seats, booking_price(seats, booking_service_open))
}

fn customer_confirmation(customer_name: &str) {
    println!("{customer_name} CONFIRMED");
}
