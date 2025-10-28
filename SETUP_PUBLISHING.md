# Setting Up Automated Publishing

This guide ensures that GitHub Actions can automatically publish to crates.io when you push tags.

## 1. Get Your Crates.io Token

First, get your crates.io API token:

1. Go to https://crates.io/settings/tokens
2. Create a new token with "publish-update" scope
3. Copy the token (starts with `crates-io_...`)

## 2. Add Token to GitHub Secrets

Add the token to your GitHub repository:

1. Go to https://github.com/hanzoai/rust-sdk/settings/secrets/actions
2. Click "New repository secret"
3. Name: `CARGO_REGISTRY_TOKEN`
4. Value: Your crates.io token
5. Click "Add secret"

## 3. Verify Setup

### Check Token is Set
Go to https://github.com/hanzoai/rust-sdk/settings/secrets/actions and verify you see:
- ✅ `CARGO_REGISTRY_TOKEN`

### Test with Dry Run
```bash
# Test the release process without actually publishing
./scripts/release-package.sh hanzo-message-primitives 0.1.0 --dry-run
```

### Manual Workflow Test
1. Go to https://github.com/hanzoai/rust-sdk/actions/workflows/release.yml
2. Click "Run workflow"
3. Select "dry_run: true"
4. Choose a crate to test
5. Run and verify it completes without errors

## 4. First Real Release

When ready for your first release:

```bash
# 1. Ensure you're on main branch with clean working directory
git checkout main
git pull origin main

# 2. Run pre-release checks
./scripts/check-release.sh

# 3. Release a package (this will push a tag)
./scripts/release-package.sh hanzo-message-primitives 0.1.0

# 4. Monitor the release
# Go to: https://github.com/hanzoai/rust-sdk/actions
# You should see the "Release & Publish" workflow running

# 5. Verify on crates.io
# Check: https://crates.io/crates/hanzo-message-primitives
```

## 5. Troubleshooting

### "unauthorized" Error
- Verify `CARGO_REGISTRY_TOKEN` is set in GitHub secrets
- Ensure the token has "publish-update" scope
- Token might be expired - create a new one

### "already published" Error
- Version already exists on crates.io
- Bump to a new version and try again

### Dependencies Not Found
- Dependencies are published in order with 30s delays
- If a dependency fails, subsequent crates won't publish
- Check earlier steps in the workflow

### Workflow Not Triggering
- Ensure you pushed the tag: `git push origin --tags`
- Tag must match pattern: `hanzo-<crate>-v<version>` or `v<version>`

## 6. Local Publishing (Backup)

If CI fails, you can publish manually:

```bash
# Set your token
export CARGO_REGISTRY_TOKEN=your-token-here

# Publish in order
cd crates/hanzo-message-primitives && cargo publish
sleep 30
cd ../hanzo-pqc && cargo publish  
sleep 30
cd ../hanzo-kbs && cargo publish
```

## 7. Security Notes

- Never commit your `CARGO_REGISTRY_TOKEN`
- Use GitHub secrets for all tokens
- Rotate tokens periodically
- Use scoped tokens with minimal permissions

## 8. Monitoring

After release, monitor:
- GitHub Actions: https://github.com/hanzoai/rust-sdk/actions
- Crates.io: https://crates.io/users/hanzoai
- Docs.rs builds: https://docs.rs/releases/queue

## Need Help?

- Check workflow logs in GitHub Actions
- Review this guide and PUBLISHING.md
- Open an issue if you encounter problems