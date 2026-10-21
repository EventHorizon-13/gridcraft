#!/usr/bin/env python3
"""Fetch pinned, hash-verified Apache-2.0 font inputs outside the source checkout."""
import hashlib
import json
import pathlib
import sys
import urllib.request

if len(sys.argv) != 2:
    raise SystemExit("usage: fetch-chinese-font.py EXTERNAL_CRAFT_FONTS_DIR")
root = pathlib.Path(sys.argv[1]).resolve()
checkout = pathlib.Path(__file__).resolve().parents[2]
if root == checkout or checkout in root.parents:
    raise SystemExit("font build inputs must be outside the source checkout")
dest = root / "fonts/droid-sans-fallback"
dest.mkdir(parents=True, exist_ok=True)
base = "https://raw.githubusercontent.com/aosp-mirror/platform_frameworks_base/1aed5ea2dcf7cb0bf35635f378b2326461ba0df0/data/fonts/"
assets = [
    ("DroidSansFallbackFull.ttf", "97320619b0f1ce4a7cda74cf064a0ce44ef3091287bc42755597e177cccf94c2"),
    ("NOTICE", "38751245389e1e23f73e6f5384b5cbe7fa972cc4410c5adc9c04b082a0b9561a"),
]
manifest = []
for name, expected in assets:
    target = dest / name
    data = target.read_bytes() if target.exists() else urllib.request.urlopen(base + name, timeout=90).read()
    if hashlib.sha256(data).hexdigest() != expected:
        raise SystemExit("SHA-256 mismatch: " + name)
    target.write_bytes(data)
    manifest.append({"name": name, "source": base + name, "sha256": expected})
(dest / "font-source.json").write_text(json.dumps(manifest, indent=2) + "\n")
print("Verified external AOSP Chinese font and original Apache-2.0 notice.")
