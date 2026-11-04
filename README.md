# runo.tokyo

東京都西部(あきる野市・旧五日市・桧原村・青梅・奥多摩・羽村・福生・昭島・立川・国立・八王子・高尾・日の出町など)の暮らし・テレワークと、open-runo/open-aruaruエコシステムの入口となるWebサイト。Poem(Rust)製。

## 現状の機能

- TOPページ(`/`)
  - ユーザー確定済みの紹介文(改変禁止・そのまま掲載)
  - 東京都西部各自治体の役所・観光案内サイトへのリンク集(実在確認済みURLのみ掲載。未提供のものは推測せず「準備中」と明記)
  - 企業・工場・倉庫事業用地のドローン空撮紹介(実際の動画は未収録のため準備中プレースホルダー)
  - 親子でのテレワーク・パソコン学習の文脈でのGoogle画像検索(`https://www.google.com/imghp`)へのリンク
  - open-aruaru/open-cosmo系列アプリの一覧とGitHubリンク
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

## 配布状況の紹介(`/runo`ページ、2026-07-24追加)

各プロジェクトカードに「📦 配布状況」セクションを追加した。実際に各
リポジトリの`install.sh`/`install.ps1`/`.github/workflows/release.yml`の
有無、および`gh release list -R aon-co-jp/<repo>`で実際に公開されている
GitHub Releaseの存在を確認した上で、`meta_index::Project::distribution`
(静的フィールド)に反映している(推測で「完成」とは書いていない)。

- **Windows/Linux版がダウンロード可能**(3点セット+公開Release実在を
  確認済み): RPoem・RS-Git・RS-Red・RS-Blog・RS-EC・open-web-server
  (いずれも`https://github.com/aon-co-jp/<repo>/releases/latest`への
  直接リンクをカードに表示)。
- **インストーラースクリプトは整備済みだが公開Releaseはまだ無い**
  (CI失敗・タグ未push等、正直にその理由をカードへ記載): aruaru-db・
  open-raid-z(v0.1.0タグはpush済みだがRelease自動ビルドがCIで失敗中、
  修正済みで再タグでの検証待ち)・RS-LinkFusion(3点セット整備済み・
  タグ未push)・RS-Guard・RS-Ops・RS-Sync(install.sh/ps1のみで
  release.yml未整備)。
- **Android対応**: 唯一`open-web-server`がKotlin/Gradleシェル+APK生成
  (`app-debug.apk`)まで到達しているが、`adb`が`unauthorized`を返し続け
  実機/エミュレータでの動作確認は未完了——「開発中」であって
  「配布可能」ではないと正直に表示する。RS-Redは`cargo ndk`での
  クロスコンパイル確認のみ(APK化は未着手)。他のリポジトリはAndroid
  対応に未着手。
- **配布状況セクション自体を表示しない**: ライブラリ専用クレート等、
  エンドユーザーが実行する配布物を持たないプロジェクト(RS-JSON・
  rs-to-readme・runo-scanner等、`distribution: None`)。

**自動更新機能(`github_cache.rs`)との役割分担**: 配布状況(インストーラー
の有無・GitHub Release実在)は`meta_index::Project`側の静的フィールドと
して管理する方針を選んだ(GitHub APIで自動検出する`GET /repos/<repo>/
releases/latest`も技術的には可能だが、未認証GitHub APIのレート制限が
60req/時しかなく、既存の`github_cache.rs`が使う「全リポジトリまとめて
1〜2リクエスト」方式と異なりリポジトリ数だけ個別リクエストが必要になる
——配布状況は更新頻度が低い情報のため、この制約下では静的管理の方が
妥当と判断した、詳細は`src/meta_index.rs`の`Distribution`構造体コメント
参照)。

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

### Distribution status (added 2026-07-24)

Each project card now shows a "📦 Distribution" box, populated from a
static `distribution` field on `meta_index::Project` after actually
checking each repo for `install.sh`/`install.ps1`/
`.github/workflows/release.yml` and real published GitHub Releases (via
`gh release list`) — nothing is marked "ready" from a guess.

- **Windows/Linux downloads available** (installer trio + confirmed
  public Release): RPoem, RS-Git, RS-Red, RS-Blog, RS-EC,
  open-web-server — each links straight to
  `https://github.com/aon-co-jp/<repo>/releases/latest`.
- **Installer scripts exist but no public Release yet** (honestly
  explained per repo): aruaru-db and open-raid-z (v0.1.0 tag pushed but
  the release-build CI is currently failing; fix pushed, re-tag
  verification pending), RS-LinkFusion (trio ready, tag not pushed
  yet), RS-Guard / RS-Ops / RS-Sync (install.sh/ps1 only, no
  release.yml yet).
- **Android**: only `open-web-server` has reached Kotlin/Gradle shell +
  APK build (`app-debug.apk`), but `adb` keeps returning `unauthorized`
  so on-device/emulator verification is incomplete — shown as "in
  development", not "downloadable". RS-Red has only confirmed
  cross-compilation via `cargo ndk` (no APK yet). Other repos haven't
  started Android work.
- **No distribution box at all** for library-only crates with no
  end-user binary (RS-JSON, rs-to-readme, runo-scanner, etc.,
  `distribution: None`).

This is deliberately a static field rather than a periodic GitHub API
poll: unauthenticated GitHub API calls are capped at 60/hour, and
checking `releases/latest` per repo (unlike the existing bulk
`GET /users/aon-co-jp/repos` call) would need one request per repo for
information that changes far less often than descriptions/stars.

## ライセンス / License

MPL-2.0(LICENSE参照)。 / MPL-2.0 (see LICENSE).

## 現在準備中の項目 / Items still marked as 'coming soon'

桧原村の公式村サイトURL、企業・工場・倉庫事業用地のドローン空撮動画は、実在のURL・動画が未提供のため「準備中」表示です(推測では埋めません)。 / The official Hinohara village URL and the drone footage are shown as 'coming soon' until real URLs/videos are provided.

## API 方針: REST 全廃・gRPC / Connect へ (2026-11-04)

### 日本語
- **REST は全廃する。新しい API は REST で作らない。** ブラウザや `curl` から呼ぶ API は **Connect の単純呼び出し**(`POST /<パッケージ>.<Service>/<Method>`、`content-type: application/json`、本文は JSON)、サービス間は **gRPC / gRPC-web**(RPoem の管理API `open_runo.admin.v1.Admin` と同じ流儀)。ストリーミングは、必要になってから。
- **既存の REST は、触るときに順に移す。** 移したら旧 REST の経路は消す。移行が終わるまで残るが、**新規には追加しない**。
- **JSON は RS-JSON に統一**する(読み込みは `from_str_secure` / `parse_secure` でサイズ・深さ・重複キーの上限つき、書き出しは `rust_json::fast`)。ブラウザの WASM に `prost` は入れない(サイズが増えるため)。
- **例外(自分たちの API ではなく、相手の規格が決めているもの)**: ACME、OAuth/OIDC のコールバック、第三者(GitHub など)からの Webhook、ヘルスチェック、静的ファイル配信、OTLP など。これらは規格どおりの HTTP を受ける。
- 参考実装: `RPoem`(`open_runo.admin.v1.Admin`)、`open-easy-web`(`server/src/connect.rs`、`proto/open_easy_web/admin/v1/admin.proto`)。
- ※ このリポジトリのコードが既に方針どおりかどうかは、各リポジトリの現状による。この節は方針の記録であり、コードの変更を意味しない。

### English
- **REST is being abolished. New APIs are not built as REST.** APIs called from browsers or `curl` use **Connect unary calls** (`POST /<package>.<Service>/<Method>`, `content-type: application/json`, JSON body); service-to-service calls use **gRPC / gRPC-web** (the same style as RPoem's admin API `open_runo.admin.v1.Admin`). Streaming only when it is actually needed.
- **Existing REST endpoints are migrated one by one, when they are touched.** The old REST route is removed once migrated. Until migration is finished they remain, but **no new ones are added**.
- **JSON is unified on RS-JSON**: reading with `from_str_secure` / `parse_secure` (size, depth and duplicate-key limits), writing with `rust_json::fast`. `prost` is not put into browser WASM (it would grow the binary).
- **Exceptions (shapes dictated by someone else's standard, not our own API)**: ACME, OAuth/OIDC callbacks, webhooks from third parties (e.g. GitHub), health checks, static file serving, OTLP, and so on. These accept plain HTTP as the standard requires.
- Reference implementations: `RPoem` (`open_runo.admin.v1.Admin`) and `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Note: whether the code in a given repository already follows this policy depends on that repository. This section records the policy; it does not mean the code has been changed.

### Deutsch (KI-Übersetzung, redaktionell überarbeitet; noch nicht von Muttersprachlern geprüft)
- **REST wird vollständig abgeschafft; neue APIs entstehen nicht mehr als REST.** APIs, die aus dem Browser oder per `curl` aufgerufen werden, verwenden **unäre Connect-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`, JSON-Body). Zwischen Diensten kommen **gRPC bzw. gRPC-web** zum Einsatz, im selben Stil wie die Admin-API von RPoem (`open_runo.admin.v1.Admin`). Streaming wird erst ergänzt, wenn es tatsächlich gebraucht wird.
- **Bestehende REST-Endpunkte werden schrittweise umgestellt, und zwar immer dann, wenn man ohnehin an ihnen arbeitet.** Sobald ein Endpunkt umgestellt ist, wird die alte REST-Route entfernt. Bis zum Abschluss der Migration bleiben die übrigen bestehen, es werden aber **keine neuen mehr hinzugefügt**.
- **JSON wird einheitlich über RS-JSON verarbeitet**: Gelesen wird mit `from_str_secure` bzw. `parse_secure` (mit Obergrenzen für Größe, Verschachtelungstiefe und doppelte Schlüssel), geschrieben mit `rust_json::fast`. `prost` wird nicht in das Browser-WASM aufgenommen, da es die Datei vergrößern würde.
- **Ausnahmen (Schnittstellen, deren Form ein fremder Standard vorgibt und die nicht zu unserer eigenen API gehören)**: ACME, OAuth/OIDC-Callbacks, Webhooks von Drittanbietern (z. B. GitHub), Health-Checks, die Auslieferung statischer Dateien, OTLP und Ähnliches. Diese nehmen, wie vom jeweiligen Standard verlangt, gewöhnliches HTTP entgegen.
- Referenzimplementierungen: `RPoem` (`open_runo.admin.v1.Admin`) und `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Hinweis: Ob der Code eines einzelnen Repositorys diese Richtlinie bereits erfüllt, hängt vom jeweiligen Repository ab. Dieser Abschnitt dokumentiert lediglich die Richtlinie; er besagt nicht, dass der Code schon angepasst wurde.

### Русский (перевод выполнен с помощью ИИ и отредактирован; носителями языка пока не проверен)
- **От REST мы полностью отказываемся; новые API больше не создаются в стиле REST.** API, к которым обращаются из браузера или через `curl`, используют **унарные вызовы Connect** (`POST /<пакет>.<Service>/<Method>`, `content-type: application/json`, тело запроса — JSON). Для взаимодействия между сервисами применяются **gRPC и gRPC-web** — в том же стиле, что и административный API RPoem (`open_runo.admin.v1.Admin`). Потоковая передача данных добавляется лишь тогда, когда в ней действительно возникает необходимость.
- **Существующие REST-эндпоинты переводятся поэтапно — по мере того, как с ними приходится работать.** После перевода старый REST-маршрут удаляется. Пока миграция не завершена, оставшиеся эндпоинты сохраняются, но **новые не добавляются**.
- **Работа с JSON унифицирована на основе RS-JSON**: чтение — через `from_str_secure` или `parse_secure` (с ограничениями на размер, глубину вложенности и повторяющиеся ключи), запись — через `rust_json::fast`. `prost` в браузерный WASM не включается, поскольку это увеличило бы размер файла.
- **Исключения (интерфейсы, форму которых определяет чужой стандарт, а не наш собственный API)**: ACME, обратные вызовы OAuth/OIDC, вебхуки сторонних сервисов (например, GitHub), проверки работоспособности, раздача статических файлов, OTLP и тому подобное. Такие интерфейсы принимают обычный HTTP, как того требует соответствующий стандарт.
- Эталонные реализации: `RPoem` (`open_runo.admin.v1.Admin`) и `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Примечание: соответствует ли код конкретного репозитория этой политике, зависит от самого репозитория. Данный раздел лишь фиксирует политику и не означает, что код уже изменён.
