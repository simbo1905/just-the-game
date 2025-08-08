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
│   └── ui.js          # User interface handlers
├── data/
│   ├── schema.json    # JSON schema for knowledge base
│   └── questions.json # Game questions and answers
├── css/
│   └── style.css      # Basic styling
└── tests/
    └── puppeteer/     # Automated tests
```

## Development

This project uses [Just](https://github.com/casey/just) as its command runner.

### Quick Start

```bash
# Install Just (if not already installed)
# macOS: brew install just
# Linux: See https://github.com/casey/just#installation

# Set up development environment
just setup
source venv/bin/activate  # or venv\Scripts\activate on Windows
just install

# Build and test
just build    # Bundle everything into index.html
just test     # Run all tests (builds automatically)
just serve    # Start local server at http://localhost:8000
```

### Available Commands

```bash
just          # List all available commands
just setup    # Create Python virtual environment
just install  # Install Python dependencies
just build    # Bundle all resources into index.html
just test     # Run all tests (unit then integration)
just test-unit        # Run only unit tests
just test-integration # Run only integration tests
just test-one FILE    # Run a single test file
just clean    # Remove generated files
just verify   # Check build info
just serve    # Start HTTP server for testing
just dev      # Build and start server
```

## Build and Test Workflow

The build process bundles all resources (CSS, JS, JSON) into a single `index.html` file:

```bash
just build    # Creates index.html from templates and resources
```

Run tests with guaranteed fresh builds:

```bash
just test     # Automatically builds first, then runs all tests
```

⚠️ WARNING: Tests run against the bundled `index.html` file. Always use `just test` commands which handle building automatically.

Test a single file:
```bash
just test-one tests/unit_test_example.py
```

Verify build info:
```bash
just verify   # Check embedded timestamps and version
```

The pack script embeds build timestamps and version info to verify the build process is working correctly. Check the bottom-right corner of the generated HTML page for build info, and browser console for debug logs.

## Usage

Open `index.html` in any modern web browser.
