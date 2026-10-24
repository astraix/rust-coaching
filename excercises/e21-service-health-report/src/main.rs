struct Service {
    name: String,
    reachable: bool,
    response_time_ms: u32,
}

impl Service {
    fn ready(&self, response_time_threshold_ms: u32) -> bool {
        self.reachable && self.response_time_ms <= response_time_threshold_ms
    }
    fn print_report(&self, response_time_threshold_ms: u32) {
        println!(
            "Service {} is ready [{}] with a response time of {} ms",
            self.name,
            self.ready(response_time_threshold_ms),
            self.response_time_ms
        );
    }
}

fn main() {
    let mut services = vec![
        Service {
            name: String::from("Gumbo"),
            reachable: true,
            response_time_ms: 10,
        },
        Service {
            name: String::from("Jimbo"),
            reachable: false,
            response_time_ms: 20,
        },
        Service {
            name: String::from("Bembo"),
            reachable: true,
            response_time_ms: 5000,
        },
    ];

    services.push(Service {
        name: String::from("Yombo"),
        reachable: true,
        response_time_ms: 200,
    });

    print_services_report(&services, 200);
    print_services_report(&services, 5100);
}

fn print_services_report(services: &Vec<Service>, response_time_threshold_ms: u32) {
    println!("---- Health Service Report with response time {response_time_threshold_ms} ----");
    let mut services_ready: u32 = 0;
    for service in services {
        service.print_report(response_time_threshold_ms);
        if service.ready(response_time_threshold_ms) {
            services_ready += 1;
        }
    }
    println!("-- Total services scanned: {}", services.len());
    println!("-- Total services READY: {services_ready}");
    println!("---- End of Report ---")
}
