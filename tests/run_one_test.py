#!/usr/bin/env python3
"""
Real test runner that runs pack_project.py and executes individual tests
Passes build timestamp to tests that support it
"""

import sys
import os
import subprocess
import json
from pathlib import Path

def run_pack_project():
    """Run pack_project.py and extract build timestamp"""
    project_root = Path(__file__).parent.parent
    pack_script = project_root / "pack_project.py"
    
    try:
        result = subprocess.run(
            ["python3", str(pack_script)],
            cwd=project_root,
            capture_output=True,
            text=True,
            check=True
        )
        
        # Extract timestamp from output
        lines = result.stdout.split('\n')
        build_timestamp = None
        for line in lines:
            if "Build timestamp:" in line:
                build_timestamp = line.split("Build timestamp:")[1].strip()
                break
        
        return build_timestamp
        
    except subprocess.CalledProcessError as e:
        print(f"❌ pack_project.py failed: {e}")
        print(f"stderr: {e.stderr}")
        return None

def run_test(test_path, build_timestamp=None):
    """Run individual test, optionally passing build timestamp"""
    project_root = Path(__file__).parent.parent
    
    # Construct command
    cmd = ["python3", test_path]
    
    # Add build timestamp if available and test might use it
    env = os.environ.copy()
    if build_timestamp:
        env["BUILD_TIMESTAMP"] = build_timestamp
    
    try:
        result = subprocess.run(
            cmd,
            cwd=project_root,
            env=env,
            check=True
        )
        return True
        
    except subprocess.CalledProcessError:
        return False

def main():
    if len(sys.argv) != 2:
        print("Usage: run_one_test.py <test_file>")
        sys.exit(1)
    
    test_file = sys.argv[1]
    
    # Run pack_project.py first
    build_timestamp = run_pack_project()
    if build_timestamp is None:
        print("⚠️  pack_project.py failed, continuing without timestamp")
    
    # Run the test
    success = run_test(test_file, build_timestamp)
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
