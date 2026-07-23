# PORTING.md — お引越し可能ファイル

他プロジェクトへそのまま(または軽微な変更で)持っていける実装パターン一覧。

## `src/i18n.rs` + `src/meta_index.rs`(このリポジトリへの移植元: `aruaru-tokyo`)

エコシステム全体のメタ索引リポジトリ(`RUNO`、旧称: `open-aruaru-runo-iLumi`
→ `aon` → `RUNO`)と同内容の「プロジェクトシリーズ索引」ページ一式。
13言語対応(`?lang=`クエリパラメータ方式)・GitHub風ファイル一覧・
クライアントサイドJSでのGitHub API即時取得(⭐Star数・最終更新日時・
既定ブランチ)。他のPoemベースリポジトリへコピーする場合の必須条件:

- `crate::STYLE`(`&'static str`のCSS定数)と`crate::html_escape()`
  (`&str -> String`のHTMLエスケープ関数)をコピー先のクレートルートに
  用意すること(`meta_index.rs`が`use crate::html_escape;`で参照する)。
- `PROJECTS`一覧(`meta_index.rs`)は、コピー先リポジトリ自身を含めて
  実在確認済みのプロジェクトのみ掲載すること(推測での追加は禁止)。
- ルーティングは`Route::new().at("/runo", get(handler))`のように
  1エントリで済む(`Query<LangQuery>`で`?lang=`を受ける)。

```rust
#[handler]
async fn runo_index_page(Query(q): Query<LangQuery>) -> Html<String> {
    let lang = i18n::Lang::parse(q.lang.as_deref());
    Html(meta_index::render_page(lang, "/runo"))
}
```

## 自治体リンク集パターン(`MuniGroup`/`MuniLink`、`src/lib.rs`)

URLが未提供の項目を推測で埋めず「準備中」と正直に表示するための
データ構造。`url: Option<&'static str>` + `note: Option<&'static str>`
の組み合わせで、(a) URLあり、(b) URLあり+注記(「アクセス」等)、
(c) URLなし+注記(「準備中」等)、(d) URLなしのみ、の4パターンを
表現できる。同種の「実在確認できたものだけ載せる」リンク集機能を持つ
他リポジトリ(karu.tokyo等)へそのまま移植可能。

```rust
pub struct MuniLink {
    pub label: &'static str,
    pub url: Option<&'static str>,
    pub note: Option<&'static str>,
}
```

## `src/github_cache.rs`(GitHub API定期取得キャッシュ、2026-07-23新設)

`/runo`ページのプロジェクトカードに、GitHub API由来の説明文・最終更新
日時・スター数を「サーバー側で定期取得したもの」として表示するための
バックグラウンド更新機構。他のPoemベースリポジトリ(`aruaru-tokyo`等)へ
そのまま持っていける独立モジュール(依存は`reqwest`/`tracing`/
`serde`/`tokio`のみ、テストに`wiremock`が必要)。

- `spawn_background_refresh()`を`main()`から一度だけ呼ぶ(`build_app()`
  ではなくテストで何本も起動されないよう分離すること、`lib.rs`の
  `spawn_background_tasks()`参照)。
- `get(repo: &str) -> Option<LiveInfo>`で読み出し、`None`の場合は
  呼び出し側で静的な既定値へフォールバックすること(`meta_index.rs`の
  `render_project_card`が実例)。
- **`https://api.github.com/orgs/<owner>/repos`ではなく
  `https://api.github.com/users/<owner>/repos`を使うこと**——移植先の
  GitHubアカウントがOrganizationかUserかを`GET /orgs/<owner>`で
  事前確認すること(このリポジトリでの実例: `aon-co-jp`は実はUser
  アカウントだった)。
- テストで`refresh_once()`を並列に呼ぶ場合、プロセス全体で共有する
  `RwLock`キャッシュを複数テストが同時に上書きし合うレースが起きる
  ため、`test_lock()`(`std::sync::Mutex`)で直列化すること。

## `Project::distribution`(配布状況フィールド、2026-07-24新設)

`meta_index::Project`に`pub distribution: Option<Distribution>`を追加。
`Distribution { windows_url, linux_url, android_status, note }`はいずれも
`Option<&'static str>`(`note`のみ`&'static str`)で、静的に配布状況を
管理する。他のPoemベースリポジトリ(`aruaru-tokyo`等)へ同じ「プロジェクト
シリーズ索引」パターンをコピーする場合は、この構造体と
`render_project_card()`内の`dist_box`組み立てロジックも一緒に移植する
こと。

- `distribution: None` → 配布物を持たないプロジェクト(ライブラリ専用
  クレート等)として「配布状況」セクション自体を描画しない。
- `windows_url`/`linux_url`が`Some`の場合のみダウンロードボタンを表示。
  値は必ず**実際に`gh release list -R <repo>`でReleaseの実在を確認して
  から**設定すること(推測禁止)。
- `note`は「インストーラースクリプトはあるがReleaseがまだ無い」等の
  状況説明に使う(`windows_url`/`linux_url`が両方`None`でも空文字列に
  せず、正直な理由を書くこと)。
- **自動検出を見送った理由**: `GET /repos/<repo>/releases/latest`で
  Release実在を自動検出することも技術的には可能だが、未認証GitHub API
  のレート制限(60req/時)は`github_cache.rs`の一括取得
  (`GET /users/<owner>/repos`、1〜2リクエストで全リポジトリ分)と違い、
  リポジトリ数だけ個別リクエストが必要になる。配布状況は変化頻度が
  低い情報のため、この制約下では静的フィールド管理の方が妥当と判断した
  (`src/meta_index.rs`の`Distribution`構造体コメント参照)。

## `readme-to-rs`のpath依存を安全に外した実例(`Cargo.toml`)

依存パスが実在しないpath依存(`F:\open-runo\open-runo\readme-to-rs`)は
ビルドを壊す。移植・復元前に必ず`ls`等で実在確認してから依存を戻す
かどうか判断すること(本リポジトリでは実在しないことを確認した上で
依存自体を削除し、当該機能もコードから除去した)。
