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

enum ServiceError {
    Unnamed,
    ExcessiveLatency(u32),
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
    fn validate(&self) -> Result<(), ServiceError> {
        if self.name.is_empty() {
            Err(ServiceError::Unnamed)
        } else if self.reachable && self.response_time_ms > 60000 {
            Err(ServiceError::ExcessiveLatency(self.response_time_ms))
        } else {
            Ok(())
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

    match validate_batch(&services) {
        Ok(()) => {}
        Err(service_error) => {
            let error_message = match service_error {
                ServiceError::Unnamed => String::from("Service with no name"),
                ServiceError::ExcessiveLatency(latency) => {
                    format!("Service reachable with an excessive latency of {latency} ms")
                }
            };
            println!("Validation error: {error_message}");
            return;
        }
    }

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

    let readiness_check = check_readiness(services, response_time_threshold_ms);
    println!(
        "-- Readiness report: ready count {}, alerts: {:?}",
        readiness_check.0, readiness_check.1
    );
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

fn validate_batch(services: &[Service]) -> Result<(), ServiceError> {
    for service in services {
        service.validate()?
    }
    Ok(())
}

fn check_readiness(services: &[Service], response_time_threshold_ms: u32) -> (usize, Vec<String>) {
    let ready_count = services
        .iter()
        .map(|x| x.status(response_time_threshold_ms))
        .filter(|service_status| matches!(service_status, ServiceStatus::Ready))
        .count();

    let alerts = services
        .iter()
        .map(|s| (s, s.status(response_time_threshold_ms)))
        .filter_map(|(s, status)| match status {
            ServiceStatus::Ready => None,
            ServiceStatus::Slow(delay) => Some(format!(
                "Service {} is slow with a delay of {delay} ms",
                s.name
            )),
            ServiceStatus::Unreachable => Some(format!("Service {} is unreachable", s.name)),
        })
        .collect();

    (ready_count, alerts)
}
