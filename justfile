# Just The Game - Build System (no Python)
# =========================================
#
# TOOLING ARCHITECTURE:
# This justfile implements a cascading tool acquisition strategy:
# 1. Download from template repository (simbo1905/just-the-game)
# 2. Download from current repository (if different)
# 3. Build locally from source (requires Rust)
#
# GITHUB ACTIONS DEPLOYMENT:
# The .github/workflows/build-and-release-tools.yaml workflow handles multi-platform builds.
# It builds for: linux-x64, linux-aarch64, macos-x64, macos-aarch64, windows-x64
# 
# To deploy tools:
# 1. Push a version tag (e.g., git tag v1.0.0 && git push --tags)
# 2. GitHub Actions will build all platforms and create a release
# 
# IMPORTANT: Building and deploying happens ONLY in GitHub Actions, NOT locally.
# Local builds (just tools-build) are only for development/testing.
#
# TEMPLATE PATTERN:
# Everyone gets the same workflow file, so anyone can become a tool provider by:
# 1. Forking/using this template
# 2. Pushing a version tag to trigger builds
# 3. Others can then use their repo as a tool source

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

# setup: Download prebuilt tools with cascading fallback
# This command tries to download tools in order:
# 1. From the original template repo (simbo1905/just-the-game)
# 2. From your current repo (auto-detected from git remote)
# 3. Build from source if no prebuilt tools are available
# You can override with: TEMPLATE_REPO=owner/repo or USER_REPO=owner/repo
setup:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p "{{tools_dir}}"
    echo "📦 Setting up tools for {{platform}}..."
    
    # Helper function to extract owner/repo from git URL
    extract_owner_repo() {
        echo "$1" | sed -E 's|.*/([^/]+)/([^/]+)(\.git)?$|\1/\2|'
    }
    
    # Helper function to try downloading from a repo
    try_download() {
        local owner_repo="$1"
        local version="${2:-latest}"
        echo "🔍 Trying $owner_repo..."
        
        local auth_token="${GITHUB_TOKEN:-${GH_TOKEN:-}}"
        local archive_ext={{ if os == "windows" { "zip" } else { "tar.gz" } }}
        
        # Resolve version
        if [[ "$version" == "latest" ]]; then
            local version_url="https://api.github.com/repos/$owner_repo/releases/latest"
            local curl_auth=()
            if [[ -n "$auth_token" ]]; then curl_auth=( -H "Authorization: Bearer $auth_token" ); fi
            version=$(curl -s "${curl_auth[@]}" "$version_url" | grep -oE '"tag_name":\s*"[^"]+"' | cut -d '"' -f4 || true)
        fi
        
        if [[ -z "$version" ]]; then
            echo "   ❌ No releases found"
            return 1
        fi
        
        local url="https://github.com/$owner_repo/releases/download/${version}/just-learn-just-tools-{{platform}}.${archive_ext}"
        echo "   📥 Downloading version $version..."
        
        local curl_auth=()
        if [[ -n "$auth_token" ]]; then curl_auth=( -H "Authorization: Bearer $auth_token" ); fi
        
        # Try download
        if [[ -n "$auth_token" ]]; then
            curl_result=$(curl -f -L "${curl_auth[@]}" -o "{{tools_dir}}/tools.${archive_ext}" "$url" 2>/dev/null && echo "success" || echo "fail")
        else
            curl_result=$(curl -f -L -o "{{tools_dir}}/tools.${archive_ext}" "$url" 2>/dev/null && echo "success" || echo "fail")
        fi
        
        if [[ "$curl_result" == "success" ]]; then
            # Extract
            if [[ "{{os}}" == "windows" ]]; then
                (cd "{{tools_dir}}" && unzip -q "tools.${archive_ext}" && rm "tools.${archive_ext}") || return 1
            else
                tar -xz -C "{{tools_dir}}" -f "{{tools_dir}}/tools.${archive_ext}" && rm "{{tools_dir}}/tools.${archive_ext}" || return 1
            fi
            
            if [[ "{{os}}" != "windows" ]]; then chmod +x "{{tools_dir}}"/* || true; fi
            echo "   ✅ Success!"
            return 0
        else
            echo "   ❌ Download failed"
            return 1
        fi
    }
    
    # Cascading fallback strategy
    TEMPLATE_REPO="${TEMPLATE_REPO:-simbo1905/just-learn-just}"
    USER_REPO="${USER_REPO:-}"
    
    # If USER_REPO not set, try to detect from git remote
    if [[ -z "$USER_REPO" ]] && command -v git >/dev/null 2>&1; then
        REMOTE_URL=$(git remote get-url origin 2>/dev/null || true)
        if [[ -n "$REMOTE_URL" ]]; then
            USER_REPO=$(extract_owner_repo "$REMOTE_URL")
        fi
    fi
    
    # Try cascading downloads
    echo "🔄 Attempting cascading tool download..."
    
    # 1. Try canonical template repo
    if try_download "$TEMPLATE_REPO"; then
        echo "✅ Tools installed from template repository"
        exit 0
    fi
    
    # 2. Try current user's repo (if different from template)
    if [[ -n "$USER_REPO" ]] && [[ "$USER_REPO" != "$TEMPLATE_REPO" ]]; then
        if try_download "$USER_REPO"; then
            echo "✅ Tools installed from current repository"
            exit 0
        fi
    fi
    
    # 3. Final fallback: build from source
    echo "⚠️  No prebuilt tools available, attempting local build..."
    if command -v cargo >/dev/null 2>&1; then
        just tools-build tools-install-local
    else
        echo "❌ Cannot build: Rust/Cargo not installed"
        echo "   Install Rust from https://rustup.rs/"
        exit 1
    fi

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
        echo "   Run 'just setup' to download/build tools"
        exit 1
      fi
    done

# build: validate JSON then bundle (fails fast if tools missing)
build: ensure-tools
    {{tools_dir}}/validate{{bin_ext}}
    {{tools_dir}}/bundle{{bin_ext}}

# test: validate data and run tests (headless; run one easy and one hard)
test: ensure-tools
    {{tools_dir}}/validate{{bin_ext}}
    {{tools_dir}}/test-runner{{bin_ext}} --headless --first-per-mode

# test-visible: run tests with visible browser and verbose console (one easy + one hard)
test-visible: ensure-tools
    {{tools_dir}}/validate{{bin_ext}}
    {{tools_dir}}/test-runner{{bin_ext}} --verbose --first-per-mode

# validate: manual validation without extra checks
validate: ensure-tools
    {{tools_dir}}/validate{{bin_ext}}

# -----------------------------------------------------------------------------
# Tooling for contributors (local builds of the Rust tools)
# 
# IMPORTANT: These commands are for DEVELOPMENT ONLY.
# Production builds happen in GitHub Actions (see .github/workflows/build-and-release-tools.yaml)
# To create a release: git tag v1.0.0 && git push --tags
# -----------------------------------------------------------------------------

# tools-build: Build tools locally (DEVELOPMENT ONLY - production builds use GitHub Actions)
tools-build:
    cargo build --release --bins

# tools-install-local: Install locally built tools (DEVELOPMENT ONLY)
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