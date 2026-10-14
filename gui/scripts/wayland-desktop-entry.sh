#!/bin/sh
set -e

root=$(cd "$(dirname "$0")/.." && pwd)
bin=${1:-cyrano-gui}
data=${XDG_DATA_HOME:-$HOME/.local/share}
apps="$data/applications"
icons="$data/icons/hicolor/256x256/apps"

mkdir -p "$apps" "$icons"
cp "$root/assets/icon-256.png" "$icons/cyrano.png"
sed "s|^Exec=.*|Exec=$bin|" "$root/assets/cyrano.desktop" > "$apps/cyrano.desktop"

command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "$apps" || true
command -v kbuildsycoca6 >/dev/null 2>&1 && kbuildsycoca6 >/dev/null 2>&1 || true
