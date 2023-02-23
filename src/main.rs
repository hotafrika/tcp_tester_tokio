use tcp_tester_tokio;

#[tokio::main]
async fn main() {
    let args = tcp_tester_tokio::Args::parse_args();
    tcp_tester_tokio::run(args).await;
}
