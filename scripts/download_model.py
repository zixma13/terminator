"""Download Gemma 4 model weights for local inference."""
from huggingface_hub import snapshot_download
import os, sys

MODELS = {
    "E2B":     ("google/gemma-4-E2B-it",     "gemma-4-E2B-it",     "~5 GB"),
    "E4B":     ("google/gemma-4-E4B-it",     "gemma-4-E4B-it",     "~9 GB"),
    "26B-A4B": ("google/gemma-4-26B-A4B-it", "gemma-4-26B-A4B-it", "~16 GB"),
    "31B":     ("google/gemma-4-31B-it",     "gemma-4-31B-it",     "~20 GB"),
}

CACHE_DIR = os.path.join(os.path.dirname(__file__), "..", "models")

def main():
    # Accept model name as argument, default to E2B
    name = sys.argv[1].upper() if len(sys.argv) > 1 else None

    if name and name in MODELS:
        targets = {name: MODELS[name]}
    elif name:
        print(f"Unknown model: {name}")
        print(f"Available: {', '.join(MODELS.keys())}")
        sys.exit(1)
    else:
        print("Available Gemma 4 models:")
        for k, (_, _, size) in MODELS.items():
            print(f"  {k:8s} — {size}")
        print()
        choice = input("Which model to download? [E2B]: ").strip().upper() or "E2B"
        if choice == "ALL":
            targets = MODELS
        elif choice in MODELS:
            targets = {choice: MODELS[choice]}
        else:
            print(f"Unknown model: {choice}")
            sys.exit(1)

    for name, (repo_id, dirname, size) in targets.items():
        dest = os.path.join(CACHE_DIR, dirname)
        if os.path.isdir(dest):
            print(f"✓ {name} already downloaded at {dest}")
            continue
        print(f"Downloading {name} ({size}) to {dest}...")
        snapshot_download(
            repo_id=repo_id,
            local_dir=dest,
            ignore_patterns=["*.md", "*.txt"],
        )
        print(f"✓ {name} ready.")

if __name__ == "__main__":
    main()
