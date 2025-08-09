# Data-Driven Browser Test Runner

Note: CI smoke PR to verify workflows. No functional changes.

The `test-runner` is a Rust-based browser automation tool that dynamically discovers and tests all question sets in the `data/` directory without requiring any hardcoded test logic.

## Features

- **Automatic Test Discovery**: Scans `data/` for all `questions*.json` files
- **Mode-Aware Testing**: Handles both "easy" (retry allowed) and "hard" (immediate result) modes
- **Data-Driven**: Reads correct answers from JSON files - no hardcoded values
- **Browser Automation**: Uses Chromium via chromiumoxide for real browser testing
- **Flexible Output**: Colored terminal output with summary statistics
- **CI/CD Ready**: Supports headless mode and returns proper exit codes

## Usage

```bash
# Typical flow
just build    # validate + bundle to index.html
just test     # validate + run tests in headless mode

# Direct invocation with options (using downloaded tools)
.tools/test-runner --help
.tools/test-runner --headless
.tools/test-runner --filter="easy"
.tools/test-runner --html-path="./index.html"

# If you want to see the browser window
.tools/test-runner             # omit --headless
```

## How It Works

1. **Discovery Phase**
   - Scans `data/` directory for `questions*.json` files
   - Parses each file to extract metadata and questions
   - Builds test cases for each discovered question set

2. **Test Execution**
   - For each question set:
     - Loads the game and verifies start screen
     - Clicks the tile matching the question set
     - Answers questions based on `correctAnswer` field from JSON
     - Tests mode-specific behavior on question 2
     - Verifies game flow through to completion
     - Tests "Play Again" returns to start

3. **Mode-Specific Testing**
   - **Easy Mode**: On question 2, clicks wrong answer first to verify retry behavior
   - **Hard Mode**: On question 2, clicks wrong answer to verify immediate result screen

## Example Output

```
🎮 Starting Game Tests

📋 Found 3 question set(s) to test:
  • questions_just_easy: Just Command Runner - Getting Started (mode: easy)
  • questions_just_medium: Just Command Runner - Intermediate (mode: hard)
  • questions_just_hard: Just Command Runner - Expert Murderous Quiz (mode: hard)

🧪 Testing: Just Command Runner - Getting Started (easy)
    Testing start screen... ✓
    Testing question 1/10... ✓
    Testing question 2/10... ✓ (tested retry behavior)
    Testing question 3/10... ✓
    ...
    Testing play again... ✓
  ✅ Passed

📊 Test Summary
================================
✅ PASSED Just Command Runner - Getting Started (easy)
✅ PASSED Just Command Runner - Intermediate (hard)
✅ PASSED Just Command Runner - Expert Murderous Quiz (hard)

Results: 3/3 question sets passed
🎉 Success! All tests passed!
```

## Adding New Question Sets

The test runner automatically discovers new question sets. Simply:

1. Add a new `questions_*.json` file to the `data/` directory
2. Ensure it follows the required schema (validate with `just validate`)
3. Run `just build` to bundle it into the HTML
4. Run `just test` - your new question set will be tested automatically

No test code changes required!

## Command Line Options

- `--headless`: Run browser in headless mode (no visible window)
- `--filter <PATTERN>`: Only test question sets matching the pattern
- `--verbose`: Show browser console logs during test execution
- `--html-path <PATH>`: Path to the HTML file to test (default: ./index.html)
- `--timeout <SECONDS>`: Timeout for page operations (default: 10)

## Requirements

- Chromium or Chrome browser installed
- Built `index.html` file (run `just build` first)
- Question files in `data/` directory following the schema

## Implementation Details

The test runner is implemented in Rust using:
- `chromiumoxide`: For browser automation
- `tokio`: For async runtime
- `clap`: For command-line argument parsing
- `colored`: For terminal output formatting
- `serde`: For JSON parsing

The source code is in `src/bin/test_runner.rs`.