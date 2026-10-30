mod health;

use crate::health::{Service, compute_service_status};

fn main() {
    let services = vec![
        Service::new(String::from("Gumbo"), true, 10),
        Service::new(String::from("Jimbo"), false, 20),
        Service::new(String::from("Rombo"), false, 200),
        Service::new(String::from("Bembo"), true, 5000),
        Service::new(String::from("Bembo"), true, 6000),
    ];

    let services_status_5000 = compute_service_status(&services, 5000);
    println!(
        "Bembo status is {}",
        services_status_5000.get("Bembo").unwrap()
    );

    let services_status_7000 = compute_service_status(&services, 7000);
    println!(
        "Bembo status is {}",
        services_status_7000.get("Bembo").unwrap()
    );
    println!(
        "Rombo status is {}",
        services_status_7000.get("Rombo").unwrap()
    );

    match services_status_7000.get("Unknown") {
        Some(service_status) => println!("Found Unknown with status {}", service_status),
        None => println!("Unknown not found"),
    }

    let empty_service_status = compute_service_status(&[], 1000);
    match empty_service_status.get("Gumbo") {
        Some(service_status) => println!("Found Gumbo with status {}", service_status),
        None => println!("Gumbo not found"),
    }
}
