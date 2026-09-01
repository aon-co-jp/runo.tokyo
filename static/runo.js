// runo.tokyo — `/runo`・`/chiketto`・`/blog`・`/ec`共通のクライアントJS。
// 元は`src/meta_index.rs`の`render_page`/`render_single_project_page`が
// それぞれ本文中に文字列組み立てでほぼ同一の<script>を埋め込んでいたもの
// (差分は5個のi18nラベル文字列のみ)を、この1ファイルへ切り出したもの。
// ラベルはサーバー側が各ページのHTMLへ`<script>window.RUNO_I18N = {...}</script>`
// というデータアイランドとして埋め込み、このファイルはそれを読む
// (このファイル自体はどの言語のページからも同じ内容で配信できる)。
(function () {
  var I18N = window.RUNO_I18N || {};
  var LOADING = I18N.fetch_loading || '取得中…';
  var FAIL = I18N.fetch_fail || '取得できませんでした。';
  var STARS_LABEL = I18N.field_stars || '⭐ Stars';
  var UPDATED_LABEL = I18N.field_updated || '最終更新';
  var BRANCH_LABEL = I18N.field_default_branch || '既定ブランチ';

  async function fetchLiveInfo(repo, resultEl) {
    if (!resultEl) return;
    resultEl.textContent = LOADING;
    try {
      const res = await fetch('https://api.github.com/repos/' + repo, {
        headers: { 'Accept': 'application/vnd.github+json' },
      });
      if (!res.ok) throw new Error('status ' + res.status);
      const data = await res.json();
      const stars = data.stargazers_count;
      const updated = data.pushed_at || data.updated_at;
      const branch = data.default_branch;
      resultEl.innerHTML = '<span class="ok">' + STARS_LABEL + ': ' + stars +
        ' / ' + UPDATED_LABEL + ': ' + new Date(updated).toLocaleDateString() +
        ' / ' + BRANCH_LABEL + ': ' + branch + '</span>';
    } catch (e) {
      resultEl.textContent = FAIL;
    }
  }

  // リポジトリ直下のファイル/ディレクトリ一覧を、GitHub Contents API から
  // 取得し、GitHubの「Code」タブのファイル一覧そのままの見た目(📁/📄
  // アイコン+ファイル名+GitHub上の実ページへのリンク)で表示する。
  // 公開リポジトリの内容取得なので未認証で取得可能・ログイン不要——
  // リンク先も`github.com`の通常ページ(`html_url`)であり、閲覧側もログイン不要。
  async function fetchFileTree(repo, treeEl) {
    if (!treeEl) return;
    try {
      const res = await fetch('https://api.github.com/repos/' + repo + '/contents/', {
        headers: { 'Accept': 'application/vnd.github+json' },
      });
      if (!res.ok) throw new Error('status ' + res.status);
      const entries = await res.json();
      if (!Array.isArray(entries)) throw new Error('unexpected response shape');
      entries.sort((a, b) => {
        if (a.type !== b.type) return a.type === 'dir' ? -1 : 1;
        return a.name.localeCompare(b.name);
      });
      treeEl.innerHTML = entries.map(e => {
        const icon = e.type === 'dir' ? '📁' : '📄';
        const name = e.name.replace(/[&<>"']/g, c => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[c]));
        return '<li>' + icon + ' <a href="' + e.html_url + '" target="_blank" rel="noopener">' + name + '</a></li>';
      }).join('');
    } catch (e) {
      treeEl.innerHTML = '<li class="tree-error">' + FAIL + '</li>';
    }
  }

  // GitHubのREADME.md本文(先頭の意味のある段落)をその場で取得し、紹介文を
  // 常に最新のGitHubの内容と同期させる。取得できない場合はビルド時点の
  // 静的な紹介文(data-fallback-role)のまま変更しない。
  function escapeHtmlLocal(s) {
    return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  }

  async function fetchReadmeSummary(repo, roleEl) {
    if (!roleEl) return;
    for (const branch of ['main', 'master']) {
      try {
        const res = await fetch('https://raw.githubusercontent.com/' + repo + '/' + branch + '/README.md');
        if (!res.ok) continue;
        const text = await res.text();
        const lines = text.split('\n');
        let summary = '';
        for (const line of lines) {
          const t = line.trim();
          if (t === '' || t.startsWith('#') || t.startsWith('>') || t.startsWith('[![') || t.startsWith('![')) continue;
          summary = t;
          break;
        }
        if (summary) {
          roleEl.innerHTML = escapeHtmlLocal(summary) + ' <span class="pending">(GitHub README同期)</span>';
        }
        return;
      } catch (e) { /* try next branch */ }
    }
  }

  function wireCard(card, delayMs) {
    const repo = card.getAttribute('data-repo');
    if (!repo) return;
    const resultEl = document.querySelector('.live-fetch-result[data-repo-result="' + repo + '"]');
    const treeEl = document.querySelector('.file-tree[data-repo-tree="' + repo + '"]');
    const roleEl = document.querySelector('.role[data-repo-role="' + repo + '"]');
    const btn = card.querySelector('.live-fetch-btn[data-repo="' + repo + '"]');
    const run = () => {
      fetchLiveInfo(repo, resultEl);
      fetchFileTree(repo, treeEl);
      fetchReadmeSummary(repo, roleEl);
    };
    if (btn) btn.addEventListener('click', run);
    if (resultEl && treeEl) {
      setTimeout(run, delayMs);
    }
  }

  document.addEventListener('DOMContentLoaded', function () {
    // TOPページの「/runo」リンクをクリックしてこのページへ遷移してきた瞬間、
    // 手動でボタンを押さなくても全リポジトリの最新情報(⭐Star数・最終更新・
    // 既定ブランチ・README紹介文)と、GitHubそのままのファイル一覧を
    // 自動的に一括取得する。GitHub API(未認証)のレート制限に配慮し、
    // 同時発火ではなく400ms間隔で順に発火させる(`/runo`のような複数
    // カードを持つページ向け)。単一プロジェクトのページ(`/chiketto`等)では
    // カードが1枚だけなので、実質100ms後に1回だけ発火する。
    document.querySelectorAll('.repo-card').forEach((card, i) => {
      wireCard(card, i * 400 + 100);
    });
  });
})();
