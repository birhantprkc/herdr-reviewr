# Demo recording for herdr-reviewr. Regenerate with:
#   cargo build --release && vhs assets/demo.tape
# Self-contained: builds a throwaway git repo with an agent-style change, then drives the
# release binary through the review loop. Needs `git` and `vhs` on PATH.

Output assets/demo.gif

Set Shell "bash"
Set FontSize 13
Set Width 1320
Set Height 820
Set Theme "Catppuccin Mocha"
Set Padding 16
Set TypingSpeed 45ms

# --- Hidden setup: a temp repo with a committed baseline + an uncommitted edit to review ---
Hide
Type "BIN=$PWD/target/release/herdr-reviewr && bash assets/demo-fixture.sh /tmp/herdr-reviewr-demo" Enter
Type "cd /tmp/herdr-reviewr-demo && clear" Enter
Show

# --- The review loop ---
Type "$BIN" Enter
Sleep 2s

# Focus the diff and move into the changed region.
Tab
Sleep 700ms
Type "jjjj"
Sleep 900ms

# Select a line range...
Type "v"
Sleep 400ms
Type "jj"
Sleep 700ms

# ...and comment on it.
Type "c"
Sleep 600ms
Type "guard the empty value here too?"
Sleep 800ms
Enter
Sleep 1500ms

# Browse the whole tree on the All files tab.
Type "2"
Sleep 1200ms
Down
Sleep 500ms
Down
Sleep 900ms

# Back to Changes, open the comments list.
Type "1"
Sleep 700ms
Type "l"
Sleep 1800ms
Escape
Sleep 600ms

Type "q"
Sleep 800ms
