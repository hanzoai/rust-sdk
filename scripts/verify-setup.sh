#!/bin/bash
set -e

# Verify GitHub Actions setup for publishing

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_check() {
    echo -e "${GREEN}✓${NC} $1"
}

print_fail() {
    echo -e "${RED}✗${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

echo "Verifying Hanzo Rust SDK Publishing Setup"
echo "========================================="
echo ""

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]] || [[ ! -d "crates" ]]; then
    print_fail "Not in rust-sdk root directory"
    exit 1
fi
print_check "In rust-sdk directory"

# Check git remote
REMOTE_URL=$(git remote get-url origin 2>/dev/null || echo "")
if [[ "$REMOTE_URL" == *"hanzoai/rust-sdk"* ]]; then
    print_check "Git remote is set to hanzoai/rust-sdk"
else
    print_warn "Git remote doesn't appear to be hanzoai/rust-sdk: $REMOTE_URL"
fi

# Check GitHub CLI is installed
if command -v gh &> /dev/null; then
    print_check "GitHub CLI (gh) is installed"
    
    # Check if authenticated
    if gh auth status &> /dev/null; then
        print_check "GitHub CLI is authenticated"
        
        # Check repository access
        if gh repo view hanzoai/rust-sdk &> /dev/null; then
            print_check "Can access hanzoai/rust-sdk repository"
            
            # Check if CARGO_REGISTRY_TOKEN is set
            print_info "Checking for CARGO_REGISTRY_TOKEN secret..."
            SECRETS=$(gh secret list --repo hanzoai/rust-sdk 2>/dev/null || echo "")
            if echo "$SECRETS" | grep -q "CARGO_REGISTRY_TOKEN"; then
                print_check "CARGO_REGISTRY_TOKEN is configured in GitHub secrets"
            else
                print_fail "CARGO_REGISTRY_TOKEN not found in GitHub secrets"
                print_info "Add it at: https://github.com/hanzoai/rust-sdk/settings/secrets/actions"
            fi
        else
            print_fail "Cannot access hanzoai/rust-sdk - check permissions"
        fi
    else
        print_warn "GitHub CLI not authenticated. Run: gh auth login"
    fi
else
    print_warn "GitHub CLI not installed. Install from: https://cli.github.com/"
fi

# Check local cargo setup
if command -v cargo &> /dev/null; then
    print_check "Cargo is installed"
    
    # Check if logged in to crates.io locally
    if [[ -f ~/.cargo/credentials.toml ]]; then
        print_check "Cargo credentials file exists"
    else
        print_warn "No local cargo credentials. For manual publishing, run: cargo login"
    fi
else
    print_fail "Cargo not installed"
fi

# Check crate versions
echo ""
print_info "Current crate versions:"
for crate in hanzo-message-primitives hanzo-pqc hanzo-kbs; do
    VERSION=$(grep '^version = ' "crates/$crate/Cargo.toml" | cut -d'"' -f2)
    echo "  - $crate: v$VERSION"
done

# Check workflows exist
echo ""
print_info "Checking GitHub Actions workflows:"
for workflow in ci.yml release.yml pr.yml scheduled.yml version-check.yml tag-release.yml; do
    if [[ -f ".github/workflows/$workflow" ]]; then
        print_check "Workflow exists: $workflow"
    else
        print_fail "Missing workflow: $workflow"
    fi
done

# Check release scripts
echo ""
print_info "Checking release scripts:"
for script in check-release.sh release.sh release-package.sh; do
    if [[ -f "scripts/$script" ]] && [[ -x "scripts/$script" ]]; then
        print_check "Script exists and is executable: $script"
    else
        print_fail "Missing or not executable: scripts/$script"
    fi
done

echo ""
echo "========================================="

# Summary
if gh auth status &> /dev/null && echo "$SECRETS" | grep -q "CARGO_REGISTRY_TOKEN" 2>/dev/null; then
    echo -e "${GREEN}✓ Publishing setup looks good!${NC}"
    echo ""
    echo "Next steps:"
    echo "1. Run: ./scripts/check-release.sh"
    echo "2. Release: ./scripts/release-package.sh <crate> <version>"
    echo "3. Monitor: https://github.com/hanzoai/rust-sdk/actions"
else
    echo -e "${YELLOW}⚠ Some setup steps may be needed${NC}"
    echo ""
    echo "Required actions:"
    if ! command -v gh &> /dev/null; then
        echo "- Install GitHub CLI: https://cli.github.com/"
    elif ! gh auth status &> /dev/null; then
        echo "- Authenticate GitHub CLI: gh auth login"
    fi
    if ! echo "$SECRETS" | grep -q "CARGO_REGISTRY_TOKEN" 2>/dev/null; then
        echo "- Add CARGO_REGISTRY_TOKEN to GitHub secrets"
        echo "  Visit: https://github.com/hanzoai/rust-sdk/settings/secrets/actions"
    fi
fi