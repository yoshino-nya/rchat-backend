#!/bin/bash
set -e

BASE_DIR="$(cd "$(dirname "$0")" && pwd)"

mkdir -p "$BASE_DIR/uploads/avatars"

sudo systemctl start postgresql

cd "$BASE_DIR/../rchat-frontend"
npm run dev &

cd "$BASE_DIR/../chat"
cargo watch -x run