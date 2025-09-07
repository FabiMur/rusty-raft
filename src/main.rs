use clap::Parser;
#[derive(Parser, Debug)]
#[clap(name = "rusty-raft", version, about = "Raft node in Rust")]
struct Args {
    // Unique node id
    #[clap(long)]
    node_id: u64,

    // HTTP port for client/status
    #[clap(long, default_value = "8080")]
    http_port: u16,

    // gRPC port for Raft RPCs
    #[clap(long, default_value = "50051")]
    grpc_port: u16,

    // Comma-separated list of initial cluster members
    #[clap(long, default_value = "")]
    initial_cluster: String,
}

fn main() {
    println!("Hello, world!");
}
