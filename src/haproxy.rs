use std::{os::unix::net::UnixStream, io::{Write, Read}, net::Shutdown};

use uuid::Uuid;

pub struct HaproxyAdmin {
    stream_path: String,
    server_prefix: String,
}

impl HaproxyAdmin {
    pub fn new(stream_path: String, server_prefix: String) -> Self {
        return HaproxyAdmin { stream_path, server_prefix }
    }

    fn stream_write(stream: &mut UnixStream, command: &String) -> std::io::Result<()> {
        stream.write_all(command.as_bytes())
    }

    pub fn add_server(&self, port: u16) -> std::io::Result<()> {
        let mut stream = UnixStream::connect(&self.stream_path)?;
        let server_name = format!("{}{}", &self.server_prefix, port);

        let add_server_cmd = format!("add server {} {}:{}\n", &server_name, "127.0.0.1", port);
        Self::stream_write(&mut stream, &add_server_cmd)?;

        let set_server_cmd = format!("set server {} state ready\n", &server_name);
        Self::stream_write(&mut stream, &set_server_cmd)?;

        stream.shutdown(Shutdown::Write)
    }

    pub fn del_server(&self, port: u16) -> std::io::Result<()> {
        let mut stream = UnixStream::connect(&self.stream_path)?;
        let server_name = format!("{}{}", &self.server_prefix, port);

        let set_server_cmd = format!("set server {} state maint\n", &server_name);
        Self::stream_write(&mut stream, &set_server_cmd)?;

        let del_server_cmd = format!("del server {}", &server_name);
        Self::stream_write(&mut stream, &del_server_cmd)?;

        stream.shutdown(Shutdown::Write)
    }
}