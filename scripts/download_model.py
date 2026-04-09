"""Download Gemma 4 E2B model weights for local inference."""
from huggingface_hub import snapshot_download
import os, sys

MODEL_ID = "google/gemma-4-E2B-it"
CACHE_DIR = os.path.join(os.path.dirname(__file__), "..", "models")

def main():
    print(f"Downloading {MODEL_ID} to {CACHE_DIR}...")
    print("This may take a while (~5GB)...")
    snapshot_download(
        repo_id=MODEL_ID,
        local_dir=os.path.join(CACHE_DIR, "gemma-4-E2B-it"),
        ignore_patterns=["*.md", "*.txt"],
    )
    print("Done! Model ready.")

if __name__ == "__main__":
    main()
