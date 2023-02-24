use anyhow::Result;
use crossbeam::channel::{Receiver, Sender};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

pub enum ServerSocketMessage {
    Initialize,
}

pub trait LoginHandler {}

pub struct Daemon {
    pub socket_send_channel: Sender<ServerSocketMessage>,
    pub socket_recv_channel: Receiver<ServerSocketMessage>,
    pub login_handler: Box<dyn LoginHandler>,
}

impl Daemon {
    pub fn new(
        socket_send_channel: Sender<ServerSocketMessage>,
        socket_recv_channel: Receiver<ServerSocketMessage>,
        login_handler: Box<dyn LoginHandler>,
    ) -> Self {
        Self {
            socket_send_channel,
            socket_recv_channel,
            login_handler,
        }
    }

    pub fn run(&self) -> Result<()> {
        loop {
            let message = self.socket_recv_channel.recv()?;
            match message {
                ServerSocketMessage::Initialize => todo!(),
            }
        }
    }
}
