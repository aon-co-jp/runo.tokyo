# 開発方針＆開発環境ルール(runo.tokyo)

作業ドライブは`F:\open-runo`。この節は[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の`CLAUDE.md`を正本とし、各プロジェクトへコピーして同期する方針に準じる。

## 開発方針・開発環境ルール(全リポジトリ共通ヘッダー、要約引用)

比較的新しい・情報量がまだ少なめのWebフレームワーク(本エコシステムが
採用する[Poem](https://github.com/poem-web/poem)等)は、AI駆動開発が
実装を勘違いしやすい傾向がある。作業前にRust/Poem/Tokioの公式ドキュメント
・GitHubを軽く参照してから着手すること(詳細な参照表は`open-raid-z`の
`CLAUDE.md`正本を参照)。

**確認不要の自動継続・リミット解除後の自動再開**(open-raid-z CLAUDE.md
の運用ルール継承): セッション中断→リミット解除後は、「続けてよろしい
ですか」等の確認を挟まず、`git status`/`git log`と本ファイルの
HANDOFF節を確認した上で自動的に前回作業を再開すること。

**白画面バグ等を見逃さない検証徹底**(同上継承): WEB/UIを実装した後は、
ビルド成功・`cargo test`・curlでのステータスコード確認だけで「完了」と
報告せず、実際にHTMLボディの中身に期待した文字列が含まれているかまで
確認すること。

## このリポジトリの役割

`runo.tokyo`のTOPページ。東京都西部(あきる野市・旧五日市・桧原村・
青梅・奥多摩・羽村・福生・昭島・立川・国立・八王子・高尾・日の出町など)
の暮らし・テレワークを紹介しつつ、企業・工場・倉庫事業用地のドローン
空撮紹介(準備中)や、open-aruaru/open-runoエコシステムの入口
(`/runo`のプロジェクトシリーズ索引)を提供する。Rust + Poem製、
DB非依存・1バイナリ完結。

## 技術スタック

- Rust + Poem(hyperベースの軽量Webフレームワーク)。
- フロントエンドはサーバーサイドで文字列組み立てしたプレーンHTML
  (テンプレートエンジン不使用)。
- 重量級フレームワーク・ORMは使わない(poem-cosmo-tauriエコシステムの
  規約に準拠)。

## 主要モジュール

- `src/lib.rs` — TOPページ(`/`)の本体。`family_apps()`(アプリ紹介)・
  `MUNICIPALITY_LINKS`(自治体リンク集、`MuniGroup`/`MuniLink`)・
  `INTRO_PARAGRAPHS`(ユーザー確定済み紹介文、改変禁止)・
  `render_drone_section()`(ドローン空撮紹介、準備中プレースホルダー)を
  組み立てて`top_page_html()`を返す。
- `src/i18n.rs` / `src/meta_index.rs` — `aruaru-tokyo`の同名ファイルを
  移植。`/runo`ページ(プロジェクトシリーズ索引、13言語対応)の実装。
  詳細は`PORTING.md`参照。
- `src/main.rs` — `runo_tokyo::build_app()`を`Server`にバインドして起動
  するだけの薄いエントリポイント。

## ルーティング

- `GET /` — TOPページ
- `GET /runo` — プロジェクトシリーズ索引(`?lang=`で13言語切替)

## 運用ルール

- 自治体リンクのURLは、ユーザーから実在確認済みとして提供されたものの
  みを掲載すること。**未提供のURLを推測で埋めてはならない**——不明な
  場合は「準備中」等と正直に表示する(本リポジトリ固有ルール)。
- `readme-to-rs`のようなpath依存を復元する場合は、必ず依存先ディレクトリ
  の実在を確認してから行うこと(過去に壊れていた実例あり、詳細は
  `PORTING.md`参照)。

## HANDOFF(直近の作業ログ、上が最新)

- **2026-07-20**: 前回セッションがAPI接続エラーで中断した状態
  (`Cargo.toml`のみ変更・未コミット)から再開し、TOPページ本体と
  `/runo`ページを実装完了。
  - `Cargo.toml`: 壊れていた`readme-to-rs`のpath依存
    (`F:\open-runo\open-runo\readme-to-rs`、実在しないことを確認済み)
    を削除し、`serde_json`を追加。`readme-to-rs`によるREADME変換機能
    自体もコードから削除(GitHubリンクとしての紹介は`family_apps()`に
    残置)。
  - `src/lib.rs`: TOPページに、ユーザー確定済みの紹介文(4段落、
    改変なしでそのまま掲載)・東京都西部各自治体へのリンク集・
    企業/工場/倉庫事業用地のドローン空撮紹介(準備中プレースホルダー、
    実動画は未収録)・Google画像検索(`https://www.google.com/imghp`)
    へのリンク(親子でのテレワーク・パソコン学習の文脈)を追加。
    既存のopen-aruaru/open-runo系列アプリ一覧(`family_apps`)は維持。
  - `src/i18n.rs`/`src/meta_index.rs`: `aruaru-tokyo`の同名ファイルを
    移植し、`/runo`パスでプロジェクトシリーズ索引ページを新設
    (共通crate化はせず、ユーザー指示通りファイルをコピー・移植する
    方式)。メタリポジトリの最新名`RUNO`(旧称:
    open-aruaru-runo-iLumi → aon → RUNO)に追従し、`PROJECTS`一覧へ
    `runo.tokyo`自身のエントリを追加。TOPページから`/runo`への
    リンクも追加。
  - **自治体リンクの反映経緯(セッション中に複数回に分けてユーザーから
    実在URLの追加提供があった)**: 当初は
    羽村市・福生市・昭島市・立川市・国立市・日の出町の6自治体が
    URL未提供で「準備中」表示だったが、セッション中の追加提供により
    最終的に以下の実リンクへ差し替え済み: 高尾登山電鉄
    (`https://www.takaotozan.co.jp/`)、八王子市役所
    (`https://www.city.hachioji.tokyo.jp/`)、日の出町役場
    (`https://www.town.hinode.tokyo.jp/`)、福生市役所
    (`https://www.city.fussa.tokyo.jp/`)、羽村市役所
    (`https://www.city.hamura.tokyo.jp/`)、拝島駅(西武鉄道:
    `https://www.seiburailway.jp/railway/station/haijima/`、JR東日本:
    `https://www.jreast.co.jp/estation/stations/1213.html`——福生市・
    昭島市エリアの交通アクセス情報として掲載)、昭島市役所
    (`https://www.city.akishima.lg.jp/`)、立川市役所
    (`https://www.city.tachikawa.lg.jp/`)、国立市役所
    (`https://www.city.kunitachi.tokyo.jp/index.html`)。
  - **最終的に「準備中」のまま残っている制約(正直な開示)**: 桧原村の
    公式村サイトのみ、実在URLが未提供のため「準備中」表示のまま
    (桧原村観光案内`https://hinohara-kankou.jp/`は掲載済み)。また
    企業・工場・倉庫事業用地のドローン空撮紹介は、実際の動画・YouTube
    チャンネルが未収録・未公開のため、引き続き準備中プレースホルダー
    表示のまま(撮影・公開後にリンクを追加する設計)。
  - **検証**: `cargo check`/`cargo test`(8件)がいずれも成功。
    `cargo run --release`で実機起動(`127.0.0.1:3000`)し、`curl`で
    (1) `/`・`/runo`ともにHTTP 200、(2) TOPページHTML内に全自治体URL
    (桧原村公式を除く)・Google画像検索URL・`href="/runo"`・
    「準備中」・「ドローン」の各文字列が実在すること、(3) `/runo`の
    HTML内に`aon-co-jp/RUNO`・「設計思想＆開発方針＆開発環境ルール」
    (17箇所)が含まれること、(4) `?lang=en`で`Project Series Index`
    に切り替わること、(5) `?lang=ar`で`dir="rtl"`が付与されることを
    確認済み。

## 関連プロジェクト

- [aruaru-tokyo](https://github.com/aon-co-jp/aruaru-tokyo) — `/runo`
  ページ(`i18n.rs`/`meta_index.rs`)の移植元
- [RUNO](https://github.com/aon-co-jp/RUNO)(旧称:
  open-aruaru-runo-iLumi → aon) — エコシステム全体のメタ索引リポジトリ
- [open-runo](https://github.com/aon-co-jp/open-runo) — Rust→WASM/tokio+hyperのopen-runoエコシステム本体
- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — 開発ルールの正本
