use anyhow::Result;
use bore_cli::{client::Client, server::{Server, ServerCallbacks}, haproxy::HaproxyAdmin};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Starts a local proxy to the remote server.
    Local {
        /// The local port to expose.
        local_port: u16,

        /// The local host to expose.
        #[clap(short, long, value_name = "HOST", default_value = "localhost")]
        local_host: String,

        /// Address of the remote server to expose local ports to.
        #[clap(short, long, env = "BORE_SERVER")]
        to: String,

        /// Optional port on the remote server to select.
        #[clap(short, long, default_value_t = 0)]
        port: u16,

        /// Optional secret for authentication.
        #[clap(short, long, env = "BORE_SECRET", hide_env_values = true)]
        secret: Option<String>,
    },

    /// Runs the remote proxy server.
    Server {
        /// Minimum TCP port number to accept.
        #[clap(long, default_value_t = 1024)]
        min_port: u16,

        /// Optional secret for authentication.
        #[clap(short, long, env = "BORE_SECRET", hide_env_values = true)]
        secret: Option<String>,
    },
}

struct HaproxyCallbacks(HaproxyAdmin);
impl HaproxyCallbacks {
    fn new(stream_path: &str, server_prefix: &str) -> Self {
        Self { 0: HaproxyAdmin::new(stream_path.to_string(), server_prefix.to_string()) }
    }
}

impl ServerCallbacks for HaproxyCallbacks {
    fn on_established(&self, port: u16) -> std::io::Result<()> {
        self.0.add_server(port)
    }

    fn on_dropped(&self, port: u16) -> std::io::Result<()> {
        self.0.del_server(port)
    }
}

struct DummyCallbacks();
impl ServerCallbacks for DummyCallbacks {
    fn on_established(&self, port: u16) -> std::io::Result<()> {
        println!("established => 127.0.0.1:{}", port);
        Ok(())
    }

    fn on_dropped(&self, port: u16) -> std::io::Result<()> {
        println!("dropped     => 127.0.0.1:{}", port);
        Ok(())
    }
}

#[tokio::main]
async fn run(command: Command) -> Result<()> {

    match command {
        Command::Local {
            local_host,
            local_port,
            to,
            port,
            secret,
        } => {
            let client = Client::new(&local_host, local_port, &to, port, secret.as_deref()).await?;
            client.listen().await?;
        }
        Command::Server { min_port, secret } => {
            let callbacks = Box::new(HaproxyCallbacks::new("/var/run/admin.sock", "operators/op"));
            // let callbacks = Box::new(DummyCallbacks{});
            Server::new(min_port, secret.as_deref(), Some(callbacks)).listen().await?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    run(Args::parse().command)
}
