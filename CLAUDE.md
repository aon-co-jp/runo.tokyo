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

- **2026-07-20(さらに追記) open-runo→open-cosmo改名の反映 + systemd自動復帰の強化**:
  ユーザー指示により`aon-co-jp/open-runo`リポジトリを実際に
  `gh repo rename`で`open-cosmo`へ改名(GitHub側は自動リダイレクトが
  効くが、紹介文言・リンクテキストは改名後の名前に更新)。TOPページの
  `<title>`・meta description・見出し・紹介文中の「open-runo」表記を
  「open-cosmo」へ置換(`src/lib.rs`)。`/runo`ページのRPoem・
  open-web-serverの役割説明文中の「open-runo」も同様に修正
  (`src/meta_index.rs`)。TOPページの「エコシステム全体を見る」節の
  文言を、ユーザー指定の「RUNOリポジトリで、RUNOプロジェクトを
  御紹介しております」という表現+`https://github.com/aon-co-jp/RUNO`
  への直接リンクへ変更。
  - **systemd自動復帰の強化(ユーザー指示「サーバーERRORは自動復帰する
    ように」)**: VPS上の自作Webアプリ全サービス(`runo-tokyo`・
    `aruaru.tokyo`・`audiocafe-tokyo-rust`・`aon-tokyo-server`・
    `e-gov-server`・`karu-tokyo-server`・`open-easy-web`)の
    systemd unitに`RestartSec=2`(未設定だった4サービスへ追加)と
    `StartLimitIntervalSec=0`(全サービスへ新規追加、既定の「短時間に
    5回失敗したら諦める」制限を撤廃し、真に無制限リトライで自動復帰
    し続けるようにした)を統一適用。変更前は各unitファイルを
    タイムスタンプ付き`.bak-<timestamp>`で退避済み。**実検証**:
    `runo-tokyo`の実行中プロセスを`kill -9`し、約4秒後に新PID・
    `systemctl is-active`が`active`・`curl`でHTTP 200が返ることを
    確認(設定を書いただけでの報告ではなく実クラッシュを起こして復帰を
    確認済み)。
  - **検証**: `cargo test`(15件全green)、VPS本番反映
    (`git pull`→`cargo build --release`→再起動不要、systemdの
    daemon-reloadは稼働中プロセスを止めない)、本番`https://runo.tokyo/`
    で「open-cosmo」表記・RUNOリンクの反映を確認済み。

- **2026-07-20(再追記)**: ユーザー指示により、`src/i18n.rs`の
  `LABEL_CLAUDE_JA`定数(`/runo`ページでCLAUDE.mdへのリンクに使う表示
  ラベル、`aruaru-tokyo`からの移植元と同一パターン)を「CLAUDE.md
  (設計思想＆開発方針＆開発環境ルール)」から「CLAUDE.md(WEBアプリ
  設計思想、開発方針、開発環境ルール)」へ改名(ファイル名`CLAUDE.md`
  自体は変更なし、表示ラベルのみ)。`cargo build --release`後、
  実機起動(`127.0.0.1:3000`)し`curl`で`/runo`のHTML内に新ラベル文字列が
  17箇所含まれること・HTTP 200であることを確認済み。

- **2026-07-20(追記)**: TOPページの紹介文4段落それぞれのテーマに沿った
  「画像検索・Google検索・YouTube検索」3点セットのリンクを追加(追加要望
  対応)。
  - `src/lib.rs`: `intro_queries_for_paragraph()`で段落ごとのクエリ例を
    定義(段落0=「注文住宅 シアタールーム 壁埋め込みスピーカー」/
    「地方 郊外 移住 テレワーク 一戸建て」の2セット、段落1=「テレワーク
    リモートワーク カフェ ノートパソコン」、段落2=「親子 パソコン教室
    テレワーク」、段落3=「Claude Code Desktop 使い方」/「Claude Code
    フリーランス プログラマー」の2セット、いずれもユーザー提示のクエリ例
    をそのまま採用)。`percent_encode_query()`でRFC 3986準拠の
    パーセントエンコード(UTF-8バイト単位、非予約文字`A-Za-z0-9-_.~`
    以外を`%XX`化)を自前実装し、`render_search_link_set()`で
    `https://www.google.com/search?tbm=isch&q=<enc>`(画像検索)・
    `https://www.google.com/search?q=<enc>`(Google検索)・
    `https://www.youtube.com/results?search_query=<enc>`(YouTube検索)
    の3リンクを各段落の直後に配置。既存の汎用Google画像検索リンク
    (`https://www.google.com/imghp`)は段落2の後にそのまま残置。
  - **検証**: `cargo test`に新規テスト2件
    (`percent_encode_query_encodes_japanese_and_spaces`・
    `top_page_contains_contextual_search_link_sets_for_every_paragraph`)
    を追加し、既存分と合わせて計10件成功。`cargo run --release`で
    実機起動し、`curl`で取得した実際のTOPページHTMLに、6クエリ全てに
    ついて画像検索・Google検索・YouTube検索の3URL(パーセントエンコード
    済み)がそれぞれ実在することをPythonスクリプトで突き合わせ確認済み
    (単なるビルド成功では終わらせず、実HTML本文を直接検証)。

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
