mod server;
mod parser;

fn main() {
    let addr = "0.0.0.0:6869".parse().unwrap();
    server::run(addr);
}
