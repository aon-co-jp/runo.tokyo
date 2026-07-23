use poem::listener::TcpListener;
use poem::Server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();
    // `/runo`ページ用のGitHubリポジトリ情報を定期取得するバックグラウンド
    // タスクを起動(取得失敗時もパニックせず静的な既定値へフォールバックする、
    // `src/github_cache.rs`参照)。
    runo_tokyo::spawn_background_tasks();
    let app = runo_tokyo::build_app();
    println!("runo.tokyo (ひな型) starting on http://127.0.0.1:3000");
    Server::new(TcpListener::bind("127.0.0.1:3000")).run(app).await
}
