# Quick Start Guide

Create your own educational game in minutes - no Rust installation required!

## 1. Clone the Template

```bash
git clone https://github.com/YOUR_ORG/just-learn-just my-game
cd my-game
```

## 2. Install Just (Command Runner)

If you don't have `just` installed:

```bash
# macOS
brew install just

# Windows (scoop)
scoop install just

# Linux
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to ~/.local/bin
```

## 3. Build Your Game

```bash
# Download the right tools for your platform
just setup

# Build
just build
```

That's it! Open `index.html` in your browser to play.

## 4. Customize Your Content

### Add Your Own Questions

1. Create a new JSON file in the `data/` directory:
   ```json
   {
     "metadata": {
       "title": "My Custom Quiz",
       "description": "Test your knowledge!",
       "mode": "easy",
       "targetAge": "10+",
       "subject": "General Knowledge"
     },
     "questions": [
       {
         "id": "q1",
         "question": "What is 2 + 2?",
         "choices": ["3", "4", "5", "6"],
         "correctAnswer": 1,
         "explanation": "2 + 2 equals 4. This is basic addition!"
       }
     ]
   }
   ```

2. Validate your questions:
   ```bash
   just validate
   ```

3. Rebuild the game:
   ```bash
   just build
   ```

### Customize the Look

Edit `css/style.css` to change colors, fonts, or layout. The build process automatically includes your changes.

### Test Your Game

```bash
# Run data validation + browser tests (headless)
just test
```

## Common Workflows

### Development Cycle
```bash
# 1. Edit your questions in data/
# 2. Validate them
just validate

# 3. Build the game
just build

# 4. Open index.html in your browser
# 5. Repeat!
```

### Before Sharing
```bash
# Run all tests to ensure everything works
just test

# Your game is now in index.html - share this single file!
```

## No Installation Required!

The beauty of this system:
- **For you (developer)**: Just need `just` and `git`
- **For players**: Just need a web browser
- **No Node.js, no Python, no Rust required**

The pre-built tools are automatically downloaded for your platform when you run `just setup`.

## Platforms Supported

✅ Windows (x64)  
✅ macOS (Intel & Apple Silicon)  
✅ Linux (x64 & ARM64)  

## Next Steps

- Read the [full documentation](README.md)
- Learn about [question formats](data/schema.json)
- Explore [advanced customization](docs/CUSTOMIZATION.md)
- Share your game as a single HTML file!