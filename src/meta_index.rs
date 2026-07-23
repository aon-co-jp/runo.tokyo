//! `/runo`ページ(「プロジェクトシリーズ索引」)。
//!
//! `F:\open-runo`エコシステム全体のメタ索引リポジトリ
//! [`RUNO`](https://github.com/aon-co-jp/RUNO)(旧称: `open-aruaru-runo-iLumi`
//! → `aon` → `RUNO`、いずれもユーザーがリポジトリ名を変更したもの、
//! 内容は移行済み)と同じ内容(プロジェクト一覧・
//! README/PORTING.md/CLAUDE.mdへのリンク)を
//! Webページとして表示する。GitHubのリポジトリファイル一覧のような
//! 見た目(📄アイコン+ファイル名+リンクのミニリスト)で各プロジェクトの
//! ファイルを提示し、クリック時にGitHub APIから最新情報(⭐Star数・
//! 最終更新日時・既定ブランチ)を取得するボタンを添える。
//!
//! 本ファイルは`aruaru-tokyo`リポジトリの`src/meta_index.rs`をそのまま
//! 移植したもの(ユーザー指示、2026-07-20)。移植にあたり、このリポジトリ
//! 自身(`runo.tokyo`)をプロジェクト一覧に追加し、メタリポジトリの最新名
//! `RUNO`に追従した。
//!
//! 13ヶ国語対応(`?lang=`)・言語別UIについては`crate::i18n`参照。
//!
//! **正直な開示**: 各プロジェクトの役割説明(`ROLE_JA`)は日本語のみで、
//! 13言語へは翻訳していない(スコープの限界、`crate::i18n`冒頭コメント
//! 参照)。

use crate::github_cache;
use crate::html_escape;
use crate::i18n::{index_strings, Lang};

/// メタ索引リポジトリ`RUNO`(旧称: `open-aruaru-runo-iLumi` → `aon`)の
/// README.mdに掲載した
/// プロジェクト一覧と全く同じ内容(名称・GitHubリポジトリ・各ファイルの
/// 有無・役割の要約)。`F:\open-runo`配下で実際にgitリポジトリとして
/// 存在し、`.git/config`のリモートURLで`aon-co-jp`上の実在を確認した
/// プロジェクトのみを掲載している(推測での追加はしていない)。
pub struct Project {
    pub name: &'static str,
    /// `aon-co-jp/<repo>`形式(GitHub APIの`owner/repo`)。
    pub repo: &'static str,
    pub has_readme: bool,
    pub has_porting: bool,
    pub has_claude: bool,
    pub role_ja: &'static str,
    /// ドメイン・サブドメイン・runo.tokyo配下のパスで実際にアクセス
    /// 可能な場合のURL。未デプロイ・GitHub上のコードのみの場合は`None`
    /// (ユーザー指示、2026-07-21: 「ドメインやサブドメインにてアクセス
    /// 出来るものは、そのURLも紹介して」)。
    pub live_url: Option<&'static str>,
    /// `live_url`の状態表示(例: "稼働中"・"開発途中")。`live_url`が
    /// `Some`の場合のみ意味を持つ。
    pub live_status: Option<&'static str>,
    /// **配布状況(2026-07-23追加、ユーザー指示「完成していればWindows版・
    /// LINUX版・Android版などをインストーラー付きでダウンロード可能に」)**。
    /// `None`の場合はライブラリ専用クレート等、配布物を持たないプロジェクト
    /// として「配布状況」セクション自体を描画しない。実際に各リポジトリの
    /// `install.sh`/`install.ps1`/`.github/workflows/release.yml`の実在と、
    /// `gh release list -R aon-co-jp/<repo>`で実際に公開されているGitHub
    /// Releaseの有無を確認した上で記載している(推測での「完成」表記はしない)。
    pub distribution: Option<Distribution>,
}

/// 1プロジェクトぶんの配布状況(Windows/Linuxインストーラー・Android対応)。
/// **正直な開示(自動検出を見送った理由)**: `GET /repos/<repo>/releases/
/// latest`をGitHub APIで叩けば個々のリポジトリのRelease実在は自動検出
/// できるが、未認証GitHub APIのレート制限は60req/時のみで、既存の
/// `github_cache.rs`が使う`GET /users/aon-co-jp/repos`(1〜2リクエストで
/// 全リポジトリ分の説明文等をまとめて取得)と異なり、Release確認は
/// リポジトリ数だけ個別リクエストが必要になる。配布状況(インストーラー
/// 有無)はGitHub側のREADME更新等と比べて更新頻度がはるかに低い
/// (実際に3点セット・Releaseを追加した瞬間だけ変化する)ため、既存の
/// 低頻度な自動更新枠を食い潰してまで毎時ポーリングする価値は薄いと
/// 判断し、静的な`Project`フィールドとして管理する方式を選んだ
/// (ユーザー指示「自動検出が難しい場合は静的なフィールド追加でも構わない
/// ——工学的判断で進めてよい」に基づく判断)。
#[derive(Clone, Copy)]
pub struct Distribution {
    /// Windows版インストーラーのダウンロードURL。実際にGitHub Releaseの
    /// 資産(`*-windows-x86_64.zip`等)が公開されている場合のみ`Some`
    /// (`https://github.com/<repo>/releases/latest`固定、
    /// 2026-07-23時点で`gh release view`実機確認済み)。
    pub windows_url: Option<&'static str>,
    /// Linux版インストーラーのダウンロードURL。条件はWindows版と同じ。
    pub linux_url: Option<&'static str>,
    /// Android対応状況の一言説明。`None`は「クロスコンパイル検証すら
    /// 未着手」、`Some`はその内容(「クロスコンパイル確認のみ」
    /// 「APK生成まで到達・実機/エミュレータ未検証」等)を正直に記載する。
    pub android_status: Option<&'static str>,
    /// Windows/Linuxとも未配布(installスクリプトはあるが実Releaseが
    /// まだ無い等)の場合に表示する状況説明。`windows_url`/`linux_url`が
    /// 両方`Some`なら通常は空文字列でよい。
    pub note: &'static str,
}

pub const PROJECTS: &[Project] = &[
    Project {
        name: "RPoem",
        repo: "aon-co-jp/RPoem",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "旧称poem-cosmo-tauri。WunderGraph Cosmoの有料版とWEB高速化技術が基本で、Tauriのブラウザ内実行機能とフロントエンド機能の実装＋Poemを一から実装。姉妹リポジトリRCosmoにPoem/Tauri機能を追加した版。2026-07-23、RCosmoと共通する「Cosmoコア」20クレートの重複を`diff -rq`で実調査し、`scripts/sync-cosmo-core.sh`(差分検出・push/pull同期)を新設——以後は手作業diffではなくこのスクリプトで同期を確認する運用に切り替えた。(開発開始日: 2026-07-18)",
        live_url: None,
        live_status: None,
        distribution: Some(Distribution {
            windows_url: Some("https://github.com/aon-co-jp/RPoem/releases/latest"),
            linux_url: Some("https://github.com/aon-co-jp/RPoem/releases/latest"),
            android_status: None,
            note: "",
        }),
    },
    Project {
        name: "RCosmo",
        repo: "aon-co-jp/RCosmo",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "旧称open-runo。WunderGraph Cosmoの有料版＋WEB高速化技術だけで、Tauriや、Poem機能はありません。独自の自己学習AIを搭載(外部LLM契約不要)。2026-07-23、姉妹リポジトリRPoemとの「Cosmoコア」共通20クレート中18件が完全一致していることを`diff -rq`で実調査し、`scripts/sync-cosmo-core.sh`(手作業diff不要の同期チェックスクリプト)を新設。Windowsネイティブcargo環境固有の実バグ(テストがUnix専用コマンド`sleep`をハードコードし起動自体が失敗していた不具合)も修正済み。(開発開始日: 2026-07-18)",
        live_url: None,
        live_status: None,
        distribution: Some(Distribution {
            windows_url: None,
            linux_url: None,
            android_status: None,
            note: "install.sh/install.ps1は未整備だが.github/workflows/release.ymlのみ先行整備済み(タグpushでの実CI検証はまだ未実施)",
        }),
    },
    Project {
        name: "open-cosmo",
        repo: "aon-co-jp/open-cosmo",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "旧称open-runo(2026-07-20にGitHub上でopen-cosmoへ改名)。WunderGraph Cosmoの有料版を含む全ての機能の完全無料のオープンソース版。WEBサーバー負荷やDBバックアップ分散統合機能など。姉妹リポジトリRPoem(旧poem-cosmo-tauri)と同時並行で開発されていた独立したプロジェクトで、RCosmoとは別のリポジトリ(RCosmoも過去に「open-runo」と呼ばれていた時期があり紛らわしいが、GitHub上のopen-cosmoリポジトリ自体はRCosmoとは異なる実体)。",
        live_url: None,
        live_status: None,
        distribution: None,
    },
    Project {
        name: "RFrontEnd",
        repo: "aon-co-jp/RFrontEnd",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "HTML5/CSS3/TypeScript/React相当を、既存実装のコードを一切流用せず一から開発する複数プロジェクト(RHTML/RCSS/RTypeScript等)を束ねる親リポジトリ。(開発開始日: 2026-07-18)",
        live_url: None,
        live_status: None,
        distribution: None,
    },
    Project {
        name: "RS-Git",
        repo: "aon-co-jp/RS-Git",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "Giteaが持つ機能のうち、まずgit clone/push・OTPログイン・アクセス制御・README閲覧機能だけを実装した、Go言語版GiteaのRust＋RPoem版です。元の機能を素直に移植致しました。ディスク容量に応じた新規リポジトリ作成可否判定も実装済み。Issue・Pull Request・Wiki・Webhookは未実装(READMEに明記)。(開発開始日: 2026-07-21)",
        live_url: Some("https://runo.tokyo/rs-git"),
        live_status: Some("稼働中"),
        distribution: Some(Distribution {
            windows_url: Some("https://github.com/aon-co-jp/RS-Git/releases/latest"),
            linux_url: Some("https://github.com/aon-co-jp/RS-Git/releases/latest"),
            android_status: None,
            note: "",
        }),
    },
    Project {
        name: "RS-JSON",
        repo: "aon-co-jp/RS-JSON",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "寛容(lenient)モードと厳密(strict)モードの両方を1つのクレートから提供するJSON処理ライブラリ。パース結果は常に標準の`serde_json::Value`。RS-GitのWASMフロントエンド向けに、`serde_json`非依存の軽量`light`モジュール(ブラウザの`JSON.parse`相当)も同梱。旧称RJSON/Rust-JSON(GitHub上の実リポジトリ名はRS-JSON)。(開発開始日: 2026-07-18)",
        live_url: None,
        live_status: None,
        distribution: None,
    },
    Project {
        name: "aruaru-db",
        repo: "aon-co-jp/aruaru-db",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "CockroachDBの分散強整合×Snowflakeのストレージ/コンピュート分離×Git-on-SQLバージョン管理を、すべてPure Rustで実装するハイブリッド分散データベース。2026-07-23、`install.sh`/`install.ps1`/`.github/workflows/release.yml`のインストーラー3点セットを追加(HTAPの行→列インクリメンタル同期〈TiFlash Delta Tree方式〉も同日実装)。(開発開始日: 2026-06-23)",
        live_url: None,
        live_status: None,
        distribution: Some(Distribution {
            windows_url: None,
            linux_url: None,
            android_status: None,
            note: "インストーラー3点セットは追加済みだが、v0.1.0タグのRelease自動ビルドがCIで失敗中(siblingリポジトリRS-JSONのpath依存が原因、修正済みだが再タグでの検証待ち)——現時点でダウンロード可能なバイナリ配布は無い",
        }),
    },
    Project {
        name: "aruaru-llm",
        repo: "aon-co-jp/aruaru-llm",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "aruaruエコシステム(aruaru-tokyo・aruaru-db・e-gov.info・karu.tokyo等)共通の「AIチャットコマース」応答サービス。Python向けのAIライブラリとLLMをRust向けの書き直しを開始しました。ベースになったのは、aruaru-llm(開発途中)＋open-cuda(Windows＋MAC＋LINUX互換 ＆ INTEL＋AMD＋nVIDIA互換を開発途中です。)。(開発開始日: 2026-07-18)",
        live_url: None,
        live_status: None,
        distribution: None,
    },
    Project {
        name: "aruaru-tokyo",
        repo: "aon-co-jp/aruaru.tokyo",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "aruaru.tokyoのTOPページ。Rust + Poem製、DB非依存・1バイナリ完結。audiocafe.tokyo(PHP)とは別ドメイン・別スタックの姉妹サイト。この「プロジェクトシリーズ索引」機能の実装元。(開発開始日: 2026-07-18)",
        live_url: Some("https://aruaru.tokyo/"),
        live_status: Some("稼働中"),
        distribution: None,
    },
    Project {
        name: "audiocafe-tokyo-rust",
        repo: "aon-co-jp/audiocafe-tokyo-rust",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "audiocafe.tokyoの既存PHPモノリスをRust + Poemへ段階的に移行するプロジェクト(第一段)。既存PHP実装はaudiocafe-tokyoリポジトリのまま並行運用。(開発開始日: 2026-07-17)",
        live_url: None,
        live_status: None,
        distribution: None,
    },
    Project {
        name: "audiocafe.tokyo",
        repo: "aon-co-jp/audiocafe-tokyo-php",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "PHP製のマルチコンテンツサイト。IT/建築系求人情報(aruaru)・女性向け求人/夜間エンターテインメント情報(aruaru-lady)・楽天モバイル関連情報・会社案内などを扱う。(開発開始日: 2026-07-13)",
        live_url: Some("https://audiocafe.tokyo/"),
        live_status: Some("稼働中"),
        distribution: None,
    },
    Project {
        name: "e-gov.info",
        repo: "aon-co-jp/e-gov",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "行政のデジタル化と、個人〜貿易商社まで対応するオンライン貿易・不動産プラットフォームを、LINEアプリ・WEBサイト・コンビニ端末という複数の入口から統合するプロジェクト。まだサンプル・デモンストレーション段階(READMEに明記)。(開発開始日: 2026-07-01)",
        live_url: Some("https://e-gov.info/"),
        live_status: Some("稼働中"),
        distribution: None,
    },
    Project {
        name: "karu.tokyo",
        repo: "aon-co-jp/karu-tokyo",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "karu.tokyoのTOPページ。Rust + Poem製、DB非依存の1バイナリ完結サーバー。軽井沢・あきる野市・東京の観光とリモートワーク、IT・AI・AUDIO・貿易産業を紹介。(開発開始日: 2026-07-17)",
        live_url: Some("https://karu.tokyo/"),
        live_status: Some("稼働中"),
        distribution: None,
    },
    Project {
        name: "open-cuda",
        repo: "aon-co-jp/open-cuda",
        has_readme: false,
        has_porting: false,
        has_claude: false,
        role_ja: "第二のCUDA。Windows＋MAC＋LINUX互換 ＆ INTEL＋AMD＋nVIDIA互換を開発途中です(OmniGPU設計文書等、README.mdの代わりにREADME-Japan.md等が存在、標準構成のCLAUDE.md/PORTING.mdは現時点で未整備)。(開発開始日: 2026-06-26)",
        live_url: None,
        live_status: None,
        distribution: None,
    },
    Project {
        name: "open-easy-web",
        repo: "aon-co-jp/open-easy-web",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "「第二のKUSANAGI」——アプリのアップロード後にIPアドレスで起動し、ドメイン登録・HTTPS化を簡単に自動適用できる運用ツール(Rust → WebAssembly、フレームワーク不使用)。2026-07-23、無料DDNS(DuckDNS)連携の「簡単ドメイン設定ウィザード」を追加(トークン入力・希望サブドメイン名入力・セットアップ&疎通確認ボタン、一覧/削除にも対応)——実DuckDNSトークンでの実接続検証は未実施と正直に開示。(開発開始日: 2026-07-17)",
        live_url: Some("https://easy-web.tokyo/"),
        live_status: Some("稼働中"),
        distribution: None,
    },
    Project {
        name: "open-raid-z",
        repo: "aon-co-jp/open-raid-z",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "Rust製の実マウント可能なRAID-Z/Z2/Z3ストレージプール実装(ZFS「風」のCoW/チェックサム/スナップショット)。エコシステム開発ルールの正本リポジトリ。2026-07-23、`install.sh`/`install.ps1`/`.github/workflows/release.yml`のインストーラー3点セットを追加(実体はネストした`open_runo_zfs_source/open_raid_z_core`クレート、`orzctl`バイナリを配布対象)。(開発開始日: 2026-07-07)",
        live_url: None,
        live_status: None,
        distribution: Some(Distribution {
            windows_url: None,
            linux_url: None,
            android_status: None,
            note: "インストーラー3点セットは追加済みだが、v0.1.0タグのRelease自動ビルドがCIで失敗中(Windows専用crateのfeature設定を修正済みだが、再タグでの検証待ち)——現時点でダウンロード可能なバイナリ配布は無い",
        }),
    },
    Project {
        name: "open-web-server",
        repo: "aon-co-jp/open-web-server",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "Rust + tokio/hyper自前実装のWebサーバー——課金アイテム・金融データを「消失させない」ために設計。open-cosmo・aruaru-dbと4層防御通信で連携するミッションクリティカル向け。2026-07-23、`russh`/`russh-sftp`による組み込みSFTPサーバー(公開鍵認証)・`igd-next`によるUPnP自動ポート開放・無料DDNS(DuckDNS)ドメイン自動化(最大20ドメイン)・CORSミドルウェア(`OPEN_WEB_SERVER_CORS_ALLOWED_ORIGINS`)を追加。Android版はKotlin/GradleシェルとAPK生成(`app-debug.apk`)まで到達したが、`adb`が`unauthorized`を返し続け実機/エミュレータでの動作確認は未完了(開発中であり配布可能ではない)。(開発開始日: 2026-07-04)",
        live_url: Some("https://runo.tokyo/open-web-server"),
        live_status: Some("開発途中(紹介ページのみ公開、本体サーバー化は準備中)"),
        distribution: Some(Distribution {
            windows_url: Some("https://github.com/aon-co-jp/open-web-server/releases/latest"),
            linux_url: Some("https://github.com/aon-co-jp/open-web-server/releases/latest"),
            android_status: Some("開発中(Kotlinシェル+APK生成まで到達、adb実機/エミュレータ確認は未完了——配布可能ではない)"),
            note: "",
        }),
    },
    Project {
        name: "rs-to-readme",
        repo: "aon-co-jp/rs-to-readme",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "Rustクレートの Cargo.toml メタデータからREADME.mdを自動生成するCLIツール(crates.io公開)。(開発開始日: 2026-07-10)",
        live_url: None,
        live_status: None,
        distribution: None,
    },
    Project {
        name: "runo.tokyo",
        repo: "aon-co-jp/runo.tokyo",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "runo.tokyoのTOPページ(このリポジトリ自身)。Rust + Poem製、DB非依存・1バイナリ完結。東京都西部各自治体へのリンク集・企業/工場/倉庫事業用地のドローン空撮紹介(準備中)を掲載し、この「プロジェクトシリーズ索引」ページ(`/runo`)自体もここで提供する。(開発開始日: 2026-07-08)",
        live_url: Some("https://runo.tokyo/"),
        live_status: Some("稼働中"),
        distribution: None,
    },
    Project {
        name: "RS-Red",
        repo: "aon-co-jp/RS-Red",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "旧称RS-Chiketto(2026-07-22にRS-Redへ改名)。REDMINEのRust＋RPoem版。チケット(Issue)管理・プロジェクト/サブプロジェクト階層・Wiki・コメント・OTPログイン・アクセス制御を実装、ブラウザGUI(Rust→WebAssembly)も新設済み。ハイスピード・ハイセキュリティ・省メモリが特徴。v0.1.0時点でRedmine全体の機能網羅率はまだ2〜3割程度(README/CLAUDE.mdに正直に明記)。2026-07-23、ガントチャート用フィールド(`start_date`/`due_date`/`done_ratio`)・チケット一覧のstatus/project_idフィルタAPI・DDNS運用対応(`open-web-server`のDDNS機構を移植)・`StorageBackend`抽象化(ローカル/SFTP/Google Driveを差し替え可能に設計、実SFTP/Google Drive環境での到達確認はまだ未実施)を追加。Androidは`cargo ndk`でのクロスコンパイル確認のみ(APK化は未着手)。(開発開始日: 2026-07-21、公開日: 2026-07-22、open-web-serverのpath_prefixルーティング経由)",
        live_url: Some("https://runo.tokyo/chiketto"),
        live_status: Some("公開中"),
        distribution: Some(Distribution {
            windows_url: Some("https://github.com/aon-co-jp/RS-Red/releases/latest"),
            linux_url: Some("https://github.com/aon-co-jp/RS-Red/releases/latest"),
            android_status: Some("クロスコンパイル確認のみ(`cargo ndk`でのビルド成功を確認、APK化・実機確認は未着手)"),
            note: "",
        }),
    },
    Project {
        name: "RS-Blog",
        repo: "aon-co-jp/RS-Blog",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "WordPressのRust＋RPoem版です。開発を開始。ハイスピードでハイセキュリティで省メモリなどの新規の特徴を持ちます。運用時は、VPSレンタルサーバー費用を安く抑えられる予定で御座います。(開発開始日: 2026-07-21、公開日: 2026-07-22、open-web-serverのpath_prefixルーティング経由)",
        live_url: Some("https://runo.tokyo/blog"),
        live_status: Some("公開中"),
        distribution: Some(Distribution {
            windows_url: Some("https://github.com/aon-co-jp/RS-Blog/releases/latest"),
            linux_url: Some("https://github.com/aon-co-jp/RS-Blog/releases/latest"),
            android_status: None,
            note: "",
        }),
    },
    Project {
        name: "RS-EC",
        repo: "aon-co-jp/RS-EC",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "EC-CUBEのRust＋RPoem版です。実際の決済連携(Stripe等)を目指します。(開発開始日: 2026-07-21、公開日: 2026-07-22、open-web-serverのpath_prefixルーティング経由。決済連携・SMTP未設定のためOTPログイン等は引き続き未検証)",
        live_url: Some("https://runo.tokyo/ec"),
        live_status: Some("公開中"),
        distribution: Some(Distribution {
            windows_url: Some("https://github.com/aon-co-jp/RS-EC/releases/latest"),
            linux_url: Some("https://github.com/aon-co-jp/RS-EC/releases/latest"),
            android_status: None,
            note: "",
        }),
    },
    Project {
        name: "RS-Ops",
        repo: "aon-co-jp/RS-Ops",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "旧称RS-AI-DevOps(2026-07-22改名)。「エコシステム全体マップの自動生成」の製品化。GitHub/RS-Chiketto/GitLab/Bitbucket/Redmineとの連携、Claude/Cursor/ChatGPT等向けコンテキストファイル(CLAUDE.md/.cursorrules/AGENTS.md)自動生成に加え、姉妹リポジトリRS-Git(`GET /api/repos`)・aruaru-llm(`/healthz`・`/v1/chat`)との連携APIを実装(RS-Ops・RS-Git・aruaru-llmの「SET」)。GitHub/RS-Chikettoのみ実機検証済み、Bitbucket/Gitbucket/GitLab/標準Redmineはインターフェースのみで正直に開示。運用先はeasy-web.tokyo/RS-Ops(未デプロイ、準備中)を予定。(開発開始日: 2026-07-22)",
        live_url: None,
        live_status: None,
        distribution: Some(Distribution {
            windows_url: None,
            linux_url: None,
            android_status: None,
            note: "install.sh/install.ps1(手動インストール手順)は整備済みだが、GitHub Actionsによる自動リリースビルド(release.yml)・公開Releaseはまだ無い",
        }),
    },
    Project {
        name: "RS-Guard",
        repo: "aon-co-jp/RS-Guard",
        has_readme: true,
        has_porting: false,
        has_claude: true,
        role_ja: "このエコシステム(aon-co-jp)のリポジトリ・依存パッケージに悪意あるコードが混入していないかを静的にチェックするサプライチェーンスキャナ。既存のClamAV等アンチウイルスを置き換えるものではなく併用する「ClamAV＋α」という位置づけ(独自のウイルス定義DBは持たない)。依存パッケージ名のブロックリスト照合・スパイウェア的挙動のコード片検出・破壊的操作/無断自動実行の検出・aruaru-llmによるAI二次判定・ClamAV連携(導入済み環境のみ)を実装、Windows/Linux版はruno.tokyo/RS-Guardで公開中。(開発開始日: 2026-07-22)",
        live_url: Some("https://runo.tokyo/RS-Guard"),
        live_status: Some("公開中(紹介・ダウンロードページ、easy-web.tokyo側のログイン後実運用画面は未着手)"),
        distribution: Some(Distribution {
            windows_url: None,
            linux_url: None,
            android_status: None,
            note: "install.sh/install.ps1(手動インストール手順)は整備済みだが、GitHub Actionsによる自動リリースビルド(release.yml)・公開Releaseはまだ無い",
        }),
    },
    Project {
        name: "RS-LinkFusion",
        repo: "aon-co-jp/RS-LinkFusion",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "複数のWAN/LAN/WiFi(古い規格〜WiFi 7まで)を1つの論理接続へ束ね(ボンディング)、通信の高速化・安定化を実現するアプリ。ポート単位の軽量トンネル(serve/connect)と、TUN仮想アダプタ方式のフルVPNゲートウェイ(gateway-serve/gateway-connect)の2方式を提供。ループバック環境での実データ転送・GPU(DirectX 12)アクセラレーションは実機検証済み、複数物理NICでの真のマルチホーミング効果はこの開発環境では未検証と正直に開示(README/CLAUDE.md参照)。インストーラー3点セット(install.sh/install.ps1/release.yml)は整備済み。(開発開始日: 2026-07-23)",
        live_url: None,
        live_status: None,
        distribution: Some(Distribution {
            windows_url: None,
            linux_url: None,
            android_status: None,
            note: "インストーラー3点セットは整備済みだが、バージョンタグをまだpushしていないためGitHub Releaseはまだ無い(タグpush後のCI検証が次のステップ)",
        }),
    },
    Project {
        name: "RS-SmartTCP",
        repo: "aon-co-jp/RS-SmartTCP",
        has_readme: true,
        has_porting: false,
        has_claude: true,
        role_ja: "IOWN/APN(NTTのオールフォトニクス・ネットワーク)のような超低遅延・ジッター無し回線と、Smart-TCP(AI生成通信プロトコル)の良いとこ取りハイブリッド適応制御ライブラリ。実測RTT・ジッターに基づき、TCP(RFC 6298)/QUIC(RFC 9002)と同じSRTT/RTTVAR(Jacobson/Karels EWMA)アルゴリズムでネットワーク品質を分類し、リトライ間隔をFast/Slowの2段階で切り替える。訓練済みMLモデルは使わない決定論的ヒューリスティックであり、実在の同名論文(arXiv 2512.00491)の実装そのものではないと正直に開示。RS-LinkFusion等から利用される。(開発開始日: 2026-07-23)",
        live_url: None,
        live_status: None,
        distribution: None,
    },
    Project {
        name: "RS-Sync",
        repo: "aon-co-jp/RS-Sync",
        has_readme: true,
        has_porting: false,
        has_claude: true,
        role_ja: "GitHub / RS-Git / Gitea / Gitbucket間で、アカウント登録・同期ジョブ設定(一方向/双方向)・スケジュール設定・手動実行ができるリポジトリ同期・バックアップWEBアプリ。VPS上の既存cronスクリプトを土台に汎用化。GitHubプロバイダのみ実機(実HTTP・実git push)で検証済み、RS-Git/Gitea/Gitbucketはドキュメントベースの実装のみで実機未検証と正直に開示。(開発開始日: 2026-07-22)",
        live_url: Some("https://runo.tokyo/rs-sync"),
        live_status: Some("稼働中(MVP、GitHubプロバイダのみ実機検証済み)"),
        distribution: Some(Distribution {
            windows_url: None,
            linux_url: None,
            android_status: None,
            note: "install.sh/install.ps1(手動インストール手順)は整備済みだが、GitHub Actionsによる自動リリースビルド(release.yml)・公開Releaseはまだ無い",
        }),
    },
    Project {
        name: "RS-Gitbucket",
        repo: "aon-co-jp/RS-Gitbucket",
        has_readme: true,
        has_porting: false,
        has_claude: true,
        role_ja: "GitBucket(Scala製セルフホスト型Gitプラットフォーム)相当の機能をRustで実装する構想のリポジトリ。既存のRS-Git(Git forge、OTPログイン・アクセス制御パターン先行実装済み)との役割分担を整理してから着手する方針で、2026-07-23時点ではコード未着手のプレースホルダー(README/CLAUDE.mdのみ)。",
        live_url: None,
        live_status: None,
        distribution: None,
    },
    Project {
        name: "runo-scanner",
        repo: "aon-co-jp/runo-scanner",
        has_readme: true,
        has_porting: false,
        has_claude: true,
        role_ja: "F:\\runo配下の複数プロジェクトを自動認識するRust製スキャナー。open-raid-z配下のruno-scan.txt(TOML形式のスキャンルール: 探索深度・除外ディレクトリ・プロジェクト判定マーカー・リポジトリ名/ドメイン名判定パターン・キャッシュ設定)を読み込み、F:\\runoを再帰的にスキャンしてプロジェクト一覧(パス・種別・マーカー)を出力する。スキャンルール自体の正本はopen-raid-z側が管理し、このリポジトリは実装のみを持つ。",
        live_url: None,
        live_status: None,
        distribution: None,
    },
    Project {
        name: "RUNO (旧称: open-aruaru-runo-iLumi → aon)",
        repo: "aon-co-jp/RUNO",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "このエコシステム全体の「プロジェクトシリーズ索引」を担うメタリポジトリ。個別のコード実装は持たない。open-aruaru-runo-iLumi → aon → RUNO の順にユーザーがリポジトリ名を変更(内容は都度移行済み)。(開発開始日: 2026-07-20)",
        live_url: None,
        live_status: None,
        distribution: None,
    },
];

fn file_line(repo_url: &str, filename: &str, label: &str, present: bool, none_label: &str) -> String {
    if present {
        format!(
            r#"<li>📄 <a href="{repo_url}/blob/main/{filename}" target="_blank" rel="noopener">{label}</a></li>"#,
            repo_url = repo_url,
            filename = filename,
            label = html_escape(label),
        )
    } else {
        format!(r#"<li>📄 <span class="file-missing">{label} {none}</span></li>"#, label = html_escape(label), none = html_escape(none_label))
    }
}

fn render_project_card(p: &Project, s: &crate::i18n::IndexStrings) -> String {
    let repo_url = format!("https://github.com/{}", p.repo);
    let files = format!(
        "{}\n      {}\n      {}",
        file_line(&repo_url, "README.md", s.label_readme, p.has_readme, s.label_none),
        file_line(&repo_url, "PORTING.md", s.label_porting, p.has_porting, s.label_none),
        file_line(&repo_url, "CLAUDE.md", s.label_claude, p.has_claude, s.label_none),
    );
    let live = match (p.live_url, p.live_status) {
        (Some(url), Some(status)) => format!(
            r#"<p class="live-link">🔗 <a href="{url}">{url}</a> <span class="pending">({status})</span></p>"#,
            url = url,
            status = html_escape(status),
        ),
        _ => String::new(),
    };
    // サーバー側で定期取得(既定1時間おき)したGitHub情報のキャッシュ
    // (`src/github_cache.rs`)。取得できていれば、説明文・最終更新日時・
    // スター数を「自動更新」の一言付きで表示する。未取得(起動直後・
    // GitHub API到達不能・レート制限等)の場合はこの行自体を出さず、
    // 静的な`role_ja`のみで表示する(補助機能の失敗が権威パスである
    // ページ描画をブロックしない設計)。
    let auto_sync = match github_cache::get(p.repo) {
        Some(info) => {
            let desc = info.description.as_deref().filter(|s| !s.is_empty());
            let stars = info.stargazers_count.map(|n| n.to_string()).unwrap_or_else(|| "?".to_string());
            let updated = info.pushed_at.as_deref().unwrap_or("?");
            format!(
                r#"<p class="auto-sync" data-repo-autosync="{repo}">🔄 <span class="pending">GitHub自動同期(サーバー側、1時間おき)</span>: {desc}最終更新 {updated} / ⭐ {stars}</p>"#,
                repo = html_escape(p.repo),
                desc = desc.map(|d| format!("{} / ", html_escape(d))).unwrap_or_default(),
                updated = html_escape(updated),
                stars = html_escape(&stars),
            )
        }
        None => String::new(),
    };
    // 「配布状況」ボックス(2026-07-23追加): Windows/Linuxインストーラーの
    // 実ダウンロードリンク・Android対応状況を表示する。`distribution`が
    // `None`(ライブラリ専用クレート等、配布物を持たないプロジェクト)の
    // 場合はこのセクション自体を描画しない(ユーザー指示)。
    let dist_box = match p.distribution {
        Some(d) => {
            let win_row = match d.windows_url {
                Some(url) => format!(r#"<a class="dist-btn" href="{url}" target="_blank" rel="noopener">🪟 Windows版をダウンロード</a>"#, url = url),
                None => String::new(),
            };
            let linux_row = match d.linux_url {
                Some(url) => format!(r#"<a class="dist-btn" href="{url}" target="_blank" rel="noopener">🐧 Linux版をダウンロード</a>"#, url = url),
                None => String::new(),
            };
            let android_row = match d.android_status {
                Some(status) => format!(r#"<p class="dist-android">🤖 Android: {status}</p>"#, status = html_escape(status)),
                None => String::new(),
            };
            let note_row = if d.note.is_empty() {
                String::new()
            } else {
                format!(r#"<p class="dist-note">{note}</p>"#, note = html_escape(d.note))
            };
            let no_binary_note = if d.windows_url.is_none() && d.linux_url.is_none() && d.note.is_empty() {
                r#"<p class="dist-note">配布可能なインストーラーは現時点ではありません(準備中)。</p>"#.to_string()
            } else {
                String::new()
            };
            format!(
                r#"<div class="dist-box">
    <strong>📦 配布状況 / Distribution</strong>
    <div class="dist-links">
      {win_row}
      {linux_row}
    </div>
    {android_row}
    {note_row}
    {no_binary_note}
  </div>"#,
                win_row = win_row,
                linux_row = linux_row,
                android_row = android_row,
                note_row = note_row,
                no_binary_note = no_binary_note,
            )
        }
        None => String::new(),
    };
    // 「プロジェクトのご利用」ボックス: ファイル説明(README/PORTING/CLAUDE)とは
    // 独立させ、利用に特化したリンクのみを日英併記で置く(ユーザー指示、
    // 2026-07-22)。
    let site_button = match p.live_url {
        Some(url) => format!(
            r#"<a class="usage-btn" href="{url}">🌐 サイトを開く / Open Site</a>"#,
            url = url,
        ),
        None => String::new(),
    };
    let usage_box = format!(
        r#"<div class="usage-box">
    <strong>🚀 プロジェクトのご利用 / Project Usage</strong>
    <p>このサイトで実際に使う、またはGitHubからダウンロードしてご利用いただけます。<br>Use it live on this site, or download it from GitHub.</p>
    <div class="usage-links">
      {site_button}
      <a class="usage-btn" href="{repo_url}" target="_blank" rel="noopener">⬇️ GitHubで入手 / Get on GitHub</a>
    </div>
  </div>"#,
        site_button = site_button,
        repo_url = repo_url,
    );
    format!(
        r#"<article class="repo-card" data-repo="{repo}">
  <h3><a href="{repo_url}" target="_blank" rel="noopener">📦 {name}</a></h3>
  <p class="role" data-repo-role="{repo}" data-fallback-role="{role}">{role}</p>
  {auto_sync}
  {live}
  {dist_box}
  {usage_box}
  <ul class="file-list">
      {files}
  </ul>
  <div class="live-fetch-row">
    <button type="button" class="live-fetch-btn" data-repo="{repo}">{btn_label}</button>
    <span class="live-fetch-result" data-repo-result="{repo}"></span>
  </div>
  <ul class="file-tree" data-repo-tree="{repo}"></ul>
</article>"#,
        repo = html_escape(p.repo),
        repo_url = repo_url,
        name = html_escape(p.name),
        role = html_escape(p.role_ja),
        auto_sync = auto_sync,
        live = live,
        dist_box = dist_box,
        usage_box = usage_box,
        files = files,
        btn_label = html_escape(s.btn_fetch_live),
    )
}

fn render_lang_switcher(current: Lang, path: &str) -> String {
    Lang::ALL
        .iter()
        .map(|&l| {
            let cur = if l == current { " class=\"current\"" } else { "" };
            format!(r#"<a href="{path}?lang={code}"{cur}>{name}</a>"#, path = path, code = l.code(), name = html_escape(l.native_name()))
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

pub const META_INDEX_STYLE: &str = r#"
  .meta-index main { max-width: 980px; }
  .lang-switch { text-align:center; font-size:.78rem; margin-bottom:1.5rem; line-height:2; }
  .lang-switch a { text-decoration:none; color:var(--muted); margin:0 .15rem; }
  .lang-switch a.current { color:var(--accent-2); font-weight:700; }
  .meta-intro { color:var(--muted); font-size:.9rem; margin-bottom:2rem; }
  .repo-card { background:var(--bg-card); border:1px solid var(--border); border-radius:.75rem; padding:1.1rem 1.4rem; margin-bottom:1.1rem; }
  .repo-card h3 { margin:0 0 .4rem; font-size:1.02rem; }
  .repo-card h3 a { text-decoration:none; }
  .repo-card p.role { color:var(--muted); font-size:.85rem; margin:0 0 .75rem; line-height:1.6; }
  .repo-card p.live-link { font-size:.85rem; margin:0 0 .75rem; }
  .repo-card p.live-link a { color:var(--accent-2); font-weight:600; }
  .repo-card p.auto-sync { font-size:.8rem; color:var(--muted); margin:0 0 .6rem; }
  .usage-box { background:rgba(47,111,237,.07); border:1px solid var(--border); border-radius:.6rem; padding:.75rem .9rem; margin:0 0 .9rem; font-size:.85rem; }
  .usage-box strong { font-size:.88rem; }
  .usage-box p { margin:.35rem 0 .6rem; color:var(--muted); line-height:1.5; }
  .usage-links { display:flex; gap:.5rem; flex-wrap:wrap; }
  .usage-btn { display:inline-block; text-decoration:none; background:var(--accent-2); color:#fff; border-radius:999px; padding:.35rem 1rem; font-size:.82rem; font-weight:600; }
  .dist-box { background:rgba(34,197,94,.08); border:1px solid var(--border); border-radius:.6rem; padding:.75rem .9rem; margin:0 0 .9rem; font-size:.85rem; }
  .dist-box strong { font-size:.88rem; }
  .dist-links { display:flex; gap:.5rem; flex-wrap:wrap; margin:.35rem 0; }
  .dist-btn { display:inline-block; text-decoration:none; background:#16a34a; color:#fff; border-radius:999px; padding:.35rem 1rem; font-size:.82rem; font-weight:600; }
  .dist-android { margin:.4rem 0 0; color:var(--muted); }
  .dist-note { margin:.4rem 0 0; color:var(--muted); font-size:.8rem; line-height:1.5; }
  ul.file-list { list-style:none; margin:0 0 .75rem; padding:0; font-family:"SFMono-Regular",Consolas,"Liberation Mono",Menlo,monospace; font-size:.82rem; }
  ul.file-list li { padding:.25rem 0; border-bottom:1px dashed var(--border); }
  ul.file-list li:last-child { border-bottom:none; }
  ul.file-list a { text-decoration:none; color:var(--accent-2); }
  .file-missing { color:var(--muted); font-style:italic; }
  .live-fetch-row { display:flex; align-items:center; gap:.6rem; flex-wrap:wrap; }
  .live-fetch-btn { font-size:.78rem; padding:.35rem .9rem; }
  .live-fetch-result { font-size:.78rem; color:var(--muted); }
  .live-fetch-result .ok { color:var(--fg); }
  ul.file-tree { list-style:none; margin:.6rem 0 0; padding:.6rem .8rem; font-family:"SFMono-Regular",Consolas,"Liberation Mono",Menlo,monospace; font-size:.8rem; background:rgba(148,163,184,.08); border-radius:.5rem; }
  ul.file-tree li { padding:.15rem 0; }
  ul.file-tree a { text-decoration:none; color:var(--accent-2); }
  ul.file-tree .tree-error { color:var(--muted); font-style:italic; }
"#;

/// `repo`(`owner/repo`)に一致する`Project`を1件だけ探す。
pub fn find_project(repo: &str) -> Option<&'static Project> {
    PROJECTS.iter().find(|p| p.repo == repo)
}

/// サブサービス個別ページ(`/chiketto`等)用に、単一プロジェクトだけを
/// GitHub風(README/PORTING.md/CLAUDE.mdへのリンク＋ファイル一覧自動取得)
/// で表示する軽量版ページ。`/runo`の全件一覧ページと同じ`render_project_card`
/// とJSロジックを再利用する(ユーザー指示、2026-07-21: リンク先で
/// README・設計思想＆開発方針＆開発環境ルール・お引越しポーターをGitHubの
/// ように読めるようにする)。
pub fn render_single_project_page(repo: &str, lang: Lang) -> Option<String> {
    let project = find_project(repo)?;
    let s = index_strings(lang);
    let dir = if lang.is_rtl() { "rtl" } else { "ltr" };
    let card = render_project_card(project, &s);

    Some(format!(
        r#"<!DOCTYPE html>
<html lang="{html_lang}" dir="{dir}">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{name} | runo.tokyo</title>
<style>{STYLE}{META_INDEX_STYLE}</style>
</head>
<body class="meta-index">
<main>
  <header><h1>{name}</h1></header>
  <p><a href="/">← runo.tokyo TOPへ戻る</a> ／ <a href="/runo">📚 プロジェクトシリーズ索引全体を見る</a></p>
  {card}
  <footer>&copy; 2026 runo.tokyo (Rust + Poem)</footer>
</main>
<script>
  async function fetchLiveInfo(repo, resultEl) {{
    resultEl.textContent = {loading_json};
    try {{
      const res = await fetch('https://api.github.com/repos/' + repo, {{ headers: {{ 'Accept': 'application/vnd.github+json' }} }});
      if (!res.ok) throw new Error('status ' + res.status);
      const data = await res.json();
      resultEl.innerHTML = '<span class="ok">' + {stars_label_json} + ': ' + data.stargazers_count +
        ' / ' + {updated_label_json} + ': ' + new Date(data.pushed_at || data.updated_at).toLocaleDateString() +
        ' / ' + {branch_label_json} + ': ' + data.default_branch + '</span>';
    }} catch (e) {{ resultEl.textContent = {fail_json}; }}
  }}
  async function fetchFileTree(repo, treeEl) {{
    try {{
      const res = await fetch('https://api.github.com/repos/' + repo + '/contents/', {{ headers: {{ 'Accept': 'application/vnd.github+json' }} }});
      if (!res.ok) throw new Error('status ' + res.status);
      const entries = await res.json();
      if (!Array.isArray(entries)) throw new Error('unexpected response shape');
      entries.sort((a, b) => {{ if (a.type !== b.type) return a.type === 'dir' ? -1 : 1; return a.name.localeCompare(b.name); }});
      treeEl.innerHTML = entries.map(e => {{
        const icon = e.type === 'dir' ? '📁' : '📄';
        const name = e.name.replace(/[&<>"']/g, c => ({{'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}})[c]);
        return '<li>' + icon + ' <a href="' + e.html_url + '" target="_blank" rel="noopener">' + name + '</a></li>';
      }}).join('');
    }} catch (e) {{ treeEl.innerHTML = '<li class="tree-error">' + {tree_fail_json} + '</li>'; }}
  }}
  // GitHubのREADME.md本文(先頭の意味のある段落)をその場で取得し、紹介文を
  // 常に最新のGitHubの内容と同期させる(ユーザー指示、2026-07-22:
  // 「クリックした瞬間に、githubの最新といつも同期を取って、いつも最新の
  // 内容を紹介して」)。取得できない場合はビルド時点の静的な紹介文
  // (data-fallback-role)のまま変更しない。
  function escapeHtmlLocal(s) {{ return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;"); }}
  async function fetchReadmeSummary(repo, roleEl) {{
    if (!roleEl) return;
    for (const branch of ['main', 'master']) {{
      try {{
        const res = await fetch('https://raw.githubusercontent.com/' + repo + '/' + branch + '/README.md');
        if (!res.ok) continue;
        const text = await res.text();
        const lines = text.split('\n');
        let summary = '';
        for (const line of lines) {{
          const t = line.trim();
          if (t === '' || t.startsWith('#') || t.startsWith('>') || t.startsWith('[![') || t.startsWith('![')) continue;
          summary = t;
          break;
        }}
        if (summary) {{
          roleEl.innerHTML = escapeHtmlLocal(summary) + ' <span class="pending">(GitHub README同期)</span>';
        }}
        return;
      }} catch (e) {{ /* try next branch */ }}
    }}
  }}
  document.querySelectorAll('.live-fetch-btn').forEach(btn => {{
    const repo = btn.getAttribute('data-repo');
    const resultEl = document.querySelector('.live-fetch-result[data-repo-result="' + repo + '"]');
    const treeEl = document.querySelector('.file-tree[data-repo-tree="' + repo + '"]');
    const roleEl = document.querySelector('.role[data-repo-role="' + repo + '"]');
    btn.addEventListener('click', () => {{ fetchLiveInfo(repo, resultEl); fetchFileTree(repo, treeEl); fetchReadmeSummary(repo, roleEl); }});
    setTimeout(() => {{ fetchLiveInfo(repo, resultEl); fetchFileTree(repo, treeEl); fetchReadmeSummary(repo, roleEl); }}, 100);
  }});
</script>
</body>
</html>
"#,
        html_lang = lang.html_lang(),
        dir = dir,
        name = html_escape(project.name),
        STYLE = crate::STYLE,
        META_INDEX_STYLE = META_INDEX_STYLE,
        card = card,
        loading_json = serde_json::to_string(s.fetch_loading).unwrap_or_default(),
        stars_label_json = serde_json::to_string(s.field_stars).unwrap_or_default(),
        updated_label_json = serde_json::to_string(s.field_updated).unwrap_or_default(),
        branch_label_json = serde_json::to_string(s.field_default_branch).unwrap_or_default(),
        fail_json = serde_json::to_string(s.fetch_fail).unwrap_or_default(),
        tree_fail_json = serde_json::to_string(s.fetch_fail).unwrap_or_default(),
    ))
}

/// `/open-aruaru-runo-iLumi`ページ本体のHTMLを組み立てる。
pub fn render_page(lang: Lang, canonical_path: &str) -> String {
    let s = index_strings(lang);
    let dir = if lang.is_rtl() { "rtl" } else { "ltr" };
    let cards = PROJECTS.iter().map(|p| render_project_card(p, &s)).collect::<Vec<_>>().join("\n  ");
    let lang_switch = render_lang_switcher(lang, canonical_path);

    format!(
        r#"<!DOCTYPE html>
<html lang="{html_lang}" dir="{dir}">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{title}</title>
<meta name="description" content="{intro}">
<style>{STYLE}{META_INDEX_STYLE}</style>
</head>
<body class="meta-index">
<main>
  <header>
    <h1>{h1}</h1>
  </header>

  <div class="lang-switch">{lang_switch}</div>

  <div class="org-link">
    <a href="https://github.com/aon-co-jp" target="_blank" rel="noopener">{org_link_label}</a>
  </div>

  <p class="meta-intro">{intro}</p>

  {cards}

  <div class="org-link">
    <a href="/">{back_to_top}</a>
  </div>

  <footer>&copy; 2026 runo.tokyo (Rust + Poem) — meta index for the open-cosmo ecosystem</footer>
</main>
<script>
  // 各リポジトリカードの「最新情報取得」を1関数に切り出す(手動クリックと
  // 自動一括実行の両方から呼べるようにする)。
  async function fetchLiveInfo(repo, resultEl) {{
    resultEl.textContent = {loading_json};
    try {{
      const res = await fetch('https://api.github.com/repos/' + repo, {{
        headers: {{ 'Accept': 'application/vnd.github+json' }}
      }});
      if (!res.ok) throw new Error('status ' + res.status);
      const data = await res.json();
      const stars = data.stargazers_count;
      const updated = data.pushed_at || data.updated_at;
      const branch = data.default_branch;
      resultEl.innerHTML = '<span class="ok">' + {stars_label_json} + ': ' + stars +
        ' / ' + {updated_label_json} + ': ' + new Date(updated).toLocaleDateString() +
        ' / ' + {branch_label_json} + ': ' + branch + '</span>';
    }} catch (e) {{
      resultEl.textContent = {fail_json};
    }}
  }}

  // リポジトリ直下のファイル/ディレクトリ一覧を、GitHub Contents API から
  // 取得し、GitHubの「Code」タブのファイル一覧そのままの見た目(📁/📄
  // アイコン+ファイル名+GitHub上の実ページへのリンク)で表示する
  // (ユーザー指示、2026-07-20)。公開リポジトリの内容取得なので未認証で
  // 取得可能・ログイン不要——リンク先も`github.com`の通常ページ
  // (`html_url`)であり、閲覧側もログイン不要。
  async function fetchFileTree(repo, treeEl) {{
    try {{
      const res = await fetch('https://api.github.com/repos/' + repo + '/contents/', {{
        headers: {{ 'Accept': 'application/vnd.github+json' }}
      }});
      if (!res.ok) throw new Error('status ' + res.status);
      const entries = await res.json();
      if (!Array.isArray(entries)) throw new Error('unexpected response shape');
      entries.sort((a, b) => {{
        if (a.type !== b.type) return a.type === 'dir' ? -1 : 1;
        return a.name.localeCompare(b.name);
      }});
      treeEl.innerHTML = entries.map(e => {{
        const icon = e.type === 'dir' ? '📁' : '📄';
        const name = e.name.replace(/[&<>"']/g, c => ({{'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}})[c]);
        return '<li>' + icon + ' <a href="' + e.html_url + '" target="_blank" rel="noopener">' + name + '</a></li>';
      }}).join('');
    }} catch (e) {{
      treeEl.innerHTML = '<li class="tree-error">' + {tree_fail_json} + '</li>';
    }}
  }}

  // GitHubのREADME.md本文(先頭の意味のある段落)をその場で取得し、紹介文を
  // 常に最新のGitHubの内容と同期させる(ユーザー指示、2026-07-22:
  // 「クリックした瞬間に、githubの最新といつも同期を取って、いつも最新の
  // 内容を紹介して」)。取得できない場合はビルド時点の静的な紹介文
  // (data-fallback-role)のまま変更しない。
  function escapeHtmlLocal(s) {{ return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;"); }}
  async function fetchReadmeSummary(repo, roleEl) {{
    if (!roleEl) return;
    for (const branch of ['main', 'master']) {{
      try {{
        const res = await fetch('https://raw.githubusercontent.com/' + repo + '/' + branch + '/README.md');
        if (!res.ok) continue;
        const text = await res.text();
        const lines = text.split('\n');
        let summary = '';
        for (const line of lines) {{
          const t = line.trim();
          if (t === '' || t.startsWith('#') || t.startsWith('>') || t.startsWith('[![') || t.startsWith('![')) continue;
          summary = t;
          break;
        }}
        if (summary) {{
          roleEl.innerHTML = escapeHtmlLocal(summary) + ' <span class="pending">(GitHub README同期)</span>';
        }}
        return;
      }} catch (e) {{ /* try next branch */ }}
    }}
  }}

  document.querySelectorAll('.live-fetch-btn').forEach(btn => {{
    const repo = btn.getAttribute('data-repo');
    const resultEl = document.querySelector('.live-fetch-result[data-repo-result="' + repo + '"]');
    const treeEl = document.querySelector('.file-tree[data-repo-tree="' + repo + '"]');
    const roleEl = document.querySelector('.role[data-repo-role="' + repo + '"]');
    btn.addEventListener('click', () => {{
      fetchLiveInfo(repo, resultEl);
      fetchFileTree(repo, treeEl);
      fetchReadmeSummary(repo, roleEl);
    }});
  }});

  // TOPページの「/runo」リンクをクリックしてこのページへ遷移してきた瞬間、
  // 手動でボタンを押さなくても全リポジトリの最新情報(⭐Star数・最終更新・
  // 既定ブランチ・README紹介文)と、GitHubそのままのファイル一覧を
  // 自動的に一括取得する(ユーザー指示、2026-07-20/2026-07-22)。
  // GitHub API(未認証)のレート制限に配慮し、同時発火ではなく400ms間隔で
  // 順に発火させる(このリポジトリ一覧はGitHub側のプロジェクト数が少なく
  // 現実的な待ち時間で完了する)。
  document.querySelectorAll('.repo-card').forEach((card, i) => {{
    const repo = card.getAttribute('data-repo');
    const resultEl = document.querySelector('.live-fetch-result[data-repo-result="' + repo + '"]');
    const treeEl = document.querySelector('.file-tree[data-repo-tree="' + repo + '"]');
    const roleEl = document.querySelector('.role[data-repo-role="' + repo + '"]');
    if (repo && resultEl && treeEl) {{
      setTimeout(() => {{
        fetchLiveInfo(repo, resultEl);
        fetchFileTree(repo, treeEl);
        fetchReadmeSummary(repo, roleEl);
      }}, i * 400);
    }}
  }});
</script>
</body>
</html>
"#,
        html_lang = lang.html_lang(),
        dir = dir,
        title = html_escape(s.title),
        intro = html_escape(s.intro),
        STYLE = crate::STYLE,
        META_INDEX_STYLE = META_INDEX_STYLE,
        h1 = html_escape(s.h1),
        lang_switch = lang_switch,
        org_link_label = html_escape(s.org_link_label),
        cards = cards,
        back_to_top = html_escape(s.back_to_top),
        loading_json = serde_json::to_string(s.fetch_loading).unwrap_or_default(),
        stars_label_json = serde_json::to_string(s.field_stars).unwrap_or_default(),
        updated_label_json = serde_json::to_string(s.field_updated).unwrap_or_default(),
        branch_label_json = serde_json::to_string(s.field_default_branch).unwrap_or_default(),
        fail_json = serde_json::to_string(s.fetch_fail).unwrap_or_default(),
        // ファイル一覧取得失敗時の表示も、既存の「取得失敗」文言を流用する
        // (専用の翻訳文字列を新設せず既存i18n資産を再利用、2026-07-20)。
        tree_fail_json = serde_json::to_string(s.fetch_fail).unwrap_or_default(),
    )
}
