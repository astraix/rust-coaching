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

pub fn service_status_report_text(service_map: &HashMap<String, ServiceStatus>) -> String {
    let mut report = String::new();
    for (service_name, status) in service_map {
        report.push_str(format!("{service_name}: {status}\n").as_str());
    }
    report
}

#[cfg(test)]
mod tests {

    use super::{Service, ServiceStatus, compute_service_status, service_status_report_text};

    #[test]
    fn service_status_reachable() {
        let service = Service::new(String::from("Yombo"), true, 200);
        assert!(matches!(service.status(100), ServiceStatus::Slow(_)));
        assert!(matches!(service.status(200), ServiceStatus::Ready));
        assert!(matches!(service.status(300), ServiceStatus::Ready));
    }

    #[test]
    fn service_status_unreachable_priority() {
        let service = Service::new(String::from("Yombo"), false, 400);
        assert!(matches!(service.status(300), ServiceStatus::Unreachable));
    }

    #[test]
    fn service_status_slow_carries_delay() {
        let service = Service::new(String::from("Yombo"), true, 200);
        assert!(matches!(service.status(100), ServiceStatus::Slow(delay) if delay == 200));
    }

    #[test]
    fn compute_service_status_tests() {
        let services = vec![
            Service::new(String::from("Gumbo"), true, 10),
            Service::new(String::from("Jimbo"), false, 20),
            Service::new(String::from("Jimbo"), true, 15),
        ];
        let service_status_map_20_ms = compute_service_status(&services, 20);

        assert!(matches!(service_status_map_20_ms.get("Oombo"), None));
        assert!(
            matches!(service_status_map_20_ms.get("Jimbo"), Some(status) if matches!(status, ServiceStatus::Ready))
        );

        let empty_service_status_map = compute_service_status(&[], 100);
        assert!(empty_service_status_map.is_empty());
    }

    #[test]
    fn service_status_report_text_test() {
        let services = vec![
            Service::new(String::from("Gumbo"), true, 10),
            Service::new(String::from("Wambo"), true, 30),
            Service::new(String::from("Dambo"), false, 5),
        ];
        let service_status_map = &compute_service_status(&services, 20);

        let text = service_status_report_text(service_status_map);
        assert!(text.contains("Gumbo: Ready"));
        assert!(text.contains("Wambo: Slow with a delay of 30 ms"));
        assert!(text.contains("Dambo: Unreachable"));
    }
}
