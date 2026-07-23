//! runo.tokyo Webサイト。
//!
//! 【現状のスコープ】
//! - open-aruaru/open-runo系列アプリの紹介ページ(アプリ名・説明・GitHubリンク)
//! - 東京都西部各自治体(あきる野市・旧五日市・桧原村・青梅・奥多摩・羽村・
//!   福生・昭島・立川・国立・八王子・高尾・日の出町など)の役所・観光案内
//!   ホームページへのリンク集
//! - 企業・工場・倉庫事業用地の空き状況をドローン空撮しYouTubeで紹介する
//!   機能(実際の動画は未収録のため、準備中のプレースホルダーとして掲載)
//! - `/runo` — エコシステム全体のメタ索引リポジトリ`RUNO`(旧称:
//!   open-aruaru-runo-iLumi → aon → RUNO)と同内容の「プロジェクトシリーズ
//!   索引」ページ(`aruaru-tokyo`の`meta_index.rs`/`i18n.rs`を移植、13言語対応)
//!
//! 【未実装(今後の拡張、要件メモ参照)】
//! - AI辞書のGoogleドライブ連携(登録・ダウンロード・読み書きをユーザー自身の
//!   Googleドライブ上で行う)
//! - PostgreSQL + aruaru-dbのデュアルDB構成
//! - バージョンレス/バージョン管理/Git管理のトライブリッド方式
//! - Tauri 2.0デスクトップ版
//! - ドローン空撮の実際の動画・YouTubeチャンネル(現状はプレースホルダー)
//!
//! これらは設計(特にGoogle OAuth連携の認可フロー、デュアルDBの整合性モデル)
//! を詰めてから着手する。
//!
//! 【`readme-to-rs`について(正直な開示)】
//! 以前このリポジトリには`readme-to-rs`をpath依存で使うWeb版変換機能が
//! あったが、依存先`F:\open-runo\open-runo\readme-to-rs`が実在しないため
//! ビルドが壊れていた。実在確認の結果、当該パスは存在しないことを確認した
//! ため、この機能は削除した(`Cargo.toml`のpath依存も削除済み)。
//! `readme-to-rs`自体はGitHub上に独立リポジトリとして存在するため、
//! アプリ紹介一覧(`family_apps`)にはそのGitHubリンクを引き続き掲載する。

use poem::web::{Html, Query};
use poem::{get, handler, IntoResponse, Route};
use serde::{Deserialize, Serialize};

mod github_cache;
mod i18n;
mod meta_index;

/// アプリ起動時に呼ぶバックグラウンドタスク一式(GitHub APIからの
/// プロジェクト詳細情報の定期取得等)。`build_app()`から独立させている
/// のは、テストで`build_app()`を呼ぶたびにバックグラウンドタスクが
/// 何本も起動してしまうのを避けるため(`main.rs`からのみ呼び出す)。
pub fn spawn_background_tasks() {
    github_cache::spawn_background_refresh();
}

pub(crate) const STYLE: &str = r#"
  :root { color-scheme: light dark; --bg:#fdf6ec; --bg-card:#ffffff; --fg:#2c2418; --muted:#8a7f6b; --accent:#ff7a45; --accent-2:#2f6fed; --border:#eadfc9; }
  @media (prefers-color-scheme: dark) { :root { --bg:#1a1712; --bg-card:#24201a; --fg:#f1ece0; --muted:#b3a893; --accent:#ff9466; --accent-2:#6ea1ff; --border:#3a3327; } }
  * { box-sizing: border-box; }
  body { margin:0; font-family:"Hiragino Sans","Noto Sans JP",system-ui,sans-serif; background:var(--bg); color:var(--fg); line-height:1.7; }
  a { color:var(--fg); }
  a:visited { color:var(--fg); }
  main { max-width:820px; margin:0 auto; padding:2.5rem 1.25rem 5rem; }
  header { text-align:center; margin-bottom:1.25rem; }
  header h1 { font-size:2rem; margin:0 0 .4rem; letter-spacing:-.02em; }
  header h1 span { color:var(--accent); }
  header p { color:var(--muted); margin:0; }
  .blog-link { text-align:center; font-size:.85rem; margin:0 0 1rem; }
  .blog-link a { color:var(--accent-2); text-decoration:none; font-weight:600; }
  .quick-links { text-align:center; margin-bottom:2rem; }
  .quick-links a { display:inline-block; text-decoration:none; font-weight:600; background:var(--accent-2); color:#fff; border-radius:999px; padding:.55rem 1.4rem; margin:.2rem; font-size:.9rem; }
  section.block { background:var(--bg-card); border:1px solid var(--border); border-radius:.9rem; padding:1.25rem 1.75rem; margin:2rem 0; }
  section.block h2 { font-size:1.1rem; margin:0 0 .75rem; color:var(--accent-2); }
  section.block p { margin:0 0 1rem; }
  section.block p:last-child { margin-bottom:0; }
  .app-list, .muni-list { margin:0; padding-left:1.2rem; }
  .app-list li, .muni-list li { margin-bottom:.6rem; }
  .muni-group { margin-bottom:1.1rem; }
  .muni-group h3 { font-size:.92rem; margin:0 0 .4rem; color:var(--muted); }
  .muni-group ul { margin:0; padding-left:1.2rem; }
  .muni-group li { margin-bottom:.35rem; }
  .pending { color:var(--muted); font-style:italic; }
  .search-links { font-size:.85rem; color:var(--muted); margin:.3rem 0 1.1rem; }
  .search-links a { color:var(--accent-2); text-decoration:none; font-weight:600; }
  .drone-placeholder { background:rgba(148,163,184,.12); border:1px dashed var(--border); border-radius:.6rem; padding:1rem 1.25rem; color:var(--muted); font-size:.9rem; }
  .org-link { text-align:center; margin:2rem 0; }
  .org-link a { color:var(--accent-2); font-weight:600; text-decoration:none; }
  footer { text-align:center; color:var(--muted); font-size:.8rem; margin-top:3rem; }
"#;

pub(crate) fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[derive(Debug, Clone, Serialize)]
pub struct AppEntry {
    pub name: &'static str,
    pub description: &'static str,
    pub github_url: &'static str,
}

/// open-aruaru/open-runo系列アプリの一覧(アプリ紹介機能のデータ源)。
pub fn family_apps() -> Vec<AppEntry> {
    vec![
        AppEntry {
            name: "open-raid-z",
            description: "NTFS/exFATほぼ互換 + RAID全系列(0/1/5/6/10/Z2/Z3)対応の実験的ファイルシステム",
            github_url: "https://github.com/aon-co-jp/open-raid-z",
        },
        AppEntry {
            name: "ai-ime-runo",
            description: "前後の文脈から変換候補をAIが用意する日本語IME(ひな型)",
            github_url: "https://github.com/aon-co-jp/ai-ime-runo",
        },
        AppEntry {
            name: "readme-to-rs",
            description: "READMEをRustのドキュメントコメント付きモジュールへ変換するツール",
            github_url: "https://github.com/aon-co-jp/readme-to-rs",
        },
        AppEntry {
            name: "aruaru-db",
            description: "Pure Rustの分散ハイブリッドデータベース(Raft強整合 + Git-on-SQL)",
            github_url: "https://github.com/aon-co-jp/aruaru-db",
        },
    ]
}

/// 自治体・観光案内サイトへのリンク1件分。`url`が`None`の場合は
/// URL未提供のため「準備中」表示にする(推測でURLを埋めないこと、
/// ユーザー指示・2026-07-20)。
pub struct MuniLink {
    pub label: &'static str,
    pub url: Option<&'static str>,
    pub note: Option<&'static str>,
}

/// 自治体グループ(市区町村単位でまとめたリンク群)。
pub struct MuniGroup {
    pub area: &'static str,
    pub links: &'static [MuniLink],
}

/// 東京都西部各自治体へのリンク集。実在確認済みのURLのみ掲載し、
/// ユーザーから提供されていないURLは推測せず「準備中」と明記する
/// (ユーザー指示、2026-07-20および同日の追加提供分)。
pub const MUNICIPALITY_LINKS: &[MuniGroup] = &[
    MuniGroup {
        area: "あきる野市・旧五日市町",
        links: &[
            MuniLink { label: "あきる野市(公式)", url: Some("https://www.city.akiruno.tokyo.jp/"), note: None },
            MuniLink { label: "あきる野市観光案内", url: Some("https://www.city.akiruno.tokyo.jp/kanko/"), note: None },
            MuniLink {
                label: "旧五日市町",
                url: None,
                note: Some("独立した公式サイトはなく、あきる野市に統合済み。上記あきる野市のリンクで代替"),
            },
        ],
    },
    MuniGroup {
        area: "桧原村",
        links: &[
            MuniLink { label: "桧原村(公式)", url: None, note: Some("準備中") },
            MuniLink { label: "桧原村観光案内", url: Some("https://hinohara-kankou.jp/"), note: None },
        ],
    },
    MuniGroup {
        area: "青梅市",
        links: &[MuniLink { label: "青梅市(公式)", url: Some("https://www.city.ome.tokyo.jp/"), note: None }],
    },
    MuniGroup {
        area: "奥多摩町",
        links: &[
            MuniLink { label: "奥多摩町(公式)", url: Some("https://www.town.okutama.tokyo.jp/index.html"), note: None },
            MuniLink { label: "奥多摩観光案内", url: Some("https://www.okutama.gr.jp/site/"), note: None },
        ],
    },
    MuniGroup {
        area: "高尾山・八王子市",
        links: &[
            MuniLink { label: "八王子市役所(公式)", url: Some("https://www.city.hachioji.tokyo.jp/"), note: None },
            MuniLink { label: "高尾山観光案内", url: Some("https://www.hkc.or.jp/takaosan/"), note: None },
            MuniLink {
                label: "高尾山・八王子市観光案内",
                url: Some("https://tokyotouristinfo.com/detail/M0021"),
                note: None,
            },
            MuniLink { label: "高尾登山電鉄", url: Some("https://www.takaotozan.co.jp/"), note: None },
        ],
    },
    MuniGroup {
        area: "羽村市",
        links: &[MuniLink { label: "羽村市役所(公式)", url: Some("https://www.city.hamura.tokyo.jp/"), note: None }],
    },
    MuniGroup {
        area: "福生市",
        links: &[MuniLink { label: "福生市役所(公式)", url: Some("https://www.city.fussa.tokyo.jp/"), note: None }],
    },
    MuniGroup {
        area: "福生市＆昭島市",
        links: &[
            MuniLink {
                label: "拝島駅(西武鉄道)",
                url: Some("https://www.seiburailway.jp/railway/station/haijima/"),
                note: Some("アクセス"),
            },
            MuniLink {
                label: "拝島駅(JR東日本)",
                url: Some("https://www.jreast.co.jp/estation/stations/1213.html"),
                note: Some("アクセス"),
            },
        ],
    },
    MuniGroup {
        area: "昭島市",
        links: &[MuniLink { label: "昭島市役所(公式)", url: Some("https://www.city.akishima.lg.jp/"), note: None }],
    },
    MuniGroup {
        area: "立川市",
        links: &[MuniLink { label: "立川市役所(公式)", url: Some("https://www.city.tachikawa.lg.jp/"), note: None }],
    },
    MuniGroup {
        area: "国立市",
        links: &[MuniLink { label: "国立市役所(公式)", url: Some("https://www.city.kunitachi.tokyo.jp/index.html"), note: None }],
    },
    MuniGroup {
        area: "日の出町",
        links: &[MuniLink { label: "日の出町役場(公式)", url: Some("https://www.town.hinode.tokyo.jp/"), note: None }],
    },
];

/// ユーザー確定済みの紹介文(そのまま使用、改変しない)。
const INTRO_PARAGRAPHS: &[&str] = &[
    "都心から程よい近さの、地方郊外。安くて広い土地を見つけて、巨大な壁埋め込みスピーカー搭載のシアタールーム付き一戸建て注文住宅を建てる——そんな暮らし方はいかがでしょうか。",
    "素敵な音楽を聴きながら、あるいは素敵な映画を観ながら、オンラインPCで快適にテレワーク・リモートワークをこなす毎日。時にはオーディオカフェのようなカフェレストランにノートパソコンを持参して、テレワーク・リモートワークをする生活も、また格別です。",
    "親子で楽しくパソコンを教え合いながら過ごすテレワーク・リモートワークの時間もおすすめです。気になることがあれば、その場でGoogle画像検索を使って、親子で一緒に調べてみましょう。",
    "スマホやタブレット、PCのキーボードでGoogle検索やYouTube検索、ChatGPTへ質問ができるくらいのスキルからでも大丈夫です。Claude Code Desktopを使えば、IT経験が浅くてもアプリやWebサイトの開発が可能になり、フリーランスプログラマーのお仕事もリモートワークでこなせるようになります。",
];

const GOOGLE_IMAGE_SEARCH_URL: &str = "https://www.google.com/imghp";

/// ユーザー提供のブログ記事(タイトルをリンクテキストにし、URLそのものは
/// 表示しない、2026-07-20追記。姉妹サイトaruaru.tokyoにも同じリンクを掲載)。
const BLOG_POST_URL: &str = "https://ameblo.jp/www-aon/entry-12973252437.html";
const BLOG_POST_TITLE_JA: &str = "プログラム言語やフレームワークなどの全てをRust(Poemやhyper)版に移植するメリット?";
/// ユーザー指示(2026-07-20)により日英両方で掲載。ブログ本文自体は
/// 日本語のみだが、リンクのラベルは英語話者にも内容が伝わるよう
/// 意訳したもの(URLは日英共通、リンク先は変えない)。
const BLOG_POST_TITLE_EN: &str = "The benefits of migrating everything — programming languages, frameworks, and more — to Rust";

/// 各段落のテーマに沿った検索クエリ(ユーザー提示の例をそのまま使用)。
/// 段落0(注文住宅・シアタールーム・地方移住)と段落3(Claude Code)は
/// 2つのクエリ例が提示されているため、それぞれ2セット掲載する。
fn intro_queries_for_paragraph(i: usize) -> &'static [&'static str] {
    match i {
        0 => &["注文住宅 シアタールーム 壁埋め込みスピーカー", "地方 郊外 移住 テレワーク 一戸建て"],
        1 => &["テレワーク リモートワーク カフェ ノートパソコン"],
        2 => &["親子 パソコン教室 テレワーク"],
        3 => &["Claude Code Desktop 使い方", "Claude Code フリーランス プログラマー"],
        _ => &[],
    }
}

/// クエリ文字列をURLのクエリパラメータ用にパーセントエンコードする
/// (RFC 3986の非予約文字`A-Za-z0-9-_.~`以外をすべて`%XX`化、UTF-8バイト単位)。
fn percent_encode_query(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(*b as char),
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

/// 1クエリ分の「画像検索・Google検索・YouTube検索」3点セットのリンクを
/// 生成する。
fn render_search_link_set(query: &str) -> String {
    let q = percent_encode_query(query);
    format!(
        r##"<p class="search-links">🔗 「{label}」で調べる: <a href="https://www.google.com/search?tbm=isch&q={q}" target="_blank" rel="noopener">🖼️ 画像検索</a> / <a href="https://www.google.com/search?q={q}" target="_blank" rel="noopener">🔎 Google検索</a> / <a href="https://www.youtube.com/results?search_query={q}" target="_blank" rel="noopener">▶️ YouTube検索</a></p>"##,
        label = html_escape(query),
        q = q,
    )
}

/// サイトのキャッチコピー(ユーザー提示の文言そのまま、改変しない)。
/// 「巨大な壁埋め込みスピーカー」の暮らし(段落0)とは別に、あきる野市・
/// 多摩地域から東京・軽井沢・世界をつなぐ「AUDIOを聴きながらのテレワーク」
/// という切り口を、検索ワードそのものとしてGoogle検索・YouTube検索の
/// 結果へ誘導する(2026-07-20追記)。
const CATCHCOPY: &str =
    "東京都あきる野市と多摩地域周辺の東京と軽井沢と世界をつなぎ、郊外でAUDIOでも心地良く聴きながらリモートワークやテレワークする生活はいかがですか？";

/// 1クエリ分の「Google検索・YouTube検索」2点セット(画像検索は含まない)。
/// キャッチコピーのような長い文章クエリ向け(2026-07-20追記)。
fn render_google_youtube_link_set(query: &str) -> String {
    let q = percent_encode_query(query);
    format!(
        r##"<p class="search-links">🔗 「{label}」で調べる: <a href="https://www.google.com/search?q={q}" target="_blank" rel="noopener">🔎 Google検索</a> / <a href="https://www.youtube.com/results?search_query={q}" target="_blank" rel="noopener">▶️ YouTube検索</a></p>"##,
        label = html_escape(query),
        q = q,
    )
}

/// 1クエリ分の「画像検索・YouTube検索」2点セット(通常のGoogle検索は
/// 含まない)。写真・映像で見せたいテーマ(親子でのテレワーク風景・
/// 巨大なスピーカーなど)向け(2026-07-20追記)。
fn render_image_and_youtube_link_set(query: &str) -> String {
    let q = percent_encode_query(query);
    format!(
        r##"<p class="search-links">🔗 「{label}」で調べる: <a href="https://www.google.com/search?tbm=isch&q={q}" target="_blank" rel="noopener">🖼️ 画像検索</a> / <a href="https://www.youtube.com/results?search_query={q}" target="_blank" rel="noopener">▶️ YouTube検索</a></p>"##,
        label = html_escape(query),
        q = q,
    )
}

/// TOPページ冒頭のキャッチコピー・検索誘導セクション(ユーザー指示、
/// 2026-07-20)。
/// - キャッチコピー本文 → Google検索・YouTube検索リンク
/// - 「親子でパソコンを教えながらテレワークする」写真 → 画像検索・YouTube検索リンク
/// - 「巨大なスピーカー」 → 画像検索・YouTube検索リンク
fn render_catchcopy_section() -> String {
    format!(
        r#"<section class="block" id="catchcopy"><h2>runo.tokyoって、どんな場所？</h2><p>{catchcopy}</p>{catchcopy_links}<p>親子でパソコンを教え合いながらテレワーク・リモートワークをする、そんな時間の過ごし方も。</p>{family_links}<p>巨大な壁埋め込みスピーカーで、心地よい音に包まれながら暮らす一戸建て。</p>{speaker_links}</section>"#,
        catchcopy = html_escape(CATCHCOPY),
        catchcopy_links = render_google_youtube_link_set(CATCHCOPY),
        family_links = render_image_and_youtube_link_set("親子 パソコンを教える テレワーク 写真"),
        speaker_links = render_image_and_youtube_link_set("巨大なスピーカー"),
    )
}

fn render_intro_section() -> String {
    let mut html = String::from(r#"<section class="block" id="intro"><h2>こんな暮らし方はいかがでしょうか</h2>"#);
    for (i, p) in INTRO_PARAGRAPHS.iter().enumerate() {
        html.push_str(&format!("<p>{}</p>", html_escape(p)));
        for query in intro_queries_for_paragraph(i) {
            html.push_str(&render_search_link_set(query));
        }
        // 3番目の段落(親子でのGoogle画像検索の話)の直後に、汎用の
        // Google画像検索トップページへのリンクも併せて残す。
        if i == 2 {
            html.push_str(&format!(
                r#"<p>👉 <a href="{url}" target="_blank" rel="noopener">Google画像検索を開く(汎用)</a>(親子で一緒に調べ物をするときに)</p>"#,
                url = GOOGLE_IMAGE_SEARCH_URL,
            ));
        }
    }
    html.push_str("</section>");
    html
}

fn render_app_directory_section() -> String {
    let mut html = String::from(r#"<section class="block" id="apps"><h2>open-aruaru / open-cosmo シリーズ</h2><ul class="app-list">"#);
    for app in family_apps() {
        html.push_str(&format!(
            "<li><strong>{}</strong>: {} (<a href=\"{}\" target=\"_blank\" rel=\"noopener\">GitHub</a>)</li>",
            html_escape(app.name),
            html_escape(app.description),
            app.github_url
        ));
    }
    html.push_str("</ul></section>");
    html
}

fn render_municipality_section() -> String {
    let mut html = String::from(
        r#"<section class="block" id="municipalities"><h2>東京都西部 自治体・観光案内リンク集</h2><p>あきる野市・旧五日市・桧原村・青梅・奥多摩・羽村・福生・昭島・立川・国立・八王子・高尾・日の出町など、各自治体の公式サイト・観光案内サイトへのリンクです。URLが未提供の自治体は、推測で埋めず「準備中」と正直に表示しています。</p>"#,
    );
    for group in MUNICIPALITY_LINKS {
        html.push_str(&format!(r#"<div class="muni-group"><h3>{}</h3><ul>"#, html_escape(group.area)));
        for link in group.links {
            let line = match (link.url, link.note) {
                (Some(url), None) => format!(
                    r#"<li><a href="{url}" target="_blank" rel="noopener">{label}</a></li>"#,
                    url = url,
                    label = html_escape(link.label)
                ),
                (Some(url), Some(note)) => format!(
                    r#"<li><a href="{url}" target="_blank" rel="noopener">{label}</a> <span class="pending">({note})</span></li>"#,
                    url = url,
                    label = html_escape(link.label),
                    note = html_escape(note)
                ),
                (None, Some(note)) => format!(
                    r#"<li>{label} <span class="pending">({note})</span></li>"#,
                    label = html_escape(link.label),
                    note = html_escape(note)
                ),
                (None, None) => format!(r#"<li>{label} <span class="pending">(準備中)</span></li>"#, label = html_escape(link.label)),
            };
            html.push_str(&line);
        }
        html.push_str("</ul></div>");
    }
    html.push_str("</section>");
    html
}

fn render_drone_section() -> String {
    r#"<section class="block" id="drone"><h2>企業・工場・倉庫事業用地のドローン空撮紹介</h2>
<p>企業・工場・倉庫事業用地の空き状況を実際にドローンで空撮し、YouTubeで紹介する機能を準備しています。</p>
<div class="drone-placeholder">🚁 準備中: 実際の空撮動画・YouTubeチャンネルは現時点では未収録・未公開です。撮影・公開が完了し次第、ここに動画へのリンクを掲載します。</div>
</section>"#
        .to_string()
}

/// runo.tokyo配下のパスで公開している自作サブサービス1件分。
pub struct SubService {
    pub path: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub status: &'static str,
    /// `owner/repo`形式。READMEをGitHubから読める自前ページを持つ場合に`Some`。
    pub github_repo: Option<&'static str>,
}

/// `runo.tokyo/`配下の各サブサービスへのリンク集(ユーザー指示、
/// 2026-07-21: タイトル付きでURLを紹介し、未完成のものはその旨明記)。
pub const SUB_SERVICES: &[SubService] = &[
    SubService {
        path: "/rs-git",
        title: "RS-Git",
        description: "自作Git forge。OTPログイン・アクセス権限管理付き",
        status: "稼働中",
        github_repo: None,
    },
    SubService {
        path: "/rs-sync",
        title: "RS-Sync",
        description: "GitHub/RS-Git/Gitea/Gitbucket間のリポジトリ同期・バックアップツール",
        status: "稼働中(MVP、GitHubプロバイダのみ実機検証済み)",
        github_repo: Some("aon-co-jp/RS-Sync"),
    },
    SubService {
        path: "/open-web-server",
        title: "open-web-server",
        description: "Apache+Nginx相当のRust製Webサーバー紹介ページ",
        status: "開発途中(紹介ページのみ公開、本体サーバー化は準備中)",
        github_repo: None,
    },
    SubService {
        path: "/chiketto",
        title: "RS-Red",
        description: "Redmine相当のチケット管理システム(旧称RS-Chiketto、2026-07-22にRS-Redへ改名)",
        status: "公開中(open-web-serverのpath_prefixルーティング経由でRS-Redへ、2026-07-22)",
        github_repo: Some("aon-co-jp/RS-Red"),
    },
    SubService {
        path: "/blog",
        title: "RS-Blog",
        description: "WordPress相当のブログシステム",
        status: "公開中(open-web-serverのpath_prefixルーティング経由でRS-Blogへ、2026-07-22)",
        github_repo: Some("aon-co-jp/RS-Blog"),
    },
    SubService {
        path: "/ec",
        title: "RS-EC",
        description: "EC-CUBE相当のECサイトシステム",
        status: "公開中(open-web-serverのpath_prefixルーティング経由でRS-ECへ、2026-07-22)",
        github_repo: Some("aon-co-jp/RS-EC"),
    },
];

fn render_subservices_section() -> String {
    let mut html = String::from(
        r#"<section class="block" id="subservices"><h2>runo.tokyo 内のサブサービス</h2><ul class="app-list">"#,
    );
    for s in SUB_SERVICES {
        html.push_str(&format!(
            r#"<li><strong><a href="{path}">{title}</a></strong>: {desc} <span class="pending">({status})</span></li>"#,
            path = s.path,
            title = html_escape(s.title),
            desc = html_escape(s.description),
            status = html_escape(s.status),
        ));
    }
    html.push_str("</ul></section>");
    html
}

fn render_runo_link_section() -> String {
    r#"<section class="block" id="runo-index"><h2>エコシステム全体を見る</h2>
<p><a href="https://github.com/aon-co-jp/RUNO" target="_blank" rel="noopener">RUNO</a>リポジトリで、RUNOプロジェクト(open-cosmo / open-aruaru エコシステム全体のプロジェクト一覧、README・PORTING.md・CLAUDE.mdへのリンク付き)を御紹介しております。「プロジェクトシリーズ索引」ページでもまとめて確認できます(13言語(日本語・英語・中国語・韓国語ほか)対応、ログイン不要でGitHubと同じようにファイル一覧を閲覧できます)。</p>
<p><a href="https://github.com/aon-co-jp/RS-Ops" target="_blank" rel="noopener">RS-Ops</a>(エコシステム全体マップの自動生成、姉妹リポジトリ<a href="https://github.com/aon-co-jp/RS-Git" target="_blank" rel="noopener">RS-Git</a>・<a href="https://github.com/aon-co-jp/aruaru-llm" target="_blank" rel="noopener">aruaru-llm</a>との「SET」連携)も、このプロジェクトシリーズ索引に含まれています。</p>
<div class="org-link"><a href="/runo">📚 プロジェクトシリーズ索引を見る (/runo)</a></div>
</section>"#
        .to_string()
}

/// TOPページのHTML本文を生成する(純粋関数、テスト容易)。
pub fn top_page_html() -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>runo.tokyo | 東京都西部の暮らしとテレワーク、open-cosmoエコシステム</title>
<meta name="description" content="東京都西部各自治体へのリンク集、企業・工場・倉庫事業用地のドローン空撮紹介、そしてopen-aruaru/open-cosmoエコシステムのアプリ一覧を掲載するruno.tokyoのTOPページ。Rust + Poem製。">
<style>{STYLE}</style>
</head>
<body>
<main>
<p class="blog-link"><a href="{blog_url}" target="_blank" rel="noopener">📝 {blog_title_ja}</a> / <a href="{blog_url}" target="_blank" rel="noopener">{blog_title_en}</a></p>
<header>
  <h1><span>runo</span>.tokyo</h1>
  <p>東京都西部の暮らし・テレワークと、open-cosmoエコシステムの入口</p>
</header>

<div class="quick-links">
  <a href="#municipalities">自治体リンク集</a>
  <a href="#drone">ドローン空撮紹介</a>
  <a href="/runo">📚 プロジェクトシリーズ</a>
</div>

{catchcopy}

{intro}

{municipalities}

{drone}

{apps}

{subservices}

{runo_link}

<footer>&copy; 2026 runo.tokyo (Rust + Poem)</footer>
</main>
</body>
</html>"##,
        STYLE = STYLE,
        blog_url = BLOG_POST_URL,
        blog_title_ja = html_escape(BLOG_POST_TITLE_JA),
        blog_title_en = html_escape(BLOG_POST_TITLE_EN),
        catchcopy = render_catchcopy_section(),
        intro = render_intro_section(),
        municipalities = render_municipality_section(),
        drone = render_drone_section(),
        apps = render_app_directory_section(),
        subservices = render_subservices_section(),
        runo_link = render_runo_link_section(),
    )
}

#[handler]
fn index_handler() -> impl IntoResponse {
    Html(top_page_html())
}

#[derive(Deserialize)]
struct LangQuery {
    lang: Option<String>,
}

/// `/runo` — エコシステム全体のメタ索引リポジトリ`RUNO`と同内容の
/// 「プロジェクトシリーズ索引」ページ。
#[handler]
async fn runo_index_page(Query(q): Query<LangQuery>) -> Html<String> {
    let lang = i18n::Lang::parse(q.lang.as_deref());
    Html(meta_index::render_page(lang, "/runo"))
}

/// サブサービス個別ページの共通ハンドラを生成する。`/chiketto`等のリンク先で
/// README.md・PORTING.md(お引越しポーター)・CLAUDE.md(設計思想＆開発方針
/// ＆開発環境ルール)をGitHubと同じように読めるようにする(ユーザー指示、
/// 2026-07-21)。対象プロジェクトが未着手でリポジトリが存在しない場合は
/// 404を返す。
async fn subservice_page(repo: &'static str, q: LangQuery) -> Html<String> {
    let lang = i18n::Lang::parse(q.lang.as_deref());
    match meta_index::render_single_project_page(repo, lang) {
        Some(html) => Html(html),
        None => Html(format!("<p>{}は現在準備中です。</p>", html_escape(repo))),
    }
}

#[handler]
async fn chiketto_page(Query(q): Query<LangQuery>) -> Html<String> {
    // 2026-07-22にGitHub上のリポジトリ名がRS-ChikettoからRS-Redへ改名された
    // (`F:\runo\RS-Red\CLAUDE.md`参照)。runo.tokyo側の公開パス自体は
    // 引き続き`/chiketto`のまま(URLの大規模な変更は本タスクのスコープ外)。
    subservice_page("aon-co-jp/RS-Red", q).await
}

#[handler]
async fn blog_page(Query(q): Query<LangQuery>) -> Html<String> {
    subservice_page("aon-co-jp/RS-Blog", q).await
}

#[handler]
async fn ec_page(Query(q): Query<LangQuery>) -> Html<String> {
    subservice_page("aon-co-jp/RS-EC", q).await
}

/// Poemのルーティングを構築する(サーバー起動処理とは分離し、テスト可能にする)。
///
/// `/rs-git`・`/open-web-server`は本アプリではなく、それぞれ実際に稼働する
/// RS-Git・open-web-serverへnginxがproxy_passするため、ここではルートを
/// 定義しない(定義しても本番はnginxが先に振り分ける)。
///
/// **RS-Opsについて(2026-07-22、ユーザー指示)**: RS-Opsは`/RS-Ops`という
/// 専用のサブサービスページ・ルートは持たない——「プロジェクトシリーズ
/// 索引」(`/runo`、`meta_index::PROJECTS`)と、TOPページの「エコシステム
/// 全体を見る」節(`render_runo_link_section`)の2箇所でのみ紹介する方針
/// (ユーザー指示: 「プロジェクトシリーズ　と　エコシステム全体を見るの
/// 中で紹介して」)。
pub fn build_app() -> Route {
    Route::new()
        .at("/", get(index_handler))
        .at("/runo", get(runo_index_page))
        .at("/chiketto", get(chiketto_page))
        .at("/blog", get(blog_page))
        .at("/ec", get(ec_page))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// キャッチコピー本文がそのままページに含まれ、Google検索・YouTube検索
    /// リンク(画像検索は含まない)が付くことを確認する(2026-07-20追記)。
    #[test]
    fn top_page_contains_catchcopy_with_google_and_youtube_links() {
        let html = top_page_html();
        assert!(html.contains("東京都あきる野市と多摩地域周辺の東京と軽井沢と世界をつなぎ"));
        let q = percent_encode_query(CATCHCOPY);
        assert!(html.contains(&format!("https://www.google.com/search?q={q}")));
        assert!(html.contains(&format!("https://www.youtube.com/results?search_query={q}")));
    }

    /// 「巨大なスピーカー」の画像検索・YouTube検索リンクが掲載されていることを確認する。
    #[test]
    fn top_page_contains_giant_speaker_image_and_youtube_search_links() {
        let html = top_page_html();
        let q = percent_encode_query("巨大なスピーカー");
        assert!(html.contains(&format!("https://www.google.com/search?tbm=isch&q={q}")));
        assert!(html.contains(&format!("https://www.youtube.com/results?search_query={q}")));
    }

    /// 「親子でパソコンを教えながらテレワークする」写真の画像検索・YouTube検索
    /// リンクが掲載されていることを確認する。
    #[test]
    fn top_page_contains_parent_child_telework_photo_search_links() {
        let html = top_page_html();
        let q = percent_encode_query("親子 パソコンを教える テレワーク 写真");
        assert!(html.contains(&format!("https://www.google.com/search?tbm=isch&q={q}")));
        assert!(html.contains(&format!("https://www.youtube.com/results?search_query={q}")));
    }

    #[test]
    fn app_directory_lists_all_family_apps() {
        let html = top_page_html();
        for app in family_apps() {
            assert!(html.contains(app.name), "{}がHTMLに含まれていません", app.name);
            assert!(html.contains(app.github_url));
        }
    }

    #[test]
    fn top_page_contains_intro_text_verbatim() {
        let html = top_page_html();
        for p in INTRO_PARAGRAPHS {
            assert!(html.contains(p), "紹介文の段落がHTMLに含まれていません: {p}");
        }
    }

    #[test]
    fn top_page_contains_google_image_search_link() {
        let html = top_page_html();
        assert!(html.contains(GOOGLE_IMAGE_SEARCH_URL));
    }

    #[test]
    fn percent_encode_query_encodes_japanese_and_spaces() {
        let encoded = percent_encode_query("注文住宅 シアタールーム");
        assert!(!encoded.contains(' '), "空白がエンコードされていません: {encoded}");
        assert!(encoded.contains("%20"), "半角スペースが%20になっていません: {encoded}");
        assert!(!encoded.chars().any(|c| c == '注' || c == '宅'), "日本語がエンコードされずに残っています: {encoded}");
    }

    #[test]
    fn top_page_contains_contextual_search_link_sets_for_every_paragraph() {
        let html = top_page_html();
        let all_queries = [
            "注文住宅 シアタールーム 壁埋め込みスピーカー",
            "地方 郊外 移住 テレワーク 一戸建て",
            "テレワーク リモートワーク カフェ ノートパソコン",
            "親子 パソコン教室 テレワーク",
            "Claude Code Desktop 使い方",
            "Claude Code フリーランス プログラマー",
        ];
        for query in all_queries {
            let q = percent_encode_query(query);
            let isch_url = format!("https://www.google.com/search?tbm=isch&q={q}");
            let web_url = format!("https://www.google.com/search?q={q}");
            let yt_url = format!("https://www.youtube.com/results?search_query={q}");
            assert!(html.contains(&isch_url), "画像検索リンクが見つかりません: {isch_url}");
            assert!(html.contains(&web_url), "Google検索リンクが見つかりません: {web_url}");
            assert!(html.contains(&yt_url), "YouTube検索リンクが見つかりません: {yt_url}");
        }
    }

    /// RS-Ops(RS-Git・aruaru-llmとのSET)が「エコシステム全体を見る」節に
    /// 紹介されており、`/runo`(プロジェクトシリーズ索引)にも掲載されて
    /// いることを確認する(ユーザー指示、2026-07-22: 「プロジェクト
    /// シリーズとエコシステム全体を見るの中で紹介して」)。RS-Ops専用の
    /// トップレベルルート(`/RS-Ops`等)は意図的に持たない。
    #[test]
    fn rs_ops_is_introduced_in_ecosystem_section_and_project_series() {
        let html = top_page_html();
        assert!(html.contains("aon-co-jp/RS-Ops"), "エコシステム全体を見る節にRS-Opsへのリンクが見つかりません");
        assert!(html.contains("aon-co-jp/RS-Git"));
        assert!(html.contains("aon-co-jp/aruaru-llm"));

        let runo_html = meta_index::render_page(i18n::Lang::Ja, "/runo");
        assert!(runo_html.contains("aon-co-jp/RS-Ops"), "プロジェクトシリーズ索引にRS-Opsが見つかりません");
        assert!(runo_html.contains("RS-Ops"));
    }

    /// TOPページにサブサービス一覧(タイトル・パス・進捗状況)が掲載され、
    /// 未完成のものは「開発途中」等と明記されていることを確認する
    /// (ユーザー指示、2026-07-21)。
    #[test]
    fn top_page_lists_subservices_with_status() {
        let html = top_page_html();
        for s in SUB_SERVICES {
            assert!(html.contains(s.path), "{}へのリンクが見つかりません", s.path);
            assert!(html.contains(s.title), "{}のタイトルが見つかりません", s.title);
            assert!(html.contains(s.status), "{}の状態表示が見つかりません", s.title);
        }
    }

    /// `/chiketto`等のサブサービスページで、README/PORTING.md/CLAUDE.mdへの
    /// リンクがGitHubのように読めることを確認する。
    #[test]
    fn subservice_pages_expose_readme_porting_claude_links() {
        for repo in ["aon-co-jp/RS-Red", "aon-co-jp/RS-Blog", "aon-co-jp/RS-EC"] {
            let html = meta_index::render_single_project_page(repo, i18n::Lang::Ja).expect("project should exist");
            assert!(html.contains(&format!("{repo}/blob/main/README.md")), "{repo}: READMEリンクが見つかりません");
            assert!(html.contains(&format!("{repo}/blob/main/PORTING.md")), "{repo}: PORTING.mdリンクが見つかりません");
            assert!(html.contains(&format!("{repo}/blob/main/CLAUDE.md")), "{repo}: CLAUDE.mdリンクが見つかりません");
        }
    }

    /// 「プロジェクトのご利用」ボックスが、ファイル説明とは独立して
    /// 日英併記のリンク(サイトを開く/GitHubで入手)のみを持つことを確認する
    /// (ユーザー指示、2026-07-22)。
    #[test]
    fn project_cards_have_usage_box_separate_from_file_links() {
        let html = meta_index::render_page(i18n::Lang::Ja, "/runo");
        assert!(html.contains("プロジェクトのご利用 / Project Usage"));
        assert!(html.contains("Get on GitHub"));
        assert!(html.contains("class=\"usage-box\""));
    }

    #[test]
    fn top_page_contains_provided_municipality_urls_and_pending_notes() {
        let html = top_page_html();
        assert!(html.contains("https://www.city.akiruno.tokyo.jp/"));
        assert!(html.contains("https://www.city.akiruno.tokyo.jp/kanko/"));
        assert!(html.contains("https://hinohara-kankou.jp/"));
        assert!(html.contains("https://www.city.ome.tokyo.jp/"));
        assert!(html.contains("https://www.town.okutama.tokyo.jp/index.html"));
        assert!(html.contains("https://www.okutama.gr.jp/site/"));
        assert!(html.contains("https://www.hkc.or.jp/takaosan/"));
        assert!(html.contains("https://tokyotouristinfo.com/detail/M0021"));
        assert!(html.contains("https://www.takaotozan.co.jp/"));
        assert!(html.contains("https://www.city.hachioji.tokyo.jp/"));
        assert!(html.contains("https://www.town.hinode.tokyo.jp/"));
        assert!(html.contains("https://www.city.fussa.tokyo.jp/"));
        assert!(html.contains("https://www.city.hamura.tokyo.jp/"));
        assert!(html.contains("https://www.seiburailway.jp/railway/station/haijima/"));
        assert!(html.contains("https://www.jreast.co.jp/estation/stations/1213.html"));
        assert!(html.contains("https://www.city.akishima.lg.jp/"));
        assert!(html.contains("https://www.city.tachikawa.lg.jp/"));
        assert!(html.contains("https://www.city.kunitachi.tokyo.jp/index.html"));
        assert!(html.contains("羽村市"));
        assert!(html.contains("準備中"));
    }

    #[test]
    fn top_page_links_to_runo_index() {
        let html = top_page_html();
        assert!(html.contains(r#"href="/runo""#));
    }

    #[test]
    fn top_page_has_drone_placeholder_not_fake_video() {
        let html = top_page_html();
        assert!(html.contains("準備中"));
        assert!(html.contains("ドローン"));
        assert!(!html.contains("youtube.com/watch"));
    }

    #[test]
    fn runo_index_page_lists_projects() {
        let html = meta_index::render_page(i18n::Lang::Ja, "/runo");
        for p in meta_index::PROJECTS {
            assert!(html.contains(p.name), "{}がプロジェクトシリーズ索引に含まれていません", p.name);
        }
        assert!(html.contains("aon-co-jp/RUNO"));
    }

    /// 2026-07-22のGitHubリポジトリ改名(RS-Chiketto → RS-Red)が
    /// `/runo`ページ・PROJECTS配列に反映されており、旧名のリポジトリ参照が
    /// 残っていないことを確認する。
    #[test]
    fn runo_index_page_reflects_rs_chiketto_rename_to_rs_red() {
        let html = meta_index::render_page(i18n::Lang::Ja, "/runo");
        assert!(html.contains("aon-co-jp/RS-Red"), "RS-Redへの参照が見つかりません");
        assert!(!html.contains("aon-co-jp/RS-Chiketto"), "改名済みのはずのRS-Chikettoへの参照が残っています");
        assert!(meta_index::find_project("aon-co-jp/RS-Red").is_some());
        assert!(meta_index::find_project("aon-co-jp/RS-Chiketto").is_none());
    }

    /// `github_cache`(バックグラウンド定期取得のキャッシュ)にGitHub情報が
    /// 反映されている場合、`/runo`ページの該当プロジェクトカードに
    /// 「GitHub自動同期」の表示が実際に現れることを確認する
    /// (モックHTTPサーバー経由の実HTTP検証、`wiremock`使用)。
    #[tokio::test]
    async fn runo_index_page_shows_auto_synced_github_info_when_cached() {
        let _guard = github_cache::test_lock().lock().unwrap_or_else(|e| e.into_inner());
        let server = wiremock::MockServer::start().await;
        let body = serde_json::json!([{
            "full_name": "aon-co-jp/runo-scanner",
            "description": "テスト用の自動同期説明文",
            "pushed_at": "2026-07-23T12:00:00Z",
            "stargazers_count": 7
        }]);
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/users/aon-co-jp/repos"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = reqwest::Client::new();
        github_cache::refresh_once(&client, &server.uri()).await.expect("refresh should succeed");

        let html = meta_index::render_page(i18n::Lang::Ja, "/runo");
        assert!(html.contains("GitHub自動同期"), "自動同期の表示が見つかりません");
        assert!(html.contains("テスト用の自動同期説明文"));
        assert!(html.contains("⭐ 7"));
    }

    /// 2026-07-23時点で`F:\runo`配下に実在すると確認できた、それまで
    /// `/runo`ページに掲載漏れだったリポジトリ(RS-Guard・RS-LinkFusion・
    /// RS-SmartTCP・RS-Gitbucket・RS-Sync・runo-scanner)が、それぞれ
    /// 掲載されていることを確認する。
    #[test]
    fn runo_index_page_lists_previously_missing_repos() {
        let html = meta_index::render_page(i18n::Lang::Ja, "/runo");
        for repo in [
            "aon-co-jp/RS-Guard",
            "aon-co-jp/RS-LinkFusion",
            "aon-co-jp/RS-SmartTCP",
            "aon-co-jp/RS-Gitbucket",
            "aon-co-jp/RS-Sync",
            "aon-co-jp/runo-scanner",
        ] {
            assert!(html.contains(repo), "{repo}が/runoページに見つかりません");
            assert!(meta_index::find_project(repo).is_some(), "{repo}がPROJECTS配列に見つかりません");
        }
    }

    #[test]
    fn runo_index_page_supports_lang_switch() {
        let html_en = meta_index::render_page(i18n::Lang::En, "/runo");
        assert!(html_en.contains("Project Series Index"));
        let html_ar = meta_index::render_page(i18n::Lang::Ar, "/runo");
        assert!(html_ar.contains(r#"dir="rtl""#));
    }

    /// GitHub本家の「Code」タブと同じ見た目(📁/📄+ファイル名+GitHubの実
    /// ページへのリンク)でファイル一覧を表示するための土台
    /// (`.file-tree`要素+GitHub Contents APIを叩く`fetchFileTree`関数)が
    /// ページに存在することを確認する(ユーザー指示、2026-07-20:
    /// 「重要なREADMEやお引越しポーターや設計思想などのファイルを
    /// ログインせずにGITHUBの様に見れる様にして」)。
    #[test]
    fn runo_index_page_has_github_style_file_tree_fetch() {
        let html = meta_index::render_page(i18n::Lang::Ja, "/runo");
        assert!(html.contains(r#"class="file-tree""#));
        assert!(html.contains("data-repo-tree"));
        assert!(html.contains("fetchFileTree"));
        assert!(html.contains("/contents/"));
    }

    /// TOPページ(`/`)の`/runo`リンクをクリックして遷移した瞬間、手動操作
    /// 無しに全リポジトリの最新情報を自動取得するJS(`setTimeout`による
    /// 順次自動発火)が組み込まれていることを確認する(ユーザー指示、
    /// 2026-07-20: 「TOPページからのリンクをクリックした瞬間に最新の
    /// リポジトリ一覧を取得して」)。
    #[test]
    fn runo_index_page_auto_fetches_all_repos_on_load() {
        let html = meta_index::render_page(i18n::Lang::Ja, "/runo");
        assert!(html.contains("querySelectorAll('.repo-card')"));
        assert!(html.contains("setTimeout"));
    }

    /// クリック時・ページ読み込み時に、紹介文(role)がGitHubのREADME.mdの
    /// 内容と同期されることを確認する(ユーザー指示、2026-07-22:
    /// 「クリックした瞬間に、githubの最新といつも同期を取って、いつも
    /// 最新の内容を紹介して」)。
    #[test]
    fn runo_index_page_syncs_role_text_with_github_readme() {
        let html = meta_index::render_page(i18n::Lang::Ja, "/runo");
        assert!(html.contains("data-repo-role="));
        assert!(html.contains("fetchReadmeSummary"));
        assert!(html.contains("raw.githubusercontent.com"));
    }

    #[test]
    fn subservice_page_syncs_role_text_with_github_readme() {
        let html = meta_index::render_single_project_page("aon-co-jp/RS-Red", i18n::Lang::Ja).expect("project should exist");
        assert!(html.contains("data-repo-role="));
        assert!(html.contains("fetchReadmeSummary"));
    }
}
