# runo.tokyo

東京都西部(あきる野市・旧五日市・桧原村・青梅・奥多摩・羽村・福生・昭島・立川・国立・八王子・高尾・日の出町など)の暮らし・テレワークと、open-runo/open-aruaruエコシステムの入口となるWebサイト。Poem(Rust)製。

## 現状の機能

- TOPページ(`/`)
  - ユーザー確定済みの紹介文(改変禁止・そのまま掲載)
  - 東京都西部各自治体の役所・観光案内サイトへのリンク集(実在確認済みURLのみ掲載。未提供のものは推測せず「準備中」と明記)
  - 企業・工場・倉庫事業用地のドローン空撮紹介(実際の動画は未収録のため準備中プレースホルダー)
  - 親子でのテレワーク・パソコン学習の文脈でのGoogle画像検索(`https://www.google.com/imghp`)へのリンク
  - open-aruaru/open-runo系列アプリの一覧とGitHubリンク
  - `/runo`(プロジェクトシリーズ索引)へのリンク
- `/runo` — エコシステム全体のメタ索引リポジトリ`RUNO`(旧称: open-aruaru-runo-iLumi → aon → RUNO)と同内容の「プロジェクトシリーズ索引」ページ。13言語対応(`?lang=`)。`aruaru-tokyo`の`src/meta_index.rs`/`src/i18n.rs`を移植。
  - **どのリポジトリを載せるか**(`meta_index::PROJECTS`)は引き続きこのリポジトリ側で静的に管理する。**各リポジトリの詳細情報(説明文・最終更新日時・スター数)はGitHub APIから自動更新**する(`src/github_cache.rs`、既定1時間おきのバックグラウンド取得、詳細は下記「GitHub自動更新機能」節参照)。

## GitHub自動更新機能(`/runo`ページ、2026-07-23実装)

`meta_index::PROJECTS`(掲載するリポジトリ一覧)は静的に管理する一方、各
リポジトリの説明文・最終更新日時・スター数は`src/github_cache.rs`が
プロセス内キャッシュ(`RwLock<HashMap<owner/repo, LiveInfo>>`)として
定期的(既定1時間おき)にGitHub APIから取得・更新する。`/runo`ページの
各プロジェクトカードは、キャッシュに値があれば「🔄 GitHub自動同期」の
行として説明文・最終更新日時・⭐スター数を表示し、未取得(起動直後・
GitHub API到達不能・レート制限等)の場合はこの行を出さず静的な
`role_ja`のみで表示する(補助機能の失敗が権威パスであるページ描画を
ブロックしない設計、`open-web-server`の`ddns.rs`/`acme.rs`と同じ方針)。

**正直な開示(実HTTP到達確認の結果判明した事実)**: 当初は
`GET /orgs/aon-co-jp/repos`(Organizationの全リポジトリ一括取得)の
利用を想定していたが、実際に到達確認したところ**`aon-co-jp`は
Organizationではなく通常のUserアカウント**であり(`GET /orgs/aon-co-jp`
は`404`)、代わりに`GET /users/aon-co-jp/repos`(User版、ページネーション
対応)を使う実装に変更した。認証不要の公開エンドポイントのみ使用。

## 未実装(今後の拡張)

- AI辞書のGoogleドライブ連携(ユーザー自身のGoogleドライブで辞書を管理)
- PostgreSQL + aruaru-dbのデュアルDB構成
- バージョンレス/バージョン管理/Git管理のトライブリッド方式
- Tauri 2.0デスクトップ版
- ドローン空撮の実際の動画・YouTubeチャンネル(現状はプレースホルダー)

## 自治体リンクの制約(正直な開示)

以下は実在URLが未提供のため「準備中」と表示している(推測でURLを埋めていない):

- 桧原村(公式サイト)

上記以外の全自治体(あきる野市・青梅市・奥多摩町・八王子市・羽村市・福生市・昭島市・立川市・国立市・日の出町)は実在URLが判明済みで実リンクを掲載している。

## 実行

```powershell
cargo run
# http://127.0.0.1:3000 で起動
```

## テスト

```powershell
cargo test
```

## ライセンス

MPL-2.0

---

## English summary

`runo.tokyo`'s TOP page, plus `/runo` — a "project series index" page listing
every repository in the open-cosmo/open-aruaru ecosystem. Which repositories
are listed (`meta_index::PROJECTS`) is still curated by hand in this repo;
each listed repository's live details (description, last-pushed time, star
count) are now auto-refreshed from the GitHub API in the background
(`src/github_cache.rs`, hourly by default, in-process `RwLock` cache). If a
fetch fails (rate limit, network outage, etc.) the page falls back to the
last known cache value, or to the static description if nothing has been
fetched yet — this auxiliary feature never blocks page rendering.

Honest note: `aon-co-jp` turned out to be a regular GitHub **user** account,
not an Organization (`GET /orgs/aon-co-jp` returns 404), so this feature
uses the `GET /users/aon-co-jp/repos` endpoint rather than the
organization-repos endpoint originally assumed.
