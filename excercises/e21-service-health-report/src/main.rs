struct Service {
    name: String,
    reachable: bool,
    response_time_ms: u32,
}

enum ServiceStatus {
    Ready,
    Slow(u32),
    Unreachable,
}

impl Service {
    fn status(&self, response_time_threshold_ms: u32) -> ServiceStatus {
        if !self.reachable {
            ServiceStatus::Unreachable
        } else if self.response_time_ms > response_time_threshold_ms {
            ServiceStatus::Slow(self.response_time_ms)
        } else {
            ServiceStatus::Ready
        }
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
            name: String::from("Rombo"),
            reachable: false,
            response_time_ms: 200,
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
        response_time_ms: 0,
    });

    print_services_report(&services, 201);
    print_services_report(&services, 5100);
    print_services_report(&services[1..3], 5100);
    print_services_report(&services[..0], 0);
}

fn print_services_report(services: &[Service], response_time_threshold_ms: u32) {
    println!("---- Health Service Report with response time {response_time_threshold_ms} ----");
    let mut services_ready: u32 = 0;
    for service in services {
        let service_status = service.status(response_time_threshold_ms);
        let service_status_str = match service_status {
            ServiceStatus::Ready => String::from("ready"),
            ServiceStatus::Slow(latency) => format!("slow with latency {latency} and"),
            ServiceStatus::Unreachable => String::from("unreachable"),
        };
        if let ServiceStatus::Ready = service_status {
            services_ready += 1;
        }
        println!(
            "Service {} is {service_status_str} with a response time of {} ms",
            service.name, service.response_time_ms
        );
    }
    let lowest_response_time_str = match lowest_response_time(services) {
        Some(lowest_response_time) => format!("{lowest_response_time}"),
        None => String::from("no services available"),
    };
    println!("-- Total services scanned: {}", services.len());
    println!("-- Total services READY: {services_ready}");
    println!("-- Lowest response time: {lowest_response_time_str}");
    println!("---- End of Report ---")
}

fn lowest_response_time(services: &[Service]) -> Option<u32> {
    let mut lowest: Option<u32> = None;
    for service in services {
        if service.reachable && (lowest.is_none_or(|lw| lw > service.response_time_ms)) {
            lowest = Some(service.response_time_ms);
        }
    }
    lowest
}
