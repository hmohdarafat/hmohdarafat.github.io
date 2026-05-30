$ErrorActionPreference = "Stop"

if (Test-Path docs) {
    Remove-Item docs -Recurse -Force
}

dx bundle --platform web --release --out-dir docs

# Dioxus currently places web output under docs/public when using --out-dir docs.
# GitHub Pages needs index.html directly inside docs/.
if (Test-Path docs\public) {
    Get-ChildItem docs -Force | Where-Object { $_.Name -ne "public" } | Remove-Item -Recurse -Force
    Copy-Item docs\public\* docs -Recurse -Force
    Remove-Item docs\public -Recurse -Force
}

# Required for client-side routes like /resume and /blog/my-post on GitHub Pages.
Copy-Item docs\index.html docs\404.html -Force

New-Item -ItemType File -Path docs\.nojekyll -Force | Out-Null

Write-Host "Built GitHub Pages output in ./docs"
