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
}

pub const PROJECTS: &[Project] = &[
    Project {
        name: "RPoem",
        repo: "aon-co-jp/RPoem",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "旧称poem-cosmo-tauri。WunderGraph Cosmoの有料版とWEB高速化技術が基本で、Tauriのブラウザ内実行機能とフロントエンド機能の実装＋Poemを一から実装。姉妹リポジトリRCosmoにPoem/Tauri機能を追加した版。(開発開始日: 2026-07-18)",
        live_url: None,
        live_status: None,
    },
    Project {
        name: "RCosmo",
        repo: "aon-co-jp/RCosmo",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "旧称open-runo。WunderGraph Cosmoの有料版＋WEB高速化技術だけで、Tauriや、Poem機能はありません。独自の自己学習AIを搭載(外部LLM契約不要)。(開発開始日: 2026-07-18)",
        live_url: None,
        live_status: None,
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
    },
    Project {
        name: "RGit",
        repo: "aon-co-jp/RGit",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "Giteaが持つ機能のうち、まずgit clone/push・OTPログイン・アクセス制御・README閲覧機能だけを実装した、Go言語版GiteaのRust＋RPoem版です。元の機能を素直に移植致しました。ディスク容量に応じた新規リポジトリ作成可否判定も実装済み。Issue・Pull Request・Wiki・Webhookは未実装(READMEに明記)。(開発開始日: 2026-07-21)",
        live_url: Some("https://runo.tokyo/rgit"),
        live_status: Some("稼働中"),
    },
    Project {
        name: "RJSON",
        repo: "aon-co-jp/RJSON",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "寛容(lenient)モードと厳密(strict)モードの両方を1つのクレートから提供するJSON処理ライブラリ。パース結果は常に標準の`serde_json::Value`。RGitのWASMフロントエンド向けに、`serde_json`非依存の軽量`light`モジュール(ブラウザの`JSON.parse`相当)も同梱。(開発開始日: 2026-07-18)",
        live_url: None,
        live_status: None,
    },
    Project {
        name: "aruaru-db",
        repo: "aon-co-jp/aruaru-db",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "CockroachDBの分散強整合×Snowflakeのストレージ/コンピュート分離×Git-on-SQLバージョン管理を、すべてPure Rustで実装するハイブリッド分散データベース。(開発開始日: 2026-06-23)",
        live_url: None,
        live_status: None,
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
    },
    Project {
        name: "open-easy-web",
        repo: "aon-co-jp/open-easy-web",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "「第二のKUSANAGI」——アプリのアップロード後にIPアドレスで起動し、ドメイン登録・HTTPS化を簡単に自動適用できる運用ツール(Rust → WebAssembly、フレームワーク不使用)。(開発開始日: 2026-07-17)",
        live_url: Some("https://easy-web.tokyo/"),
        live_status: Some("稼働中"),
    },
    Project {
        name: "open-raid-z",
        repo: "aon-co-jp/open-raid-z",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "Rust製の実マウント可能なRAID-Z/Z2/Z3ストレージプール実装(ZFS「風」のCoW/チェックサム/スナップショット)。エコシステム開発ルールの正本リポジトリ。(開発開始日: 2026-07-07)",
        live_url: None,
        live_status: None,
    },
    Project {
        name: "open-web-server",
        repo: "aon-co-jp/open-web-server",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "Rust + tokio/hyper自前実装のWebサーバー——課金アイテム・金融データを「消失させない」ために設計。open-cosmo・aruaru-dbと4層防御通信で連携するミッションクリティカル向け。(開発開始日: 2026-07-04)",
        live_url: Some("https://runo.tokyo/open-web-server"),
        live_status: Some("開発途中(紹介ページのみ公開、本体サーバー化は準備中)"),
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
    },
    Project {
        name: "RS-Chiketto",
        repo: "aon-co-jp/RS-Chiketto",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "REDMINEのRust＋RPoem版を中心に開発を開始。ハイスピードでハイセキュリティで省メモリなどの新規の特徴を持ちます。まだ開発途中ですが、運用時は、VPSレンタルサーバー費用を安く抑えられる予定で御座います。(開発開始日: 2026-07-21)",
        live_url: Some("https://runo.tokyo/chiketto"),
        live_status: Some("開発途中"),
    },
    Project {
        name: "RS-Blog",
        repo: "aon-co-jp/RS-Blog",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "WordPressのRust＋RPoem版です。開発を開始。ハイスピードでハイセキュリティで省メモリなどの新規の特徴を持ちます。まだ開発途中ですが、運用時は、VPSレンタルサーバー費用を安く抑えられる予定で御座います。(開発開始日: 2026-07-21)",
        live_url: Some("https://runo.tokyo/blog"),
        live_status: Some("開発途中"),
    },
    Project {
        name: "RS-EC",
        repo: "aon-co-jp/RS-EC",
        has_readme: true,
        has_porting: true,
        has_claude: true,
        role_ja: "EC-CUBEのRust＋RPoem版です。新規開発開始。実際の決済連携(Stripe等)を目指します。(開発開始日: 2026-07-21)",
        live_url: Some("https://runo.tokyo/ec"),
        live_status: Some("開発途中"),
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
  {live}
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
        live = live,
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
  .usage-box { background:rgba(47,111,237,.07); border:1px solid var(--border); border-radius:.6rem; padding:.75rem .9rem; margin:0 0 .9rem; font-size:.85rem; }
  .usage-box strong { font-size:.88rem; }
  .usage-box p { margin:.35rem 0 .6rem; color:var(--muted); line-height:1.5; }
  .usage-links { display:flex; gap:.5rem; flex-wrap:wrap; }
  .usage-btn { display:inline-block; text-decoration:none; background:var(--accent-2); color:#fff; border-radius:999px; padding:.35rem 1rem; font-size:.82rem; font-weight:600; }
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
