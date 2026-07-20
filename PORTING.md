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

## `readme-to-rs`のpath依存を安全に外した実例(`Cargo.toml`)

依存パスが実在しないpath依存(`F:\open-runo\open-runo\readme-to-rs`)は
ビルドを壊す。移植・復元前に必ず`ls`等で実在確認してから依存を戻す
かどうか判断すること(本リポジトリでは実在しないことを確認した上で
依存自体を削除し、当該機能もコードから除去した)。
