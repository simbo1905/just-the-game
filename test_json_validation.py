#!/usr/bin/env python3
"""
Fast unit test to validate all question JSON files against schema
"""

import json
import sys
from pathlib import Path
import jsonschema

def main():
    """Validate all question JSON files against schema"""
    project_root = Path(__file__).parent
    data_dir = project_root / 'data'
    schema_path = data_dir / 'schema.json'
    
    # Load schema
    try:
        with open(schema_path, 'r') as f:
            schema = json.load(f)
        print(f"✅ Schema loaded: {schema_path}")
    except Exception as e:
        print(f"❌ Failed to load schema: {e}")
        return False
    
    # Find all question JSON files
    question_files = list(data_dir.glob('questions*.json'))
    if not question_files:
        print("❌ No question files found")
        return False
    
    print(f"📋 Found {len(question_files)} question files to validate")
    
    all_valid = True
    for file_path in question_files:
        try:
            with open(file_path, 'r') as f:
                data = json.load(f)
            
            # Validate against schema
            jsonschema.validate(data, schema)
            print(f"✅ Valid: {file_path.name}")
            
        except json.JSONDecodeError as e:
            print(f"❌ JSON error in {file_path.name}: {e}")
            all_valid = False
        except jsonschema.ValidationError as e:
            print(f"❌ Schema validation error in {file_path.name}: {e.message}")
            all_valid = False
        except Exception as e:
            print(f"❌ Error validating {file_path.name}: {e}")
            all_valid = False
    
    if all_valid:
        print(f"🎉 All {len(question_files)} question files are valid")
        return True
    else:
        print("❌ Some question files failed validation")
        return False

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
