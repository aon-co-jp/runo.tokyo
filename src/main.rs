use poem::listener::TcpListener;
use poem::Server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = runo_tokyo::build_app();
    println!("runo.tokyo (ひな型) starting on http://127.0.0.1:3000");
    Server::new(TcpListener::bind("127.0.0.1:3000")).run(app).await
}
