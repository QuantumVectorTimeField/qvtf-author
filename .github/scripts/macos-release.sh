#!/usr/bin/env bash
# Tauri 2: https://v2.tauri.app/distribute/sign/macos/
# Apple: https://developer.apple.com/documentation/security/customizing-the-notarization-workflow
set +x
set -euo pipefail
umask 077

fail() { echo "::error::$1" >&2; exit 1; }

credentials="${RUNNER_TEMP:?}/qvtf-apple-signing"
keychain="$credentials/build.keychain-db"

setup() {
  for name in APPLE_CERTIFICATE APPLE_CERTIFICATE_PASSWORD KEYCHAIN_PASSWORD APPLE_API_KEY_CONTENT APPLE_API_ISSUER APPLE_API_KEY; do
    if [[ -z "${!name:-}" ]]; then
      echo "::error::Required repository secret $name is missing."
      exit 1
    fi
  done
  mkdir -m 700 "$credentials"
  security list-keychains -d user > "$credentials/original-keychains"
  printf '%s' "$APPLE_CERTIFICATE" | base64 --decode > "$credentials/certificate.p12"
  printf '%s\n' "$APPLE_API_KEY_CONTENT" > "$credentials/AuthKey.p8"
  # Check the key format without printing its contents.
  openssl pkey -in "$credentials/AuthKey.p8" -noout >/dev/null 2>&1 || {
    echo '::error::APPLE_API_KEY_CONTENT must contain the raw PEM .p8 private key.'
    exit 1
  }
  security create-keychain -p "$KEYCHAIN_PASSWORD" "$keychain"
  security set-keychain-settings -lut 21600 "$keychain"
  security unlock-keychain -p "$KEYCHAIN_PASSWORD" "$keychain"
  security import "$credentials/certificate.p12" -k "$keychain" \
    -P "$APPLE_CERTIFICATE_PASSWORD" -T /usr/bin/codesign >/dev/null 2>&1 || {
    echo '::error::Could not import the Apple signing certificate.'
    exit 1
  }
  security set-key-partition-list -S apple-tool:,apple:,codesign: -s \
    -k "$KEYCHAIN_PASSWORD" "$keychain" >/dev/null 2>&1
  rm -f "$credentials/certificate.p12"
  # Preserve the existing keychain search list and add only this temporary keychain.
  original=()
  while IFS= read -r line; do
    line="${line#*\"}"; line="${line%\"*}"
    [[ -z "$line" ]] || original+=("$line")
  done < "$credentials/original-keychains"
  security list-keychains -d user -s "$keychain" "${original[@]}"
  identities=$(security find-identity -v -p codesigning "$keychain" | \
    sed -n 's/.*"\(Developer ID Application: [^"]*\)".*/\1/p')
  if [[ -z "$identities" || "$identities" == *$'\n'* ]]; then
    echo '::error::Expected exactly one valid Developer ID Application identity with its private key.'
    exit 1
  fi
  # Export the installed identity, not APPLE_CERTIFICATE: Tauri must not re-import it.
  printf '::add-mask::%s\n' "$identities"
  printf 'APPLE_SIGNING_IDENTITY=%s\nAPPLE_API_KEY_PATH=%s\n' \
    "$identities" "$credentials/AuthKey.p8" >> "$GITHUB_ENV"
}

verify_app() {
  local app="$1" details executable
  codesign --verify --deep --strict --verbose=2 "$app"
  details=$(codesign --display --verbose=4 "$app" 2>&1)
  [[ "$details" == *"Authority=$APPLE_SIGNING_IDENTITY"* ]] || fail 'App Developer ID identity mismatch.'
  [[ "$details" == *'runtime'* ]] || fail 'App hardened runtime is missing.'
  [[ "$details" == *'Timestamp='* ]] || fail 'App signature timestamp is missing.'
  executable=$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$app/Contents/Info.plist")
  [[ "$(lipo -archs "$app/Contents/MacOS/$executable")" == arm64 ]] || fail 'App is not ARM64.'
  xcrun stapler validate "$app"
  spctl --assess --type execute --verbose=2 "$app"
}

verify() {
  local bundle app dmg result mountpoint
  bundle='src-tauri/target/aarch64-apple-darwin/release/bundle'
  app="$bundle/macos/QVTF Author.app"
  # Tauri submits the app with notarytool and staples it before constructing the DMG.
  verify_app "$app"
  shopt -s nullglob
  dmgs=("$bundle/dmg/"*.dmg)
  [[ ${#dmgs[@]} -eq 1 ]] || fail 'Expected exactly one DMG.'
  dmg="${dmgs[0]}"
  codesign --verify --strict --verbose=2 "$dmg"
  details=$(codesign --display --verbose=4 "$dmg" 2>&1)
  [[ "$details" == *"Authority=$APPLE_SIGNING_IDENTITY"* ]] || fail 'DMG Developer ID identity mismatch.'
  result="$credentials/dmg-notarization.json"
  xcrun notarytool submit "$dmg" --key "$APPLE_API_KEY_PATH" \
    --key-id "$APPLE_API_KEY" --issuer "$APPLE_API_ISSUER" \
    --wait --timeout 30m --output-format json > "$result"
  if [[ "$(plutil -extract status raw -o - "$result")" != Accepted ]]; then
    echo '::error::Apple did not accept the DMG notarization submission.'
    exit 1
  fi
  xcrun stapler staple "$dmg"
  xcrun stapler validate "$dmg"
  codesign --verify --strict --verbose=2 "$dmg"
  spctl --assess --type open --context context:primary-signature --verbose=2 "$dmg"
  mountpoint="$credentials/mounted-dmg"
  mkdir "$mountpoint"
  hdiutil attach "$dmg" -readonly -nobrowse -mountpoint "$mountpoint" >/dev/null
  verify_app "$mountpoint/QVTF Author.app"
  hdiutil detach "$mountpoint" >/dev/null
  # Repackage the verified, stapled app; never distribute an archive from before verification.
  mkdir -p "$RUNNER_TEMP/verified-macos-release"
  tar -czf "$RUNNER_TEMP/verified-macos-release/QVTF.Author_aarch64.app.tar.gz" \
    -C "$bundle/macos" 'QVTF Author.app'
  cp "$dmg" "$RUNNER_TEMP/verified-macos-release/$(basename "$dmg" | tr ' ' '.')"
  printf '%s\n' 'macOS ARM64: Developer ID signatures verified; app and DMG notarized, stapled, and accepted by Gatekeeper.' >> "$GITHUB_STEP_SUMMARY"
}

cleanup() {
  local failed=0
  if [[ -d "$credentials/mounted-dmg" ]]; then
    hdiutil detach "$credentials/mounted-dmg" >/dev/null 2>&1 || true
  fi
  if [[ -f "$credentials/original-keychains" ]]; then
    original=()
    while IFS= read -r line; do
      line="${line#*\"}"; line="${line%\"*}"
      [[ -z "$line" ]] || original+=("$line")
    done < "$credentials/original-keychains"
    security list-keychains -d user -s "${original[@]}" || failed=1
  fi
  if [[ -f "$keychain" ]]; then
    security delete-keychain "$keychain" || failed=1
  fi
  rm -rf "$credentials"
  return "$failed"
}

case "${1:-}" in
  setup) setup ;;
  verify) verify ;;
  cleanup) cleanup ;;
  *) echo 'Usage: macos-release.sh setup|verify|cleanup' >&2; exit 2 ;;
esac
