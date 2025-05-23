use std::io::{self};

mod tcp;
mod models;

fn main() {
    match tcp::server::spawn_tcp_server() {
        Ok(_) => {
            println!("Server exited succesfully!");
        }
        Err(e) => {
            println!("'Server' exit status: {e}");
        }
    }
}