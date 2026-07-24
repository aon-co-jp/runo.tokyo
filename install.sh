#!/usr/bin/env bash
# runo.tokyo installer for Linux.
# ソースからビルドしてインストールする(ビルド済みバイナリの署名/配布
# パイプラインは今回未整備、他のRS-系リポジトリと同じ暫定方式)。
# 必要なもの: git, curl, Rustツールチェーン(無ければrustup経由で自動導入)。
set -euo pipefail

INSTALL_DIR="${RUNO_TOKYO_HOME:-$HOME/.local/runo-tokyo}"
BIN_DIR="${RUNO_TOKYO_BIN_DIR:-$HOME/.local/bin}"
REPO_URL="https://github.com/aon-co-jp/runo.tokyo.git"

echo "== runo.tokyo installer (Linux) =="

if ! command -v cargo >/dev/null 2>&1; then
  echo "Rust toolchain not found; installing via rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  # shellcheck disable=SC1090
  source "$HOME/.cargo/env"
fi

if [ ! -d "$INSTALL_DIR" ]; then
  git clone "$REPO_URL" "$INSTALL_DIR"
else
  echo "Existing install found at $INSTALL_DIR, pulling latest..."
  git -C "$INSTALL_DIR" pull --ff-only
fi

cd "$INSTALL_DIR"
cargo build --release

mkdir -p "$BIN_DIR"
cp "$INSTALL_DIR/target/release/runo-tokyo" "$BIN_DIR/runo-tokyo"

echo
echo "Installed: $BIN_DIR/runo-tokyo"
case ":$PATH:" in
  *":$BIN_DIR:"*) ;;
  *) echo "Note: $BIN_DIR is not on your PATH. Add it, e.g.: export PATH=\"$BIN_DIR:\$PATH\"" ;;
esac

echo
echo "systemdサービスとして常時稼働させる場合の例:"
echo "  sudo tee /etc/systemd/system/runo-tokyo.service <<'UNIT'"
echo "  [Unit]"
echo "  Description=runo.tokyo web server"
echo "  After=network.target"
echo "  [Service]"
echo "  ExecStart=$BIN_DIR/runo-tokyo"
echo "  Restart=always"
echo "  [Install]"
echo "  WantedBy=multi-user.target"
echo "  UNIT"
echo "  sudo systemctl daemon-reload && sudo systemctl enable --now runo-tokyo"
