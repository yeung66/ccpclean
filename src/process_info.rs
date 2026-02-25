use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cmd: Vec<String>,
    pub ports: Vec<u16>,
    pub start_time_secs: u64,
    pub memory_kb: u64,
    pub parent_pid: Option<u32>,
    pub parent_name: Option<String>,
    pub is_dev_runtime: bool,
    pub score: u8,
}

impl ProcessInfo {
    pub fn uptime(&self) -> Duration {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let elapsed = now.saturating_sub(self.start_time_secs);
        Duration::from_secs(elapsed)
    }

    pub fn uptime_display(&self) -> String {
        let secs = self.uptime().as_secs();
        if secs < 60 {
            format!("{}s ago", secs)
        } else if secs < 3600 {
            format!("{}m ago", secs / 60)
        } else {
            format!("{}h {}m ago", secs / 3600, (secs % 3600) / 60)
        }
    }

    pub fn memory_display(&self) -> String {
        if self.memory_kb < 1024 {
            format!("{} KB", self.memory_kb)
        } else {
            format!("{:.1} MB", self.memory_kb as f64 / 1024.0)
        }
    }

    pub fn ports_display(&self) -> String {
        self.ports
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_process() -> ProcessInfo {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        ProcessInfo {
            pid: 1234,
            name: "node".to_string(),
            cmd: vec!["node".to_string(), "server.js".to_string()],
            ports: vec![3000, 3001],
            start_time_secs: now - 7380,
            memory_kb: 89600,
            parent_pid: Some(999),
            parent_name: Some("bash".to_string()),
            is_dev_runtime: true,
            score: 90,
        }
    }

    #[test]
    fn test_uptime_display_hours() {
        let p = make_process();
        let display = p.uptime_display();
        assert!(display.contains("h"), "expected hours: {}", display);
    }

    #[test]
    fn test_memory_display_mb() {
        let p = make_process();
        assert_eq!(p.memory_display(), "87.5 MB");
    }

    #[test]
    fn test_ports_display() {
        let p = make_process();
        assert_eq!(p.ports_display(), "3000, 3001");
    }

    #[test]
    fn test_ports_display_empty() {
        let mut p = make_process();
        p.ports = vec![];
        assert_eq!(p.ports_display(), "");
    }
}
