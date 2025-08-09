# Just The Game

An educational flash-card game to learn the Just command runner. Practice concepts, commands, and best practices through interactive quizzes.

## Features

- Multi-choice flash-card game format
- Serverless - runs as a single HTML file
- Separates knowledge base (JSON) from game engine
- Compatible with Chrome OS, macOS, Windows, and mobile devices
- Minimal dependencies, modern web standards

## Project Structure

```
/
├── index.html          # Main game interface
├── js/
│   ├── game-engine.js  # Core game logic
│   └── ui.js           # User interface handlers
├── data/
│   ├── schema.json     # JSON schema for knowledge base
│   └── questions_*.json# Game questions and answers
├── css/
│   └── style.css       # Basic styling
└── templates/
    └── index.hbs       # Handlebars template used by the bundler
```

## Development

This project uses [Just](https://github.com/casey/just) as its command runner and small Rust tools for validation, bundling, and testing. No Python is required.

### Prerequisites

- [Just](https://github.com/casey/just#installation)

### Quick Start

```bash
# 1) Download platform tools into .tools (from GitHub Releases)
just setup   # fetches binaries from the latest GitHub Release for this repo

# 2) Clean, build, and test
just clean
just build     # Validates data and generates index.html
just test      # Runs the Rust browser tests

# Optional
just validate  # Validate JSON files against schema
```

### Available Commands

```bash
just           # List commands
just setup     # Download platform-specific tools into .tools (from releases)
just clean     # Remove generated files
just build     # Validate JSON and bundle assets into index.html
just validate  # Validate JSON files against schema
just test      # Run browser tests via Rust test-runner
```

## Build and Test Workflow

The build process validates JSON files and bundles all resources (CSS, JS, JSON) into a single `index.html` file:

```bash
just build     # Validates and bundles
```

The tools are:
- `validate` - Validates JSON question files against the schema
- `bundle` - Bundles resources into a single HTML file using Handlebars templating

Run tests:

```bash
just test      # Runs the Rust test-runner against index.html
```

## Usage

Open `index.html` in any modern web browser.

## Contributing (Tools Development)

If you need to develop the tools themselves, use:

```bash
just tools:build           # Build Rust tools locally
just tools:install-local   # Copy built tools into .tools/
```

Then re-run `just build` and `just test`.

## Binary Distribution and Releases

This project publishes prebuilt binaries for `validate`, `bundle`, and `test-runner` as GitHub Release assets.

- End users run `just setup` to download platform-specific binaries into `.tools/` without compiling Rust.
- Power users can still build from source with `just tools-build tools-install-local`.

### Release Flow (Maintainers)

Checklist for a preview release:

1. Ensure CI green on `main`.
2. Create a tag: `git tag v0.0.1 && git push --tags`.
3. GitHub Actions workflow `Build and Release Tools` will:
   - Build binaries for Linux/macOS/Windows targets from the tag
   - Package artifacts and create a GitHub Release
   - Mark it as a prerelease with auto-generated release notes
4. Verify release assets appear on the tag’s release page.

To promote to latest for end users (so `just setup` finds it):

5. Edit the release on GitHub and uncheck "This is a pre-release" to mark it as a final release.
6. Optionally create a new tag for a stable release.

`just setup` uses the GitHub API to fetch the latest non-prerelease release and downloads the archive matching your platform (e.g., `just-learn-just-tools-macos-aarch64.tar.gz`).
