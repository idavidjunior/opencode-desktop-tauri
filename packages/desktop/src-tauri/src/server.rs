use std::io::{BufRead, BufReader};
use std::net::{TcpStream, ToSocketAddrs};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::state::SidecarHandle;

pub fn find_free_port() -> Result<u16, String> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    let addr = listener.local_addr().map_err(|e| e.to_string())?;
    drop(listener);
    Ok(addr.port())
}

pub fn spawn_sidecar(
    port: u16,
    password: &str,
    server_dir: &str,
    node_modules: &str,
    data_dir: &str,
    stderr_log: &Arc<Mutex<Vec<String>>>,
) -> Result<Child, String> {
    let mut cmd = Command::new("node");
    cmd.arg("--experimental-sqlite");
    // ponytail: NODE_PATH for externalized deps (@lydell/node-pty). In production, bundle deps as resources.
    cmd.arg("sidecar-entry.mjs")
        .current_dir(server_dir)
        .env("NODE_PATH", node_modules)
        .env("OPENCODE_PORT", port.to_string())
        .env("OPENCODE_SERVER_USERNAME", "opencode")
        .env("OPENCODE_SERVER_PASSWORD", password)
        .env("OPENCODE_DISABLE_EMBEDDED_WEB_UI", "true")
        .env("OPENCODE_CLIENT", "desktop")
        .env("XDG_STATE_HOME", data_dir)
        .env("NO_PROXY", "127.0.0.1,localhost,::1")
        .env("no_proxy", "127.0.0.1,localhost,::1")
        .stdout(Stdio::null())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| format!("failed to spawn sidecar: {}", e))?;

    if let Some(stderr) = child.stderr.take() {
        let log = stderr_log.clone();
        std::thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(l) = line {
                    crate::store::lock(&log).push(l);
                }
            }
        });
    }

    Ok(child)
}

// ponytail: TCP port check — 200ms timeout keeps total poll ~30s while allowing slow starts
pub fn port_ready(host: &str, port: u16) -> bool {
    let addr = format!("{}:{}", host, port);
    if let Ok(mut addrs) = addr.to_socket_addrs() {
        if let Some(sockaddr) = addrs.next() {
            return TcpStream::connect_timeout(&sockaddr, Duration::from_millis(200)).is_ok();
        }
    }
    false
}

pub fn stop_sidecar(handle: &mut SidecarHandle) {
    let _ = handle.child.kill();
    let _ = handle.child.wait();
}
