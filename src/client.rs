use tungstenite::{Message, WebSocket, connect, stream::MaybeTlsStream};

#[derive(Debug)]
pub struct WsClient {
    socket: WebSocket<MaybeTlsStream<std::net::TcpStream>>,
}

impl WsClient {
    pub fn connect(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (socket, _) = connect(url)?;

        Ok(Self { socket })
    }

    pub fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.socket.close(None)?;
        Ok(())
    }

    pub fn query_windows(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.socket.write(Message::Text("query windows".into()))?;
        self.socket.flush()?;

        match self.socket.read()? {
            Message::Text(text) => Ok(text.to_string()),
            Message::Close(_) => Ok(String::from("")),
            _ => Ok(String::from("")),
        }
    }

    pub fn query_workspaces(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.socket
            .write(Message::Text("query workspaces".into()))?;
        self.socket.flush()?;

        match self.socket.read()? {
            Message::Text(text) => Ok(text.to_string()),
            Message::Close(_) => Ok(String::from("")),
            _ => Ok(String::from("")),
        }
    }

    pub fn focus_container(&mut self, id: &String) -> Result<String, Box<dyn std::error::Error>> {
        self.socket.write(Message::Text(
            format!("command focus --container-id {}", id).into(),
        ))?;
        self.socket.flush()?;

        match self.socket.read()? {
            Message::Text(text) => Ok(text.to_string()),
            Message::Close(_) => Ok(String::from("")),
            _ => Ok(String::from("")),
        }
    }

    pub fn focus_workspace(&mut self, name: &String) -> Result<String, Box<dyn std::error::Error>> {
        self.socket.write(Message::Text(
            format!("command focus --workspace {}", name).into(),
        ))?;
        self.socket.flush()?;

        match self.socket.read()? {
            Message::Text(text) => Ok(text.to_string()),
            Message::Close(_) => Ok(String::from("")),
            _ => Ok(String::from("")),
        }
    }

    pub fn move_window(
        &mut self,
        workspace_name: &String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.socket.write(Message::Text(
            format!("command move --workspace {}", workspace_name).into(),
        ))?;
        self.socket.flush()?;

        match self.socket.read()? {
            Message::Text(text) => Ok(text.to_string()),
            Message::Close(_) => Ok(String::from("")),
            _ => Ok(String::from("")),
        }
    }
}

impl Drop for WsClient {
    fn drop(&mut self) {
        let _ = self.socket.close(None);
    }
}
