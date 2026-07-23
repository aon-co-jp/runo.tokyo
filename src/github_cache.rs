//! GitHub REST APIから各リポジトリの最新情報(説明文・最終更新日時・スター数)を
//! 定期的(既定1時間おき)に取得し、プロセス内キャッシュ(`RwLock`)へ反映する。
//!
//! **設計方針(open-web-server の `ddns.rs`/`acme.rs` と同じ「補助機能の失敗は
//! 権威パスをブロックしない」方針を踏襲)**: このキャッシュの取得に失敗しても
//! (GitHub API到達不能・レート制限等)パニックせず、直近取得できたキャッシュ値
//! (起動直後で一度も取得できていなければ`None`、その場合`meta_index.rs`側が
//! `Project`の静的な既定値へフォールバックする)をそのまま使い続ける。
//!
//! **`PROJECTS`配列自体(どのリポジトリを載せるか)は引き続きこのリポジトリ側で
//! 静的に管理する**——このモジュールが自動更新するのは、既に`meta_index::PROJECTS`
//! に載っている各リポジトリの「詳細情報」(説明文・最終更新日時・スター数)のみ。
//!
//! **正直な開示(2026-07-23、実HTTP到達確認の結果判明した事実)**: 当初は
//! `https://api.github.com/orgs/aon-co-jp/repos`(Organizationの全リポジトリ
//! 一括取得エンドポイント)の利用を想定していたが、実際に到達確認したところ
//! `aon-co-jp`は**Organizationではなく通常のUserアカウント**であり
//! (`GET /orgs/aon-co-jp`は`404 Not Found`)、代わりに
//! `https://api.github.com/users/aon-co-jp/repos`(User版の全リポジトリ一覧
//! エンドポイント)が`200 OK`で実際に到達できることを確認した。本モジュールは
//! このUser版エンドポイントを使う(ページネーション対応、`per_page=100`)。

use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use std::time::Duration;

/// GitHub APIから取得した、1リポジトリ分の「生きた」情報。
#[derive(Debug, Clone, PartialEq)]
pub struct LiveInfo {
    pub description: Option<String>,
    /// ISO 8601形式の文字列のまま保持する(表示側でパース・整形する)。
    pub pushed_at: Option<String>,
    pub stargazers_count: Option<u64>,
}

/// GitHub `GET /users/:owner/repos`の1エントリぶんを受け取るための
/// 最小限のデシリアライズ用構造体(未知フィールドは無視する)。
#[derive(Debug, Deserialize)]
struct RawRepo {
    full_name: String,
    description: Option<String>,
    pushed_at: Option<String>,
    stargazers_count: Option<u64>,
}

const GITHUB_API_BASE: &str = "https://api.github.com";
const OWNER: &str = "aon-co-jp";
/// バックグラウンド更新の既定間隔(ユーザー指示: 「例: 1時間おき」)。
const REFRESH_INTERVAL: Duration = Duration::from_secs(60 * 60);
const PER_PAGE: u32 = 100;
/// 異常なリポジトリ数増加時の暴走ページネーション対策(実際の想定件数を
/// 大きく超える上限、安全弁として設定)。
const MAX_PAGES: u32 = 20;

fn cache() -> &'static RwLock<HashMap<String, LiveInfo>> {
    static CACHE: OnceLock<RwLock<HashMap<String, LiveInfo>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

/// `owner/repo`形式のキーで、直近取得できたキャッシュ値を返す。
/// 一度も取得に成功していない、またはそのリポジトリがGitHub側に存在しない
/// 場合は`None`(呼び出し側は`Project`の静的な既定値へフォールバックすること)。
pub fn get(repo: &str) -> Option<LiveInfo> {
    cache().read().ok()?.get(repo).cloned()
}

/// テスト用: 現在キャッシュされている件数を返す。
#[cfg(test)]
pub fn cached_len() -> usize {
    cache().read().map(|m| m.len()).unwrap_or(0)
}

/// テスト専用: `cache()`はプロセス全体で1つの共有静的変数のため、
/// `cargo test`のデフォルト並列実行下では複数テストが同時に
/// `refresh_once`(キャッシュ全体を置き換える)を呼ぶと互いを上書きして
/// しまうレースが起きる。このキャッシュに触れるテストは、必ずこの
/// ミューテックスを先に獲得してから実行すること(`open-web-server`の
/// `OPEN_WEB_SERVER_ADMIN_TOKEN`環境変数レースと同種の問題への対策)。
#[cfg(test)]
pub fn test_lock() -> &'static std::sync::Mutex<()> {
    static LOCK: OnceLock<std::sync::Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| std::sync::Mutex::new(()))
}

/// `base_url`配下の`GET /users/aon-co-jp/repos`をページネーション付きで
/// 全件取得する。1ページでも取得できればその分はキャッシュへ反映し
/// (部分的成功も無駄にしない)、ネットワークエラー自体は呼び出し元へ伝える。
async fn fetch_all_repos(client: &reqwest::Client, base_url: &str) -> Result<Vec<RawRepo>, String> {
    let mut all = Vec::new();
    for page in 1..=MAX_PAGES {
        let url = format!("{base_url}/users/{OWNER}/repos?per_page={PER_PAGE}&page={page}");
        let resp = client
            .get(&url)
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "runo-tokyo-github-cache")
            .send()
            .await
            .map_err(|e| format!("request failed: {e}"))?;
        if !resp.status().is_success() {
            return Err(format!("GitHub API returned HTTP {}", resp.status()));
        }
        let batch: Vec<RawRepo> = resp.json().await.map_err(|e| format!("failed to parse JSON: {e}"))?;
        let got = batch.len();
        all.extend(batch);
        if got < PER_PAGE as usize {
            break; // 最終ページ
        }
    }
    Ok(all)
}

/// 1回分の取得・キャッシュ更新を実行する。成功した場合は更新件数を返す。
/// 失敗時はキャッシュを一切変更しない(直近の値をそのまま保持、
/// `ddns.rs`/`acme.rs`と同じ「補助機能の失敗は権威パスをブロックしない」方針)。
pub async fn refresh_once(client: &reqwest::Client, base_url: &str) -> Result<usize, String> {
    let repos = fetch_all_repos(client, base_url).await?;
    let count = repos.len();
    let mut map = HashMap::with_capacity(count);
    for r in repos {
        map.insert(
            r.full_name,
            LiveInfo { description: r.description, pushed_at: r.pushed_at, stargazers_count: r.stargazers_count },
        );
    }
    match cache().write() {
        Ok(mut guard) => {
            *guard = map;
            Ok(count)
        }
        Err(_) => Err("cache lock poisoned".to_string()),
    }
}

/// バックグラウンドタスクとして、起動直後に1回・以後`REFRESH_INTERVAL`ごとに
/// GitHub APIから最新情報を取得し続ける。取得失敗時はログに警告を出すのみで
/// パニックしない(直近のキャッシュ値、または起動直後で未取得なら`None`のまま
/// 静的な既定値へフォールバックする設計、`meta_index.rs`側で吸収)。
pub fn spawn_background_refresh() {
    tokio::spawn(run_loop(GITHUB_API_BASE.to_string()));
}

async fn run_loop(base_url: String) {
    let client = reqwest::Client::new();
    loop {
        match refresh_once(&client, &base_url).await {
            Ok(n) => tracing::info!("github_cache: refreshed {n} repos from GitHub API"),
            Err(e) => tracing::warn!("github_cache: refresh failed, keeping last known cache: {e}"),
        }
        tokio::time::sleep(REFRESH_INTERVAL).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// 実HTTP(モックサーバー)経由で、単一ページのレスポンスが正しくパース・
    /// キャッシュされることを検証する。
    #[tokio::test]
    async fn refresh_once_populates_cache_from_single_page() {
        let _guard = test_lock().lock().unwrap_or_else(|e| e.into_inner());
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {
                "full_name": "aon-co-jp/RS-Git",
                "description": "自作Git forge",
                "pushed_at": "2026-07-23T00:00:00Z",
                "stargazers_count": 3
            },
            {
                "full_name": "aon-co-jp/RS-Red",
                "description": "Redmine相当のチケット管理",
                "pushed_at": "2026-07-22T00:00:00Z",
                "stargazers_count": 0
            }
        ]);
        Mock::given(method("GET"))
            .and(path("/users/aon-co-jp/repos"))
            .and(query_param("page", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        // 2ページ目は空配列(=最終ページ)を返す設定にする必要はない——
        // 1ページ目の件数がper_page未満なら即終了する設計のため。

        let client = reqwest::Client::new();
        let n = refresh_once(&client, &server.uri()).await.expect("refresh should succeed");
        assert_eq!(n, 2);

        let git = get("aon-co-jp/RS-Git").expect("RS-Git should be cached");
        assert_eq!(git.description.as_deref(), Some("自作Git forge"));
        assert_eq!(git.stargazers_count, Some(3));

        let red = get("aon-co-jp/RS-Red").expect("RS-Red should be cached");
        assert_eq!(red.pushed_at.as_deref(), Some("2026-07-22T00:00:00Z"));
    }

    /// GitHub APIがエラー(例: レート制限で403)を返した場合、直近のキャッシュ値が
    /// 変更されず保持されることを検証する(パニックしない・古い値を失わない)。
    #[tokio::test]
    async fn refresh_once_keeps_previous_cache_on_http_error() {
        let _guard = test_lock().lock().unwrap_or_else(|e| e.into_inner());
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/users/aon-co-jp/repos"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let client = reqwest::Client::new();
        let before = cached_len();
        let result = refresh_once(&client, &server.uri()).await;
        assert!(result.is_err(), "HTTP 403 should surface as an error, not panic");
        assert_eq!(cached_len(), before, "cache should be unchanged after a failed refresh");
    }

    /// 到達不能なURL(接続自体が失敗するケース)でもパニックせず、
    /// エラーとして扱われることを検証する。
    #[tokio::test]
    async fn refresh_once_handles_unreachable_host_without_panicking() {
        let _guard = test_lock().lock().unwrap_or_else(|e| e.into_inner());
        let client = reqwest::Client::new();
        // .invalid はRFC 2606で予約された、絶対に解決できないTLD。
        let result = refresh_once(&client, "https://does-not-exist.invalid").await;
        assert!(result.is_err());
    }

    /// `get()`は未取得のリポジトリに対して`None`を返し(パニックしない)、
    /// 呼び出し側の静的フォールバックへ委ねられることを確認する。
    #[test]
    fn get_returns_none_for_unknown_repo() {
        assert_eq!(get("aon-co-jp/definitely-not-a-real-repo-xyz"), None);
    }
}
