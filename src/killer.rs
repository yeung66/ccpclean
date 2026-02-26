use std::io;

#[derive(Debug)]
pub enum KillError {
    PermissionDenied(u32),
    ProcessNotFound(u32),
    Other(u32, io::Error),
}

impl std::fmt::Display for KillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KillError::PermissionDenied(pid) => {
                write!(f, "Permission denied killing PID {} (try running as admin)", pid)
            }
            KillError::ProcessNotFound(pid) => {
                write!(f, "Process {} not found (already exited?)", pid)
            }
            KillError::Other(pid, e) => write!(f, "Failed to kill PID {}: {}", pid, e),
        }
    }
}

pub fn kill(pid: u32) -> Result<(), KillError> {
    use sysinfo::{Pid, Signal, System};
    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    let sysinfo_pid = Pid::from_u32(pid);
    match sys.process(sysinfo_pid) {
        None => Err(KillError::ProcessNotFound(pid)),
        Some(process) => {
            match process.kill_with(Signal::Term) {
                Some(true) => Ok(()),
                Some(false) => Err(KillError::PermissionDenied(pid)),
                None => {
                    // Signal not supported on this platform, use SIGKILL
                    if process.kill() {
                        Ok(())
                    } else {
                        Err(KillError::PermissionDenied(pid))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kill_nonexistent_pid() {
        let result = kill(99999999);
        assert!(matches!(result, Err(KillError::ProcessNotFound(_))));
    }

    #[test]
    fn test_kill_error_display() {
        let e = KillError::PermissionDenied(1234);
        assert!(e.to_string().contains("1234"));
        assert!(e.to_string().contains("admin"));
    }
}
