use crossbeam::channel::{Receiver, Sender};
use daemon_lib::ServerSocketMessage;
use std::io::Read;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::thread;

use anyhow::Result;
pub struct UdsServer {
    pub daemon_recv: Receiver<ServerSocketMessage>,
    pub daemon_send: Sender<ServerSocketMessage>,
    pub path: PathBuf,
}

impl UdsServer {
    pub fn new(
        daemon_recv: Receiver<ServerSocketMessage>,
        daemon_send: Sender<ServerSocketMessage>,
        socket_path: &Path,
    ) -> Self {
        Self {
            daemon_recv,
            daemon_send,
            path: socket_path.into(),
        }
    }

    pub fn run(&self) -> Result<()> {
        let listener = UnixListener::bind(&self.path)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let recv = self.daemon_recv.clone();
                    let send = self.daemon_send.clone();
                    thread::spawn(|| UdsServer::handle_client(stream, recv, send));
                }
                Err(err) => {}
            }
        }
        Ok(())
    }

    fn handle_client(
        mut stream: UnixStream,
        daemon_recv: Receiver<ServerSocketMessage>,
        daemon_send: Sender<ServerSocketMessage>,
    ) {
        let mut buf = [0u8; 8 * 1024];
        let amount_read = stream.read(&mut buf).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
