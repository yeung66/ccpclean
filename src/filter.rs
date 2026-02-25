use crate::process_info::ProcessInfo;

const DEV_RUNTIMES: &[&str] = &[
    "node", "nodejs", "python", "python3", "python3.11", "python3.12",
    "deno", "bun", "ruby", "java", "gradle", "mvn",
];

const DEV_PARENT_NAMES: &[&str] = &["claude", "bash", "zsh", "sh", "fish", "pwsh", "powershell"];

const DEV_CMD_KEYWORDS: &[&str] = &[
    "server", "serve", "dev", "run", "start", "manage", "app", "api",
    "http", "web", "watch",
];

pub fn is_dev_runtime(name: &str) -> bool {
    let lower = name.to_lowercase();
    DEV_RUNTIMES.iter().any(|&r| lower == r || lower.starts_with(r))
}

pub fn compute_score(p: &ProcessInfo) -> u8 {
    let mut score: u16 = 0;

    if p.is_dev_runtime {
        score += 30;
    }

    if p.ports.iter().any(|&port| port >= 1024 && port <= 9999) {
        score += 20;
    }

    let cmd_str = p.cmd.join(" ").to_lowercase();
    if DEV_CMD_KEYWORDS.iter().any(|&kw| cmd_str.contains(kw)) {
        score += 20;
    }

    if let Some(ref parent) = p.parent_name {
        let parent_lower = parent.to_lowercase();
        if DEV_PARENT_NAMES.iter().any(|&pn| parent_lower.contains(pn)) {
            score += 20;
        }
    }

    if p.uptime().as_secs() > 1800 {
        score += 10;
    }

    score.min(100) as u8
}

pub fn score_display(score: u8) -> String {
    let filled = (score as usize * 5 / 100).min(5);
    let empty = 5 - filled;
    format!("{}{}", "●".repeat(filled), "○".repeat(empty))
}

#[derive(Clone, Copy, PartialEq)]
pub enum FilterMode {
    Strict,
    Loose,
}

pub fn apply_filter(processes: Vec<ProcessInfo>, mode: FilterMode) -> Vec<ProcessInfo> {
    processes
        .into_iter()
        .filter(|p| {
            if p.ports.is_empty() {
                return false;
            }
            match mode {
                FilterMode::Strict => p.is_dev_runtime,
                FilterMode::Loose => true,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process_info::ProcessInfo;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn make_proc(name: &str, ports: Vec<u16>, cmd: Vec<&str>, parent: Option<&str>) -> ProcessInfo {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let is_dev = is_dev_runtime(name);
        ProcessInfo {
            pid: 1,
            name: name.to_string(),
            cmd: cmd.iter().map(|s| s.to_string()).collect(),
            ports,
            start_time_secs: now - 3600,
            memory_kb: 1024,
            parent_pid: None,
            parent_name: parent.map(|s| s.to_string()),
            is_dev_runtime: is_dev,
            score: 0,
        }
    }

    #[test]
    fn test_is_dev_runtime_node() {
        assert!(is_dev_runtime("node"));
        assert!(is_dev_runtime("Node"));
        assert!(is_dev_runtime("python3"));
        assert!(!is_dev_runtime("nginx"));
        assert!(!is_dev_runtime("postgres"));
    }

    #[test]
    fn test_score_dev_runtime_with_port() {
        let mut p = make_proc("node", vec![3000], vec!["node", "server.js"], Some("bash"));
        p.score = compute_score(&p);
        assert_eq!(p.score, 100);
    }

    #[test]
    fn test_score_no_dev_runtime() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut p = make_proc("nginx", vec![80], vec!["nginx"], None);
        // Set start_time to recent so uptime bonus doesn't apply
        p.start_time_secs = now - 60;
        p.score = compute_score(&p);
        assert_eq!(p.score, 0);
    }

    #[test]
    fn test_score_python_dev_port() {
        let mut p = make_proc("python3", vec![8000], vec!["python3", "manage.py", "runserver"], None);
        p.score = compute_score(&p);
        assert_eq!(p.score, 80);
    }

    #[test]
    fn test_score_display() {
        assert_eq!(score_display(100), "●●●●●");
        assert_eq!(score_display(0), "○○○○○");
        assert_eq!(score_display(60), "●●●○○");
    }

    #[test]
    fn test_filter_strict_excludes_nginx() {
        let nginx = make_proc("nginx", vec![8080], vec!["nginx"], None);
        let node = make_proc("node", vec![3000], vec!["node", "app.js"], None);
        let result = apply_filter(vec![nginx, node], FilterMode::Strict);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "node");
    }

    #[test]
    fn test_filter_loose_includes_all_with_ports() {
        let nginx = make_proc("nginx", vec![8080], vec!["nginx"], None);
        let node = make_proc("node", vec![3000], vec!["node", "app.js"], None);
        let no_port = make_proc("python3", vec![], vec!["python3", "script.py"], None);
        let result = apply_filter(vec![nginx, node, no_port], FilterMode::Loose);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_filter_excludes_no_port_processes() {
        let p = make_proc("node", vec![], vec!["node", "worker.js"], None);
        let result = apply_filter(vec![p], FilterMode::Strict);
        assert!(result.is_empty());
    }
}
