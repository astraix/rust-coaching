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
    println!("-- Total services scanned: {}", services.len());
    println!("-- Total services READY: {services_ready}");
    println!("---- End of Report ---")
}
