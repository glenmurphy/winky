#[tokio::main]
async fn main() {
    println!("Starting winky::listener demo");
    let mut rx = winky::raw_listen();
    loop {
        if let Some(msg) = rx.recv().await {
            println!("{:?}", msg);
        }
    }
}