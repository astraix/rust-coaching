use std::{collections::HashMap, fmt::Display};

pub struct Service {
    name: String,
    reachable: bool,
    response_time_ms: u32,
}

pub enum ServiceStatus {
    Ready,
    Slow(u32),
    Unreachable,
}

impl Display for ServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceStatus::Ready => write!(f, "Ready"),
            ServiceStatus::Slow(delay) => write!(f, "Slow with a delay of {} ms", delay),
            ServiceStatus::Unreachable => write!(f, "Unreachable"),
        }
    }
}

impl Service {
    pub fn new(name: String, reachable: bool, response_time_ms: u32) -> Self {
        Service {
            name,
            reachable,
            response_time_ms,
        }
    }
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

pub fn compute_service_status(
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
