use std::path::PathBuf;

use alumina_lib::*;
use anyhow::Result;
use crossbeam::channel;
use daemon_lib::*;
use login_stub::LoginStub;
use tokio_uds_server::UdsServer;

fn main() -> Result<()> {
    // TODO: We probably want to add a clap interface
    // and maybe integrate some logger but this is fine
    // for now

    let (socket_send, daemon_recv) = channel::unbounded();
    let (daemon_send, socket_recv) = channel::unbounded();

    let daemon = Daemon::new(socket_send, socket_recv, Box::new(LoginStub {}));
    let socket_server = UdsServer::new(
        daemon_recv,
        daemon_send,
        &PathBuf::from("/tmp/AluminaDM.socket"),
    );

    crossbeam::thread::scope(|s| {
        s.spawn(move |_| -> Result<()> {
            socket_server.run()?;
            Ok(())
        });
    })
    .unwrap(); //anyhow doesn't support this

    daemon.run()?;
    Ok(())
}

//struct DaemonApp {}
