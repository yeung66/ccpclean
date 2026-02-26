pub mod list_view;
pub mod detail_view;
pub mod runner;

use crate::process_info::ProcessInfo;
use crate::filter::{apply_filter, FilterMode};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum View {
    List,
    Detail,
}

pub struct AppState {
    pub all_processes: Vec<ProcessInfo>,
    pub processes: Vec<ProcessInfo>,
    pub selected_index: usize,
    pub checked: Vec<bool>,
    pub view: View,
    pub filter_mode: FilterMode,
    pub status_message: Option<String>,
    pub should_quit: bool,
}

impl AppState {
    pub fn new(processes: Vec<ProcessInfo>) -> Self {
        let len = processes.len();
        Self {
            all_processes: processes.clone(),
            processes,
            selected_index: 0,
            checked: vec![false; len],
            view: View::List,
            filter_mode: FilterMode::Strict,
            status_message: None,
            should_quit: false,
        }
    }

    pub fn refilter(&mut self) {
        self.processes = apply_filter(self.all_processes.clone(), self.filter_mode);
        self.checked = vec![false; self.processes.len()];
        self.selected_index = self.selected_index.min(self.processes.len().saturating_sub(1));
    }

    pub fn remove_processes(&mut self, pids: &[u32]) {
        self.processes.retain(|p| !pids.contains(&p.pid));
        self.all_processes.retain(|p| !pids.contains(&p.pid));
        self.checked = vec![false; self.processes.len()];
        self.selected_index = self.selected_index.min(self.processes.len().saturating_sub(1));
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index + 1 < self.processes.len() {
            self.selected_index += 1;
        }
    }

    pub fn toggle_checked(&mut self) {
        if let Some(v) = self.checked.get_mut(self.selected_index) {
            *v = !*v;
        }
    }

    pub fn select_all(&mut self) {
        let all_checked = self.checked.iter().all(|&c| c);
        let target = !all_checked;
        self.checked.iter_mut().for_each(|c| *c = target);
    }

    pub fn checked_pids(&self) -> Vec<u32> {
        self.processes
            .iter()
            .zip(self.checked.iter())
            .filter(|(_, &checked)| checked)
            .map(|(p, _)| p.pid)
            .collect()
    }

    pub fn current_process(&self) -> Option<&ProcessInfo> {
        self.processes.get(self.selected_index)
    }

    pub fn switch_view(&mut self) {
        self.view = match self.view {
            View::List => View::Detail,
            View::Detail => View::List,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process_info::ProcessInfo;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn make_app() -> AppState {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let processes = vec![
            ProcessInfo { pid: 1, name: "node".to_string(), cmd: vec![], ports: vec![3000], start_time_secs: now, memory_kb: 0, parent_pid: None, parent_name: None, is_dev_runtime: true, score: 80 },
            ProcessInfo { pid: 2, name: "python".to_string(), cmd: vec![], ports: vec![8000], start_time_secs: now, memory_kb: 0, parent_pid: None, parent_name: None, is_dev_runtime: true, score: 60 },
        ];
        AppState::new(processes)
    }

    #[test]
    fn test_move_up_clamped() {
        let mut app = make_app();
        app.move_up();
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_move_down_then_up() {
        let mut app = make_app();
        app.move_down();
        assert_eq!(app.selected_index, 1);
        app.move_down();
        assert_eq!(app.selected_index, 1);
        app.move_up();
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_toggle_checked() {
        let mut app = make_app();
        assert!(!app.checked[0]);
        app.toggle_checked();
        assert!(app.checked[0]);
        app.toggle_checked();
        assert!(!app.checked[0]);
    }

    #[test]
    fn test_select_all_and_deselect() {
        let mut app = make_app();
        app.select_all();
        assert!(app.checked.iter().all(|&c| c));
        app.select_all();
        assert!(app.checked.iter().all(|&c| !c));
    }

    #[test]
    fn test_checked_pids() {
        let mut app = make_app();
        app.checked[0] = true;
        assert_eq!(app.checked_pids(), vec![1]);
    }

    #[test]
    fn test_switch_view() {
        let mut app = make_app();
        assert_eq!(app.view, View::List);
        app.switch_view();
        assert_eq!(app.view, View::Detail);
        app.switch_view();
        assert_eq!(app.view, View::List);
    }
}
