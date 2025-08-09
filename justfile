# Just The Game - Build System (no Python)

# Use bash for recipes
set shell := ["bash", "-c"]

# Default: list commands
default:
    @just --list

# Platform detection for tool downloads
os := os()
arch := arch()
platform := if os == "linux" { if arch == "x86_64" { "linux-x64" } else if arch == "aarch64" { "linux-aarch64" } else { "linux-" + arch } } else if os == "macos" { if arch == "x86_64" { "macos-x64" } else { "macos-aarch64" } } else if os == "windows" { "windows-x64" } else { error("Unsupported platform: " + os + "-" + arch) }
bin_ext := if os == "windows" { ".exe" } else { "" }
tools_dir := ".tools"

# -----------------------------------------------------------------------------
# End-user workflow
# -----------------------------------------------------------------------------

# setup: Download prebuilt tools into .tools
# TODO: Wire this to your GitHub Releases. See below for placeholder.
setup:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p "{{tools_dir}}"
    echo "📦 Setting up tools for {{platform}}..."
    OWNER_REPO="${TOOLS_OWNER_REPO:-}"
    VERSION="${TOOLS_VERSION:-latest}"
    AUTH_TOKEN="${GITHUB_TOKEN:-${GH_TOKEN:-}}"
    if [[ -z "$OWNER_REPO" ]]; then
      echo "❌ TO DO: Set TOOLS_OWNER_REPO=owner/repo or edit the justfile to configure download URL."
      echo "   Alternatively, build locally: 'just tools:build tools:install-local'"
      exit 1
    fi
    ARCHIVE_EXT={{ if os == "windows" { "zip" } else { "tar.gz" } }}
    if [[ "$VERSION" == "latest" ]]; then
      VERSION=$(curl -s https://api.github.com/repos/$OWNER_REPO/releases/latest | grep -oE '"tag_name":\s*"[^"]+"' | cut -d '"' -f4 || true)
    fi
    if [[ -z "$VERSION" ]]; then
      echo "❌ Could not resolve version from GitHub Releases for $OWNER_REPO"
      echo "   If your release is a DRAFT, set TOOLS_VERSION to its tag and GITHUB_TOKEN to an access token."
      echo "   Or build locally: 'just tools-build tools-install-local'"
      exit 1
    fi
    URL="https://github.com/$OWNER_REPO/releases/download/${VERSION}/just-learn-just-tools-{{platform}}.${ARCHIVE_EXT}"
    echo "📥 Downloading: $URL"
    CURL_AUTH=()
    if [[ -n "$AUTH_TOKEN" ]]; then CURL_AUTH=( -H "Authorization: Bearer $AUTH_TOKEN" ); fi
    if [[ "{{os}}" == "windows" ]]; then
      curl -L "${CURL_AUTH[@]}" -o "{{tools_dir}}/tools.zip" "$URL" && (cd "{{tools_dir}}" && unzip -q tools.zip && rm tools.zip) || { echo "❌ Download or extract failed"; exit 1; }
    else
      curl -L "${CURL_AUTH[@]}" "$URL" | tar -xz -C "{{tools_dir}}" || { echo "❌ Download or extract failed"; exit 1; }
    fi
    if [[ "{{os}}" != "windows" ]]; then chmod +x "{{tools_dir}}"/* || true; fi
    echo "✅ Tools installed in {{tools_dir}}"

# clean: remove generated artifacts
clean:
    rm -f index.html
    @echo "Cleaned generated files"

# Internal guard: ensure tools exist and fail fast otherwise
ensure-tools:
    #!/usr/bin/env bash
    set -euo pipefail
    for bin in bundle validate test-runner; do
      if [[ ! -x "{{tools_dir}}/${bin}{{bin_ext}}" ]]; then
        echo "❌ Missing tool: {{tools_dir}}/${bin}{{bin_ext}}"
        echo "   Run 'just setup' (downloads from GitHub Releases)"
        echo "   Or build locally: 'just tools:build tools:install-local'"
        exit 1
      fi
    done

# build: validate JSON then bundle (fails fast if tools missing)
build: ensure-tools
    {{tools_dir}}/validate{{bin_ext}}
    {{tools_dir}}/bundle{{bin_ext}}

# test: validate data and run tests (headless; run one easy and one hard)
test:
    {{tools_dir}}/validate{{bin_ext}}
    {{tools_dir}}/test-runner{{bin_ext}} --headless --first-per-mode

# test-visible: run tests with visible browser and verbose console (one easy + one hard)
test-visible:
    {{tools_dir}}/validate{{bin_ext}}
    {{tools_dir}}/test-runner{{bin_ext}} --verbose --first-per-mode

# validate: manual validation without extra checks
validate:
    {{tools_dir}}/validate{{bin_ext}}

# -----------------------------------------------------------------------------
# Tooling for contributors (local builds of the Rust tools)
# -----------------------------------------------------------------------------

tools-build:
    cargo build --release --bins

tools-install-local:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p "{{tools_dir}}"
    for bin in bundle validate test-runner; do
      src="target/release/${bin}{{bin_ext}}"
      if [[ ! -f "$src" ]]; then echo "❌ Missing built binary: $src"; exit 1; fi
      cp "$src" "{{tools_dir}}/";
    done
    if [[ "{{os}}" != "windows" ]]; then chmod +x "{{tools_dir}}"/* || true; fi
    echo "✅ Installed local tools into {{tools_dir}}"