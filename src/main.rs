mod server;
mod editor;
mod file_manager;
mod sync_manager;

fn main() {
    println!("Starting collaborative Rust editor...");
    server::start_server();
}