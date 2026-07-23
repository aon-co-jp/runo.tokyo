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

- **2026-07-24 `/runo`ページの各プロジェクトカードに「配布状況」セクションを追加
  + 開発状況の紹介文を最新化(ユーザー指示「すべてのリポジトリの開発状況や
  追加した機能などをruno.tokyoのプロジェクトシリーズ内で紹介しなおして。
  完成していればWindows版、LINUX版、Andoroid版などをインストーラー付きで
  ダウンロード可能にして」)**:
  1. **前提の確認**: `F:\open-raid-z\CLAUDE.md`「エコシステム全体インストーラー
     整備計画」節(正本)の在庫調査を`ls <repo>/install.sh <repo>/install.ps1
     <repo>/.github/workflows/release.yml`で実際に再確認し、
     `gh release list -R aon-co-jp/<repo>`で公開Releaseの実在も確認した
     (推測での「完成」表記はしない)。ユーザー要望原文の「出来ていなければ
     完成させて」は各リポジトリ側の整備(別セッション並行作業の前提)に
     委ね、このパスは`runo.tokyo`の紹介ページを実態に即して正確に刷新
     することに専念した。
  2. **`src/meta_index.rs`に`Distribution`構造体+`Project::distribution`
     フィールドを新設**: `windows_url`/`linux_url`(実際にGitHub Release
     資産が存在する場合のみ`Some("https://github.com/<repo>/
     releases/latest")`)・`android_status`(クロスコンパイル確認止まり/
     APK生成まで到達/未着手を正直に文言化)・`note`(Releaseがまだ無い
     理由の説明)を持つ。`distribution: None`のプロジェクト(ライブラリ
     専用クレート等)は「配布状況」セクション自体を描画しない。
  3. **実際に確認した配布状況(2026-07-23/24時点)**:
     - **Windows/Linuxダウンロード可能**(3点セット+公開Release実在済み、
       `gh release view`でアセット名まで確認): RPoem・RS-Git・RS-Red・
       RS-Blog・RS-EC・open-web-server。
     - **インストーラースクリプトは整備済みだがReleaseはまだ無い**:
       aruaru-db・open-raid-z(v0.1.0タグはpush済みだが`gh run list`で
       Release自動ビルドがCI失敗中と確認、修正コミット済み・再タグでの
       検証待ちと正直に記載)・RS-LinkFusion(3点セット整備済みだが
       タグ未push)・RS-Guard・RS-Ops・RS-Sync(install.sh/ps1のみで
       release.yml未整備)。
     - **Android**: `open-web-server`のみKotlin/GradleシェルとAPK生成
       (`android/app/build/outputs/apk/debug/app-debug.apk`の実在を確認)
       まで到達、`adb`が`unauthorized`を返し続け実機/エミュレータでの
       動作確認は未完了と正直に表示(「開発中」であって「配布可能」では
       ない)。RS-Redは`cargo ndk`でのクロスコンパイル確認のみ(APK化は
       未着手)。他リポジトリはAndroid未着手。
  4. **開発状況・紹介文(`role_ja`)の更新**(各リポジトリのCLAUDE.md
     HANDOFF節を実際に読んで正確な文言にした): `open-web-server`
     (SFTP組み込みサーバー・UPnP自動ポート開放・無料DDNS〈最大20ドメイン〉・
     CORS対応・Android APK着手を追記)、`open-easy-web`(DuckDNS連携の
     「簡単ドメイン設定ウィザード」を追記)、`RS-Red`(ガントチャート用
     フィールド・status/project_idフィルタAPI・DDNS運用対応・
     `StorageBackend`抽象化〈ローカル/SFTP/Google Drive〉を追記)、
     `RPoem`/`RCosmo`(Cosmo共通20クレート中18件の完全一致を`diff -rq`で
     実調査・`scripts/sync-cosmo-core.sh`新設・Windowsネイティブ環境
     固有の実バグ〈テストがUnix専用`sleep`をハードコード〉修正を追記)、
     `aruaru-db`/`open-raid-z`(インストーラー3点セット追加を追記)。
  5. **`github_cache.rs`との役割分担を明確化**: 配布状況(インストーラー・
     Release有無)は`GET /repos/<repo>/releases/latest`で自動検出も
     技術的には可能だが、未認証GitHub APIのレート制限(60req/時)は
     既存の一括取得(`GET /users/aon-co-jp/repos`、1〜2リクエストで
     全リポジトリ分)と異なりリポジトリ数だけ個別リクエストが必要になる
     ため、更新頻度が低い配布状況情報にはオーバースペックと判断し、
     静的な`Project`フィールドとして管理する工学的判断を採った
     (`src/meta_index.rs`の`Distribution`構造体コメント・`PORTING.md`に
     理由を明記)。
  6. **検証**: `cargo build --tests`成功。`cargo test`**29件全green**
     (新規1件: `runo_index_page_shows_distribution_status_for_installable_
     projects_only`——open-web-serverのReleaseダウンロードリンク・
     Android文言・「配布状況」見出しの実在、RS-JSON〈配布物なし〉の
     カードに配布状況ボックスが出ないことを確認)。**実HTTP検証**:
     `cargo run --release`で実サーバーを起動し、`curl http://
     127.0.0.1:3000/runo`で取得した実HTML本文に
     `https://github.com/aon-co-jp/open-web-server/releases/latest`・
     Android対応の正直な文言・「配布状況」の見出し(13箇所)が実在する
     こと、RS-JSONのカード範囲(`<article>`〜`</article>`)には
     `dist-box`が出現しないことを`grep`/`sed`で直接確認した(型チェック
     ・ビルド成功のみで完了と報告していない)。
  - 次にすべきこと: (1) aruaru-db/open-raid-zのv0.1.0タグ再push→CI検証
    完了後、`windows_url`/`linux_url`を`Some`へ更新、(2) RS-LinkFusion
    のタグpush後も同様に更新、(3) RS-Guard/RS-Ops/RS-Syncへの
    `release.yml`整備(各リポジトリ側の並行セッション作業待ち)完了後、
    同様にリンクを追加、(4) 配布状況の自動検出(GitHub Release API)が
    将来的にレート制限面で現実的になった場合は`github_cache.rs`への
    統合を再検討。

- **2026-07-23 `/runo`ページの掲載漏れ・名称変更を修正 + GitHub API自動更新機能を実装
  (ユーザー指示「全てのリポジトリの紹介はrunoプロジェクトで紹介して、自動更新機能を搭載して」)**:
  1. **名称変更の反映**: `meta_index::PROJECTS`の`RS-Chiketto`エントリを
     `RS-Red`へ改名(2026-07-22にGitHub上のリポジトリ自体が改名済み、
     `F:\runo\RS-Red\CLAUDE.md`参照)。`src/lib.rs`の`SUB_SERVICES`・
     `chiketto_page`ハンドラ・テストも合わせて`aon-co-jp/RS-Red`参照へ
     修正(公開パス`/chiketto`自体は変更せず、対象リポジトリのみ修正
     ——`/RS-Red`への大規模なURL変更は今回のスコープ外)。`RJSON`エントリも
     実際のGitHubリポジトリ名`RS-JSON`(`git remote`で確認、`RJSON`/
     `Rust-JSON`はいずれも旧称・記載の食い違い)へ修正。
  2. **掲載漏れの修正**: `F:\runo`直下の全Gitリポジトリ(`git remote -v`で
     実在確認)と`PROJECTS`配列を突き合わせ、掲載漏れだった6件
     (`RS-Guard`・`RS-LinkFusion`・`RS-SmartTCP`・`RS-Gitbucket`
     [旧称rs-gitbucket]・`RS-Sync`[旧称rs-sync]・`runo-scanner`)を
     追加。各リポジトリのREADME.md冒頭を実際に読んで役割説明を記載
     (推測で書いていない)。`URL`・`runo-forge`ディレクトリは`.git`が
     存在せず実在するGitリポジトリではないため、意図的に掲載対象から
     除外(正直な開示としてこのHANDOFFに記録)。
  3. **GitHub API自動更新機能を新規実装**(`src/github_cache.rs`):
     `RwLock<HashMap<owner/repo, LiveInfo>>`のプロセス内キャッシュへ、
     既定1時間おきのバックグラウンドタスク(`main()`から
     `runo_tokyo::spawn_background_tasks()`を1回呼ぶ、`build_app()`とは
     分離しテストで多重起動しない設計)がGitHub APIから各リポジトリの
     説明文・最終更新日時・スター数を取得・反映する。取得失敗時(到達
     不能・レート制限等)はパニックせず直近のキャッシュ値(起動直後で
     未取得なら`None`)を保持し、`meta_index::render_project_card`は
     `None`の場合は静的な`role_ja`のみで描画する(`open-web-server`の
     `ddns.rs`/`acme.rs`と同じ「補助機能の失敗は権威パスをブロックしない」
     方針を踏襲)。**`PROJECTS`配列(どのリポジトリを載せるか)は
     引き続きこのリポジトリ側で静的に管理**し、このモジュールが自動
     更新するのは各リポジトリの「詳細情報」のみ、というユーザー指示の
     設計を厳守。
  4. **正直な開示(実装中に判明した事実)**: 当初`https://api.github.com/
     orgs/aon-co-jp/repos`(Organizationの一括取得エンドポイント)の
     利用を想定していたが、実HTTP到達確認の結果`GET /orgs/aon-co-jp`が
     `404`(=`aon-co-jp`はOrganizationではなく通常のUserアカウント)
     と判明したため、`GET /users/aon-co-jp/repos`(User版、
     `per_page=100`でページネーション)へ設計変更した。
  5. **検証**: `cargo build --tests`成功(警告なし)。`cargo test`
     **28件全green**(新規6件: `github_cache`モジュール4件
     [モックHTTPサーバー`wiremock`経由でのキャッシュ反映・HTTPエラー時の
     キャッシュ保持・到達不能ホストでのパニックなし動作・未知リポジトリで
     の`None`返却]、`lib.rs`側2件[RS-Chiketto→RS-Red改名の反映確認・
     掲載漏れ6件の掲載確認・GitHub自動同期表示のモックHTTP経由での実証])。
     並列実行時、複数テストがプロセス全体で共有する`github_cache`の
     `RwLock`キャッシュを同時に上書きし合うレースを発見し、
     `test_lock()`(`std::sync::Mutex`)による直列化で解消(実際に
     `cargo test`を複数回連続実行しflakyが再発しないことを確認)。
     **実HTTP検証(型チェックのみで完了と報告しない方針の徹底)**:
     `curl`でGitHub APIへの実到達を事前確認(`api.github.com/repos/
     aon-co-jp/RS-Git`→200、`api.github.com/orgs/aon-co-jp`→404で
     User判定を裏取り)。`cargo run --release`で実サーバーを起動し、
     `curl http://127.0.0.1:3000/runo`のHTML本文に`aon-co-jp/RS-Red`・
     6件の新規リポジトリすべて・`aon-co-jp/RS-JSON`が含まれ、
     `aon-co-jp/RS-Chiketto`・`aon-co-jp/RJSON`が含まれないことを実際に
     確認。さらに約8秒後にバックグラウンドタスクのログに
     `github_cache: refreshed 66 repos from GitHub API`が実際に出力され、
     再取得した`/runo`のHTML本文には29件中25件のプロジェクトカードに
     「🔄 GitHub自動同期」の行(実際のGitHub上の説明文・最終更新日時・
     スター数)が実際に反映されていることを確認した(実プロセス起動・
     実バックグラウンドループ・実GitHub API到達までの一気通貫を実証)。
  - 次にすべきこと: (1) `URL`・`runo-forge`が今後実際のGitリポジトリ化
    された場合は`PROJECTS`へ追加、(2) `/chiketto`パスを`RS-Red`の
    CLAUDE.mdが言及する`/RS-Red`(旧`/chiketto`は301リダイレクト)へ
    実際に移行するかの検討(今回は対象リポジトリ名の修正のみに留めた)、
    (3) GitHub自動更新の取得間隔・対象フィールド(トピック・ライセンス等)
    を必要に応じて拡張。

- **2026-07-22 RS-Opsを「プロジェクトシリーズ索引」+「エコシステム全体を
  見る」節で紹介(ユーザー指示、当初追加した専用`/RS-Ops`ページ・ルートは
  ユーザー指示により撤回・削除)**:
  1. **当初の実装(撤回済み)**: 最初は他のサブサービス(`/chiketto`等)と
     同じ`subservice_page`パターンで`/RS-Ops`という専用ルートを
     `SUB_SERVICES`+`build_app()`に追加したが、ユーザーから「プロジェクト
     シリーズ　と　エコシステム全体を見るの中で紹介して」との明確な
     指定を受け、この専用ルート・`SubService`エントリ・`rs_ops_page`
     ハンドラは削除した(RS-Opsは`/chiketto`等と異なり独自のWeb UIを
     持つ運用対象サービスではなく開発ツールであるため、専用サブページは
     過剰だったと判断)。
  2. **`src/meta_index.rs`の`PROJECTS`に`RS-Ops`(`aon-co-jp/RS-Ops`、旧称
     `RS-AI-DevOps`)エントリを追加**——これが「プロジェクトシリーズ
     索引」(`/runo`)での紹介に対応する。role_jaにはRS-Git
     (`GET /api/repos`)・aruaru-llm(`/healthz`・`/v1/chat`)との連携API
     実装済みであること、Bitbucket/Gitbucket/GitLab/標準Redmineは未検証
     であること、運用先は`easy-web.tokyo/RS-Ops`(未デプロイ)を予定して
     いることを正直に記載。`live_url`/`live_status`は共に`None`
     (runo.tokyo上に実ページが無く、easy-web.tokyoにも未デプロイのため)。
  3. **TOPページの`render_runo_link_section()`(「エコシステム全体を
     見る」節)にRS-Ops・RS-Git・aruaru-llmへの直接リンク付き説明文を
     追加**——これが「エコシステム全体を見る」での紹介に対応する。
     「RS-Ops(エコシステム全体マップの自動生成、姉妹リポジトリRS-Git・
     aruaru-llmとの「SET」連携)も、このプロジェクトシリーズ索引に
     含まれています。」という一文を追記し、3リポジトリのGitHubへ直接
     リンク。
  4. **新規テスト`rs_ops_is_introduced_in_ecosystem_section_and_project_series`
     を追加**(`src/lib.rs`)——TOPページのHTMLに`aon-co-jp/RS-Ops`・
     `aon-co-jp/RS-Git`・`aon-co-jp/aruaru-llm`へのリンクが含まれる
     こと、`/runo`ページのレンダリング結果にも`aon-co-jp/RS-Ops`が
     含まれることを確認する。
  5. **正直な開示**: このパスでは開発環境の`Shell`ツールが応答不能
     (コマンド実行結果が返らない、既知の一時的な環境問題)だったため、
     `cargo build`/`cargo test`による実際のビルド・テスト検証が
     **できなかった**——ソースコードは既存の`render_runo_link_section`・
     `PROJECTS`エントリ追加パターンをそのまま踏襲した変更のみで、型的
     には問題ないと判断しているが、次回セッションでShellツールが復旧
     した際に必ず`cargo test --workspace`(特に今回追加した
     `rs_ops_is_introduced_in_ecosystem_section_and_project_series`)を
     実行し、グリーンであることを確認してからデプロイすること。
  6. **easy-web.tokyo/RS-Opsでの本体運用は未着手**——ユーザー指示の
     「完成したらeasy-web.tokyo/RS-Opsで運用して」に対応する部分。
     これは`runo.tokyo`のコード変更だけでは実現できず、(a) RS-Ops
     バイナリを実際にVPS上でビルド・systemdサービス化、(b) nginxで
     `easy-web.tokyo`の443番serverブロックに`location /RS-Ops/`の
     `proxy_pass`を追加、が必要(既存の`/rs-git`等と同じパターン)。
     このパスではSSH/VPSアクセスを伴う操作を実行しておらず、次回
     セッションでの実施が必要。
  - 次にすべきこと: (1) Shellツール復旧後、`cargo test --workspace`で
    今回の変更を検証、(2) `easy-web.tokyo`側でのRS-Opsデプロイ
    (systemdサービス化+nginx `location /RS-Ops/`追加)、(3) デプロイ
    完了後、`meta_index.rs`の`RS-Ops`エントリの`live_url`/`live_status`を
    実際の運用先(`https://easy-web.tokyo/RS-Ops`)に更新。

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
- [open-cosmo](https://github.com/aon-co-jp/open-cosmo)(旧称open-runo、2026-07-20にGitHub上で改名) — Rust→WASM/tokio+hyperのopen-cosmoエコシステム本体
- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — 開発ルールの正本
