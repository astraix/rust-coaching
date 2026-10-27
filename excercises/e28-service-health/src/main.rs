use std::{collections::HashMap, fmt::Display};

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

impl Display for ServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceStatus::Ready => write!(f, "Ready"),
            ServiceStatus::Slow(delay) => write!(f, "Slow with a delay of {}", delay),
            ServiceStatus::Unreachable => write!(f, "Unreachable"),
        }
    }
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

fn compute_service_status(
    services: &[Service],
    response_time_threshold_ms: u32,
) -> HashMap<String, ServiceStatus> {
    let mut status_map: HashMap<String, ServiceStatus> = HashMap::new();
    for service in services {
        status_map.insert(
            service.name.clone(),
            service.status(response_time_threshold_ms),
        );
    }
    status_map
}

fn main() {
    let services = vec![
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
        Service {
            name: String::from("Bembo"),
            reachable: true,
            response_time_ms: 6000,
        },
    ];

    let services_status = compute_service_status(&services, 5000);
    println!("Bembo status is {}", services_status.get("Bembo").unwrap())
}
