use std::error::Error;
use std::rc::Rc;
use anyhow::anyhow;
use crate::connection::{Connection};

pub struct Client {
    connection: Rc<dyn Connection>,
}

impl Client {
    pub fn new(connection: Rc<dyn Connection>) -> Self {
        Self { connection }
    }

    pub fn connect(&self) -> Result<(), Box<dyn Error>> {
        self.connection.connect().map_err(|e| anyhow!("cannot connect: {}", e).into())
    }

    pub fn disconnect(&self) -> Result<(), Box<dyn Error>> {
        self.connection.disconnect().map_err(|e| anyhow!("cannot disconnect: {}", e).into())
    }

    pub fn send_message(&self, message: &str) -> Result<(), Box<dyn Error>> {
        self.connection.write(message).map_err(|e| anyhow!("cannot write message: {}", e).into())
    }

    pub fn read_message(&self) -> Result<String, Box<dyn Error>> {
        self.connection.read().map_err(|e| anyhow!("cannot read message: {}", e).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::connection::MockConnection;
    use mockall::predicate::*;

    #[test]
    fn it_can_connect() {
        let mut fake_connection = MockConnection::new();
        fake_connection
            .expect_connect()
            .returning(|| Ok(()));

        let client = Client::new(Rc::new(fake_connection));
        let ok = client.connect();
        assert!(ok.is_ok());
    }

    #[test]
    fn it_handles_connect_errors() {
        let mut fake_connection = MockConnection::new();
        fake_connection
            .expect_connect()
            .returning(|| Err("connect error".into()));

        let client = Client::new(Rc::new(fake_connection));
        let err = client.connect();
        assert_eq!(err.err().unwrap().to_string(), "cannot connect: connect error");
    }

    #[test]
    fn it_can_disconnect() {
        let mut fake_connection = MockConnection::new();
        fake_connection
            .expect_disconnect()
            .returning(|| Ok(()));

        let client = Client::new(Rc::new(fake_connection));
        let ok = client.disconnect();
        assert!(ok.is_ok());
    }

    #[test]
    fn it_handles_disconnect_errors() {
        let mut fake_connection = MockConnection::new();
        fake_connection
            .expect_disconnect()
            .returning(|| Err("disconnect error".into()));

        let client = Client::new(Rc::new(fake_connection));
        let err = client.disconnect();
        assert_eq!(err.err().unwrap().to_string(), "cannot disconnect: disconnect error");
    }

    #[test]
    fn it_can_send_messages() {
        let message = "Hello, world!";

        let mut fake_connection = MockConnection::new();
        fake_connection
            .expect_write()
            .with(eq(message))
            .returning(|_| Ok(()));

        let client = Client::new(Rc::new(fake_connection));
        let ok = client.send_message(message);
        assert!(ok.is_ok());
    }

    #[test]
    fn it_handles_send_message_errors() {
        let message = "Hello, world!";

        let mut fake_connection = MockConnection::new();
        fake_connection
            .expect_write()
            .returning(|_| Err("write error".into()));

        let client = Client::new(Rc::new(fake_connection));
        let err = client.send_message(message);
        assert_eq!(err.err().unwrap().to_string(), "cannot write message: write error");
    }

    #[test]
    fn it_can_read_messages(){
        let message = "Hello, world!";

        let mut fake_connection = MockConnection::new();
        fake_connection
            .expect_read()
            .returning(|| Ok(message.to_string()));

        let client = Client::new(Rc::new(fake_connection));
        let ok = client.read_message();
        assert!(ok.is_ok());
        assert_eq!(ok.unwrap(), message);
    }

    #[test]
    fn it_handles_read_message_errors() {
        let mut fake_connection = MockConnection::new();
        fake_connection
            .expect_read()
            .returning(|| Err("read error".into()));

        let client = Client::new(Rc::new(fake_connection));
        let err = client.read_message();
        assert_eq!(err.err().unwrap().to_string(), "cannot read message: read error");
    }
}
