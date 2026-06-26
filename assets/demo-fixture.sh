#!/usr/bin/env bash
# Build a throwaway repo for the demo recording (assets/demo.tape): a committed baseline plus an
# uncommitted edit + a new file, so the Changes tab has a clear diff to review. Kept out of the
# tape itself because vhs's lexer can't carry the quoting.
set -euo pipefail

D="${1:-/tmp/herdr-reviewr-demo}"
rm -rf "$D"
mkdir -p "$D"
cd "$D"
git init -q
git config user.email demo@example.com
git config user.name demo

cat > parser.py <<'EOF'
def parse(line):
    parts = line.split(",")
    return {"name": parts[0], "value": parts[1]}


def total(rows):
    return sum(int(r["value"]) for r in rows)
EOF
git add -A
git commit -qm baseline

# The agent-style edit under review: input validation + a guard in total().
cat > parser.py <<'EOF'
def parse(line):
    parts = line.split(",")
    if len(parts) < 2:
        raise ValueError(f"bad row: {line!r}")
    return {"name": parts[0].strip(), "value": parts[1].strip()}


def total(rows):
    return sum(int(r["value"]) for r in rows if r["value"])
EOF

cat > utils.py <<'EOF'
def clamp(n, lo, hi):
    return max(lo, min(n, hi))
EOF
