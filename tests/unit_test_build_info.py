#!/usr/bin/env python3
"""
Simple test to verify build info is embedded correctly
"""

import re
from pathlib import Path

def test_build_info():
    """Test that build info is present in generated HTML"""
    html_file = Path(__file__).parent / "index.html"
    
    if not html_file.exists():
        print("❌ index.html not found. Run 'python pack_project.py' first.")
        return False
        
    with open(html_file, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Check for BUILD_INFO
    if 'window.BUILD_INFO' not in content:
        print("❌ BUILD_INFO not found in HTML")
        return False
    
    # Extract timestamp
    timestamp_match = re.search(r'timestamp: "([^"]+)"', content)
    if timestamp_match:
        timestamp = timestamp_match.group(1)
        print(f"✅ Build timestamp found: {timestamp}")
    else:
        print("❌ Build timestamp not found")
        return False
    
    # Extract version
    version_match = re.search(r'version: "([^"]+)"', content)
    if version_match:
        version = version_match.group(1)
        print(f"✅ Build version found: {version}")
    else:
        print("❌ Build version not found")
        return False
    
    # Check for build info footer
    if 'class="build-info"' in content:
        print("✅ Build info footer found")
    else:
        print("❌ Build info footer not found")
        return False
    
    print("✅ All build info checks passed")
    return True

if __name__ == "__main__":
    test_build_info()