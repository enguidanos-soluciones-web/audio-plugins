## Take My Headphones (Crossfeed)

![screenshot](../../docs/take_my_headphones.png)

### macOS: allow unsigned plugin

Take My Headphones is not signed with an Apple Developer ID. macOS Gatekeeper will block it on first launch.

**Option A — Finder:** Right-click the `.clap` bundle → Open → Open anyway.

**Option B — Terminal:**

```bash
xattr -dr com.apple.quarantine /path/to/take-my-headphones.clap
```

This is a one-time step. It does not affect plugin functionality.
