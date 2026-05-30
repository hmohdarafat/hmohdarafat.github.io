#!/usr/bin/env bash
set -euo pipefail

rm -rf docs
dx bundle --platform web --release --out-dir docs

# Dioxus currently places web output under docs/public when using --out-dir docs.
# GitHub Pages needs index.html directly inside docs/.
if [ -d docs/public ]; then
  find docs -mindepth 1 -maxdepth 1 ! -name public -exec rm -rf {} +
  cp -a docs/public/. docs/
  rm -rf docs/public
fi

# Required for client-side routes like /resume and /blog/my-post on GitHub Pages.
cp docs/index.html docs/404.html

touch docs/.nojekyll

echo "Built GitHub Pages output in ./docs"
