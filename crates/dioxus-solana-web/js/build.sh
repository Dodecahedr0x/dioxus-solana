#!/usr/bin/env bash
# Bundle @solana-mobile/wallet-standard-mobile for include_str! in mwa.rs.
set -euo pipefail
cd "$(dirname "$0")"
VERSION=0.5.2
rm -rf node_modules package.json package-lock.json
npm init -y >/dev/null
npm install --no-fund --no-audit "@solana-mobile/wallet-standard-mobile@${VERSION}"
npx --yes esbuild \
  "node_modules/@solana-mobile/wallet-standard-mobile/lib/esm/index.browser.js" \
  --bundle \
  --format=iife \
  --global-name=SolanaMobileWalletStandard \
  --platform=browser \
  --minify \
  --banner:js="/* @solana-mobile/wallet-standard-mobile@${VERSION} — regenerate with js/build.sh */" \
  --footer:js="globalThis.SolanaMobileWalletStandard=SolanaMobileWalletStandard;" \
  --outfile=wallet-standard-mobile.iife.js
rm -rf node_modules package.json package-lock.json
echo "wrote wallet-standard-mobile.iife.js (@solana-mobile/wallet-standard-mobile@${VERSION})"
