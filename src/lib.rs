//! runo.tokyo Webサイト(ひな型)。
//!
//! 【現状のスコープ】
//! - open-aruaru/open-runo系列アプリの紹介ページ(アプリ名・説明・GitHubリンク)
//! - `readme-to-rs`のWeb版(README本文をPOSTすると`.rs`形式のテキストを返す)
//!
//! 【未実装(今後の拡張、要件メモ参照)】
//! - AI辞書のGoogleドライブ連携(登録・ダウンロード・読み書きをユーザー自身の
//!   Googleドライブ上で行う)
//! - PostgreSQL + aruaru-dbのデュアルDB構成
//! - バージョンレス/バージョン管理/Git管理のトライブリッド方式
//! - Tauri 2.0デスクトップ版
//!
//! これらは設計(特にGoogle OAuth連携の認可フロー、デュアルDBの整合性モデル)
//! を詰めてから着手する。

use poem::web::Html;
use poem::{get, handler, post, IntoResponse, Route};
use serde::Serialize;

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

/// アプリ紹介ページのHTML本文を生成する(純粋関数、テスト容易)。
pub fn app_directory_html() -> String {
    let mut body = String::from(
        "<!doctype html><html lang=\"ja\"><head><meta charset=\"utf-8\"><title>runo.tokyo</title></head><body>",
    );
    body.push_str("<h1>open-aruaru / open-runo シリーズ</h1><ul>");
    for app in family_apps() {
        body.push_str(&format!(
            "<li><strong>{}</strong>: {} (<a href=\"{}\">GitHub</a>)</li>",
            app.name, app.description, app.github_url
        ));
    }
    body.push_str("</ul></body></html>");
    body
}

/// README本文をreadme-to-rsで変換する(Web版readme-to-rsの中核)。
pub fn convert_readme(markdown: &str) -> String {
    readme_to_rs::convert_to_rs(markdown)
}

#[handler]
fn index_handler() -> impl IntoResponse {
    Html(app_directory_html())
}

#[handler]
fn readme_to_rs_handler(body: String) -> impl IntoResponse {
    convert_readme(&body)
}

/// Poemのルーティングを構築する(サーバー起動処理とは分離し、テスト可能にする)。
pub fn build_app() -> Route {
    Route::new()
        .at("/", get(index_handler))
        .at("/readme-to-rs", post(readme_to_rs_handler))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_directory_lists_all_family_apps() {
        let html = app_directory_html();
        for app in family_apps() {
            assert!(html.contains(app.name), "{}がHTMLに含まれていません", app.name);
            assert!(html.contains(app.github_url));
        }
    }

    #[test]
    fn convert_readme_wraps_lines_in_doc_comments() {
        let out = convert_readme("# タイトル\n\n本文");
        assert_eq!(out, "//! # タイトル\n//!\n//! 本文\n");
    }
}
