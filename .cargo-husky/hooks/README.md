# Pre-commit Hooks

This directory contains Git pre-commit hooks for TermiEmu.

## What are Pre-commit Hooks?

Pre-commit hooks automatically run checks before you commit code. They help catch issues early:
- Code formatting problems (rustfmt)
- Linting warnings (clippy)
- Common mistakes

## Setup (Optional)

Pre-commit hooks are **optional** but recommended for active development.

### Enable Hooks

1. Edit `Cargo.toml` and uncomment the cargo-husky line in `[dev-dependencies]`:
   ```toml
   cargo-husky = { version = "1", default-features = false, features = ["user-hooks"] }
   ```

2. Build the project (this installs the hooks):
   ```bash
   cargo build
   ```

3. Verify hooks are installed:
   ```bash
   ls -la .git/hooks/pre-commit
   ```

### What Gets Checked

Before each commit, the hook will:
1. ✅ Check code formatting with `cargo fmt --check`
2. ✅ Run clippy with `-D warnings`

If any check fails, the commit is blocked until you fix the issues.

## Bypassing Hooks

Sometimes you need to commit work-in-progress code. You can bypass hooks with:
```bash
git commit --no-verify -m "WIP: message"
```

⚠️ **Warning:** Don't bypass hooks for final commits. CI will catch the same issues.

## Troubleshooting

### Hooks not running
- Ensure cargo-husky is uncommented in Cargo.toml
- Run `cargo build` to install hooks
- Check that `.git/hooks/pre-commit` exists and is executable

### "Permission denied" error
```bash
chmod +x .cargo-husky/hooks/pre-commit
```

### Hooks too slow
If checks take too long, consider:
- Using `cargo check` instead of full builds
- Running checks only on changed files
- Customizing the pre-commit script

## Customization

You can edit `pre-commit` to add or remove checks. The script is a standard shell script.

Example additions:
```bash
# Run tests
cargo test

# Check for TODO comments
git diff --cached | grep -i "TODO" && echo "Found TODO comments"
```

## CI Enforcement

Remember: Even if you don't use pre-commit hooks, CI will run the same checks on pull requests. Hooks just catch issues earlier in your local workflow.
