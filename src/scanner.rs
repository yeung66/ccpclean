use std::collections::HashMap;
use sysinfo::System;
use crate::process_info::ProcessInfo;
use crate::filter::is_dev_runtime;

#[cfg(not(target_os = "macos"))]
fn build_port_map() -> HashMap<u32, Vec<u16>> {
    use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

    let mut map: HashMap<u32, Vec<u16>> = HashMap::new();

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;

    if let Ok(sockets) = get_sockets_info(af_flags, proto_flags) {
        for si in sockets {
            if let ProtocolSocketInfo::Tcp(tcp) = si.protocol_socket_info {
                use netstat2::TcpState;
                if tcp.state == TcpState::Listen {
                    let port = tcp.local_port;
                    for pid in &si.associated_pids {
                        map.entry(*pid).or_default().push(port);
                    }
                }
            }
        }
    }

    map
}

#[cfg(target_os = "macos")]
fn build_port_map() -> HashMap<u32, Vec<u16>> {
    use std::process::Command;

    let mut map: HashMap<u32, Vec<u16>> = HashMap::new();

    let output = Command::new("lsof")
        .args(["-iTCP", "-sTCP:LISTEN", "-nP", "-F", "pn"])
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut current_pid: Option<u32> = None;

        for line in stdout.lines() {
            if let Some(pid_str) = line.strip_prefix('p') {
                current_pid = pid_str.parse().ok();
            } else if let Some(name) = line.strip_prefix('n') {
                if let Some(pid) = current_pid {
                    if let Some(port_str) = name.rsplit(':').next() {
                        if let Ok(port) = port_str.parse::<u16>() {
                            map.entry(pid).or_default().push(port);
                        }
                    }
                }
            }
        }
    }

    map
}

pub fn scan() -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let port_map = build_port_map();

    let mut results = Vec::new();

    for (pid, process) in sys.processes() {
        let pid_u32 = pid.as_u32();
        let ports = port_map.get(&pid_u32).cloned().unwrap_or_default();

        let name = process.name().to_string_lossy().to_string();
        let name = name.trim_end_matches(".exe").to_string();

        let cmd: Vec<String> = process
            .cmd()
            .iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect();

        let parent_pid = process.parent().map(|p| p.as_u32());
        let parent_name = parent_pid.and_then(|ppid| {
            sys.process(sysinfo::Pid::from_u32(ppid))
                .map(|p| {
                    p.name()
                        .to_string_lossy()
                        .trim_end_matches(".exe")
                        .to_string()
                })
        });

        let is_dev = is_dev_runtime(&name);

        let info = ProcessInfo {
            pid: pid_u32,
            name,
            cmd,
            ports,
            start_time_secs: process.start_time(),
            memory_kb: process.memory() / 1024,
            parent_pid,
            parent_name,
            is_dev_runtime: is_dev,
            score: 0,
        };

        results.push(info);
    }

    results
}
