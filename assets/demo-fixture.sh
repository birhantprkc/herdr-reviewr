#!/usr/bin/env bash
# Build a throwaway repo for the demo recording (assets/demo.tape): a committed baseline plus an
# uncommitted edit + a new file, so the Changes tab has a clear diff to review. It also writes a
# tiny `herdr` stand-in that adds exported comments to the demo's adjacent agent input. Kept
# out of the tape itself because vhs's lexer can't carry the quoting.
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

TOOLS="$D/.git/reviewr-demo"
mkdir -p "$TOOLS"

cat > "$TOOLS/mock-herdr" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

case "${1:-} ${2:-}" in
  "agent list")
    printf '%s\n' '{"result":{"agents":[{"agent":"codex","agent_status":"idle","pane_id":"demo:p1","tab_id":"demo:t1","workspace_id":"demo"}]}}'
    ;;
  "agent send")
    TOOLS="$(cd "$(dirname "$0")" && pwd)"
    printf '%s' "$4" > "$TOOLS/pending-input"
    ;;
  "agent focus")
    tmux select-pane -t reviewr-demo:0.0
    ;;
  *)
    printf 'unsupported demo command: %s\n' "$*" >&2
    exit 1
    ;;
esac
EOF
chmod +x "$TOOLS/mock-herdr"

cat > "$TOOLS/agent-pane" <<'EOF'
#!/usr/bin/env bash
printf '\033c\033[1;36mcodex\033[0m\n\nReview feedback lands here.\n\n'
TOOLS="$(cd "$(dirname "$0")" && pwd)"
while [[ ! -f "$TOOLS/pending-input" ]]; do
  sleep 0.1
done
printf '\033c\033[1;36mcodex\033[0m\n\nPending input — edit, then press Enter to submit.\n\n'
cat "$TOOLS/pending-input"
printf '█'
while :; do
  sleep 1
done
EOF
chmod +x "$TOOLS/agent-pane"

cat > "$TOOLS/demo-session" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

BIN="$1"
TOOLS="$(cd "$(dirname "$0")" && pwd)"
D="$(cd "$TOOLS/../.." && pwd)"
tmux kill-session -t reviewr-demo 2>/dev/null || true
tmux new-session -d -s reviewr-demo "$TOOLS/agent-pane"
tmux split-window -h -p 62 -t reviewr-demo:0 \
  "cd '$D' && HERDR_BIN_PATH='$TOOLS/mock-herdr' HERDR_TAB_ID=demo:t1 HERDR_WORKSPACE_ID=demo HERDR_PANE_ID=demo:p2 '$BIN'"
tmux set-option -t reviewr-demo status off
tmux select-pane -t reviewr-demo:0.1
tmux attach-session -t reviewr-demo
tmux kill-session -t reviewr-demo 2>/dev/null || true
EOF
chmod +x "$TOOLS/demo-session"
