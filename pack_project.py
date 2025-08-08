#!/usr/bin/env python3
"""
Pack all external resources into a single index.html file
This creates a truly standalone HTML file that can be opened directly
"""

import json
import os
import subprocess
from datetime import datetime
from pathlib import Path
from jinja2 import Environment, FileSystemLoader

def pack_project():
    """Pack CSS, JS, and JSON data into a single HTML file using Jinja2"""
    
    root_dir = Path(__file__).parent
    template_file = root_dir / "index.html.j2"
    output_file = root_dir / "index.html"
    
    print(f"📦 Packing project from: {root_dir}")
    print(f"📄 Using template: {template_file}")
    
    # Set up Jinja2 environment
    env = Environment(loader=FileSystemLoader(root_dir))
    template = env.get_template("index.html.j2")
    
    # 1. Discover and load all JSON question sets
    data_dir = root_dir / "data"
    print(f"📋 Discovering data files in: {data_dir}")
    
    question_files = []
    for file_path in data_dir.glob("*.json"):
        if file_path.name != "schema.json":
            question_files.append(file_path)
    
    print(f"📋 Found {len(question_files)} question files")
    
    question_sets = {}
    for file_path in question_files:
        print(f"  📄 Loading: {file_path.name}")
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                game_data = json.load(f)
                key = file_path.stem
                question_sets[key] = game_data
        except Exception as e:
            print(f"  ⚠️  Error loading {file_path.name}: {e}")
    
    # 2. Load CSS content
    print("🎨 Loading CSS files...")
    css_content = ""
    css_path = root_dir / "css" / "style.css"
    if css_path.exists():
        print(f"  ✅ Loading: {css_path}")
        with open(css_path, 'r', encoding='utf-8') as f:
            css_content = f.read()
    else:
        print(f"  ⚠️  CSS file not found: {css_path}")
    
    # 3. Load JavaScript content
    print("⚙️  Loading JavaScript files...")
    js_content = ""
    js_files = ["js/game-engine.js", "js/ui.js"]
    
    for js_file in js_files:
        js_path = root_dir / js_file
        if js_path.exists():
            print(f"  ✅ Loading: {js_file}")
            with open(js_path, 'r', encoding='utf-8') as f:
                js_content += f.read() + "\n\n"
        else:
            print(f"  ⚠️  JS file not found: {js_path}")
    
    # 4. Get build info
    build_timestamp = datetime.now().isoformat()
    build_timestamp_unix = int(datetime.now().timestamp())
    
    # Try to get git hash (fallback if git not available)
    try:
        git_hash = subprocess.check_output(
            ['git', 'rev-parse', '--short', 'HEAD'], 
            cwd=root_dir, 
            stderr=subprocess.DEVNULL
        ).decode().strip()
        version = f"git-{git_hash}"
    except (subprocess.CalledProcessError, FileNotFoundError):
        version = "dev-build"
    
    print(f"🏗️  Build timestamp: {build_timestamp}")
    print(f"🏗️  Version: {version}")
    
    # 5. Prepare template data
    question_sets_json = json.dumps(question_sets, indent=2)
    default_question_set = None
    
    # Use "questions" as default if available, otherwise first available set
    if "questions" in question_sets:
        default_question_set = json.dumps(question_sets["questions"], indent=2)
    elif question_sets:
        first_key = next(iter(question_sets))
        default_question_set = json.dumps(question_sets[first_key], indent=2)
    
    template_data = {
        'question_sets': question_sets,
        'question_sets_json': question_sets_json,
        'default_question_set': default_question_set,
        'css_content': css_content,
        'js_content': js_content,
        'build_timestamp': build_timestamp,
        'build_timestamp_unix': build_timestamp_unix,
        'version': version
    }
    
    # 6. Render template and write output
    print(f"🔧 Rendering template with {len(question_sets)} question sets...")
    rendered_html = template.render(**template_data)
    
    print(f"💾 Writing packed file to: {output_file}")
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write(rendered_html)
    
    print("✨ Project packed successfully!")
    print("🎯 You can now open index.html directly in any browser")

if __name__ == "__main__":
    pack_project()