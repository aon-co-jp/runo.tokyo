# runo.tokyo

open-runoシリーズのWebサイト(ひな型)。Poem(Rust)製。

## 現状の機能

- アプリ紹介ページ(`/`): open-aruaru/open-runo系列アプリの一覧とGitHubリンク
- readme-to-rs Web版(`POST /readme-to-rs`): README本文を送ると`.rs`形式のテキストを返す

## 未実装(今後の拡張)

- AI辞書のGoogleドライブ連携(ユーザー自身のGoogleドライブで辞書を管理)
- PostgreSQL + aruaru-dbのデュアルDB構成
- バージョンレス/バージョン管理/Git管理のトライブリッド方式
- Tauri 2.0デスクトップ版

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
