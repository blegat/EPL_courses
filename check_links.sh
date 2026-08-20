#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "$repo_root"

python3 - "$repo_root" <<'PY'
from __future__ import annotations

import re
import subprocess
import sys
import unicodedata
from collections import defaultdict
from pathlib import Path
from urllib.parse import unquote, urlsplit


ROOT = Path(sys.argv[1]).resolve()
IGNORED_DIRS = {".git", ".obsidian", ".venv", "node_modules", "vendor"}
EXTERNAL_SCHEMES = {"http", "https", "mailto", "tel", "data", "ftp"}

# Inline Markdown links and images. Destinations surrounded by angle brackets may
# contain spaces; ordinary destinations end at whitespace or a closing parenthesis.
LINK_RE = re.compile(
    r"(?P<image>!)?\[[^\]]*\]\(\s*(?P<destination><[^>]+>|[^\s)]+)",
    re.MULTILINE,
)
FENCE_RE = re.compile(r"^\s*(```|~~~)")
HEADING_RE = re.compile(r"^\s{0,3}#{1,6}\s+(.+?)\s*#*\s*$")


def markdown_files() -> list[Path]:
    return sorted(
        path.resolve()
        for path in ROOT.rglob("*.md")
        if not any(part in IGNORED_DIRS for part in path.relative_to(ROOT).parts)
        and not is_git_ignored(path)
    )


def without_fenced_code(text: str) -> str:
    output: list[str] = []
    in_fence = False
    marker = ""
    for line in text.splitlines(keepends=True):
        match = FENCE_RE.match(line)
        if match:
            current = match.group(1)
            if not in_fence:
                in_fence = True
                marker = current[0]
            elif current[0] == marker:
                in_fence = False
            output.append("\n")
        elif in_fence:
            output.append("\n")
        else:
            output.append(line)
    return "".join(output)


def github_slug(heading: str) -> str:
    heading = re.sub(r"<[^>]*>", "", heading)
    heading = re.sub(r"[*_~`]", "", heading).strip().lower()
    chars = []
    for char in heading:
        category = unicodedata.category(char)
        if char in {" ", "-"} or category[0] in {"L", "N"}:
            chars.append(char)
    return re.sub(r"\s+", "-", "".join(chars))


def heading_slugs(path: Path) -> set[str]:
    counts: defaultdict[str, int] = defaultdict(int)
    slugs: set[str] = set()
    for line in path.read_text(encoding="utf-8").splitlines():
        match = HEADING_RE.match(line)
        if not match:
            continue
        base = github_slug(match.group(1))
        suffix = counts[base]
        counts[base] += 1
        slugs.add(base if suffix == 0 else f"{base}-{suffix}")
    return slugs


def display(path: Path) -> str:
    try:
        return path.relative_to(ROOT).as_posix()
    except ValueError:
        return str(path)


def is_git_ignored(path: Path) -> bool:
    """Return whether Git intentionally excludes a repository path."""
    try:
        relative = path.relative_to(ROOT)
    except ValueError:
        return False
    result = subprocess.run(
        ["git", "check-ignore", "--quiet", "--", relative.as_posix()],
        cwd=ROOT,
        check=False,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    return result.returncode == 0


files = markdown_files()
file_set = set(files)
edges: set[tuple[Path, Path]] = set()
errors: list[str] = []
checked_links = 0
ignored_links = 0

for source in files:
    text = without_fenced_code(source.read_text(encoding="utf-8"))
    for match in LINK_RE.finditer(text):
        raw = match.group("destination")
        if raw.startswith("<") and raw.endswith(">"):
            raw = raw[1:-1]

        parsed = urlsplit(raw)
        if parsed.scheme.lower() in EXTERNAL_SCHEMES or raw.startswith("//"):
            continue

        checked_links += 1
        line = text.count("\n", 0, match.start()) + 1
        decoded_path = unquote(parsed.path)
        fragment = unquote(parsed.fragment)

        if not decoded_path:
            target = source
        elif decoded_path.startswith("/"):
            target = (ROOT / decoded_path.lstrip("/")).resolve()
        else:
            target = (source.parent / decoded_path).resolve()

        try:
            target.relative_to(ROOT)
        except ValueError:
            errors.append(
                f"{display(source)}:{line}: link escapes the repository: {raw}"
            )
            continue

        if not target.exists():
            if is_git_ignored(target):
                ignored_links += 1
                continue
            errors.append(f"{display(source)}:{line}: missing target: {raw}")
            continue

        if fragment and target.suffix.lower() == ".md":
            wanted = github_slug(fragment)
            if wanted not in heading_slugs(target):
                errors.append(
                    f"{display(source)}:{line}: missing heading #{fragment} "
                    f"in {display(target)}"
                )

        if (
            not match.group("image")
            and source != target
            and source.name.lower() != "readme.md"
            and target.suffix.lower() == ".md"
            and target.name.lower() != "readme.md"
        ):
            edges.add((source, target))

for source, target in sorted(edges, key=lambda edge: (str(edge[0]), str(edge[1]))):
    if target not in file_set:
        continue
    if (target, source) not in edges:
        errors.append(
            f"{display(source)}: missing backlink from {display(target)}"
        )

if errors:
    print(f"Link check failed with {len(errors)} error(s):", file=sys.stderr)
    for error in errors:
        print(f"  - {error}", file=sys.stderr)
    print(
        f"Checked {checked_links} local links in {len(files)} Markdown files.",
        file=sys.stderr,
    )
    raise SystemExit(1)

print(
    f"Link check passed: {checked_links} local links in {len(files)} Markdown "
    f"files; {len(edges)} directed topic links have backlinks; "
    f"{ignored_links} missing ignored targets were skipped."
)
PY
