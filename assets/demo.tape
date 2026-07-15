# Demo recording for herdr-reviewr. Regenerate with:
#   cargo build --release && vhs assets/demo.tape
# Self-contained: builds a throwaway git repo with an agent-style change, then drives the release
# binary beside a small fake agent pane through the review loop. Needs `git`, `tmux`, and `vhs`.

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
Type "bash /tmp/herdr-reviewr-demo/.git/reviewr-demo/demo-session $BIN" Enter
Show

# --- The review loop ---
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
Sleep 1200ms

# Send the review. reviewr adds it to the adjacent agent input and hands focus back.
Type "s"
Sleep 2800ms

# Leave tmux and clean up the demo session.
Ctrl+B
Type "d"
