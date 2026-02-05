#!/usr/bin/env bash
set -euo pipefail

# Simple installer script: build, and copy binary to ~/.local/bin (Linux/Mac) or %LocalAppData%\Programs\terminal-pet on Windows via powershell.

if [[ "$(uname -s)" == "Linux" ]] || [[ "$(uname -s)" == "Darwin" ]]; then
  cargo build --release
  mkdir -p "$HOME/.local/bin"
  cp target/release/terminal-pet "$HOME/.local/bin/terminal-pet"
  chmod +x "$HOME/.local/bin/terminal-pet"
  echo "Installed to $HOME/.local/bin/terminal-pet"
else
  echo "For Windows, use cargo build --release and copy target\\release\\terminal-pet.exe to %LocalAppData%\\Programs\\terminal-pet\\"
fi
