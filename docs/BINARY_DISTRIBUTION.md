# Binary Distribution Guide

This project provides pre-compiled binaries for all major platforms, so you don't need to install Rust to use the build tools.

## Automatic Download

The easiest way to get the tools is:

```bash
# Download the appropriate binaries for your platform
just setup
```

The tools will be downloaded to the `.tools/` directory and used automatically by all `just` commands.

## Manual Download

You can also manually download the binaries from the [releases page](https://github.com/YOUR_ORG/just-learn-just/releases).

### Available Platforms

| Platform | Architecture | Download |
|----------|-------------|----------|
| Linux | x64 | `just-learn-just-tools-linux-x64.tar.gz` |
| Linux | x64 (static) | `just-learn-just-tools-linux-x64-musl.tar.gz` |
| Linux | ARM64 | `just-learn-just-tools-linux-aarch64.tar.gz` |
| macOS | Intel | `just-learn-just-tools-macos-x64.tar.gz` |
| macOS | Apple Silicon | `just-learn-just-tools-macos-aarch64.tar.gz` |
| Windows | x64 | `just-learn-just-tools-windows-x64.zip` |

### Manual Installation

1. Download the appropriate archive for your platform
2. Extract the archive:
   ```bash
   # Linux/macOS
   tar -xzf just-learn-just-tools-*.tar.gz
   
   # Windows
   # Use Windows Explorer or:
   unzip just-learn-just-tools-*.zip
   ```
3. Add the extracted directory to your PATH, or use the tools directly:
   ```bash
   # Linux/macOS
   ./validate
   ./bundle
   ./test-runner --help
   
   # Windows
   validate.exe
   bundle.exe
   test-runner.exe --help
   ```

## Tool Descriptions

### validate
Validates all JSON question files against the schema:
```bash
# Validate all question files
validate

# Or using just:
just validate
```

### bundle
Bundles all assets (CSS, JS, JSON) into a single HTML file:
```bash
# Build the game
bundle

# Or using just:
just build
```

### test-runner
Runs browser-based tests using Chromium:
```bash
# Run tests in headless mode
test-runner --headless

# Direct options
test-runner --help
```

## Building from Source

If you prefer to build the tools from source or need to modify them:

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build all tools
cargo build --release

# The binaries will be in target/release/
```

## Environment Variables

- `TOOLS_VERSION`: Override the version of tools to download (default: latest)
  ```bash
  TOOLS_VERSION=v1.2.3 just setup
  ```

## Troubleshooting

### Download fails
- Check your internet connection
- Verify the GitHub repository URL in the justfile is correct
- Try downloading manually from the releases page

### Tools don't run
- **Linux**: Make sure the binaries are executable: `chmod +x validate bundle test-runner`
- **macOS**: You may need to allow the binaries in System Settings > Privacy & Security
- **Windows**: Ensure you're using the `.exe` versions of the tools

### Wrong architecture
- The justfile tries to detect your platform automatically
- If detection fails, download the correct version manually
- For Apple Silicon Macs, use the `aarch64` version, not `x64`

### Missing dependencies
- The Linux musl builds are fully static and should work on any Linux
- Standard Linux builds may require glibc 2.17 or newer
- Windows builds require Visual C++ Runtime (usually already installed)