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
        let seats_can_afford = most_seats_affordable(budget, booking_service_open);
        println!(
            "With {budget} you can afford {seats_can_afford} seats at {}",
            booking_price(seats_can_afford, booking_service_open)
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

fn most_seats_affordable(budget_amount: i32, booking_service_open: bool) -> i32 {
    if !booking_service_open || budget_amount <= 0 {
        return 0;
    }
    let mut seats = 0;
    while seats < 8 && booking_price(seats + 1, booking_service_open) <= budget_amount {
        seats += 1;
    }
    seats
}
