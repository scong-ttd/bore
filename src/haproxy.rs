use std::{os::unix::net::UnixStream, io::Write};

use uuid::Uuid;

pub struct HaproxyAdmin {
    stream_path: String,
    server_prefix: String,
}

impl HaproxyAdmin {
    pub fn new(stream_path: String, server_prefix: String) -> Self {
        return HaproxyAdmin { stream_path, server_prefix }
    }

    fn stream_write(&self, command: &String) -> std::io::Result<()> {
        UnixStream::connect(&self.stream_path)?.write_all(command.as_bytes())
    }

    pub fn add_server(&self, port: u16) -> std::io::Result<()> {
        let server_name = format!("{}{}", &self.server_prefix, port);

        let add_server_cmd = format!("add server {} {}:{}\n", &server_name, "127.0.0.1", port);
        self.stream_write(&add_server_cmd)?;

        let set_server_cmd = format!("set server {} state ready\n", &server_name);
        self.stream_write(&set_server_cmd)
    }

    pub fn del_server(&self, port: u16) -> std::io::Result<()> {
        let server_name = format!("{}{}", &self.server_prefix, port);

        let set_server_cmd = format!("set server {} state maint\n", &server_name);
        self.stream_write(&set_server_cmd)?;

        let del_server_cmd = format!("del server {}", &server_name);
        self.stream_write(&del_server_cmd)
    }
}