#!/usr/bin/env python3
import shutil
from pathlib import Path

def cleanup_data_dir():
    # locate the 'data' directory relative to this script
    data_path = Path(__file__).parent.parent / 'data'

    if not data_path.exists() or not data_path.is_dir():
        print(f"❗ '{data_path}' does not exist or is not a directory.")
        return

    # iterate over everything inside data/
    for child in data_path.iterdir():
        try:
            if child.is_dir():
                shutil.rmtree(child)
                print(f"🗑️  Removed directory: {child}")
            else:
                child.unlink()
                print(f"🗑️  Removed file:      {child}")
        except Exception as e:
            print(f"⚠️  Failed to remove {child}: {e}")

if __name__ == '__main__':
    cleanup_data_dir()
