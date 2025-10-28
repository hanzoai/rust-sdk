#!/bin/bash
set -e

# Pre-release checks for Hanzo Rust SDK

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_check() {
    echo -e "${GREEN}✓${NC} $1"
}

print_fail() {
    echo -e "${RED}✗${NC} $1"
    exit 1
}

print_info() {
    echo -e "${YELLOW}ℹ${NC} $1"
}

echo "Running pre-release checks..."
echo "=============================="

# Check Rust toolchain
print_info "Checking Rust toolchain..."
if ! rustc --version &> /dev/null; then
    print_fail "Rust is not installed"
fi
print_check "Rust $(rustc --version | cut -d' ' -f2) installed"

# Check required tools
REQUIRED_TOOLS=("cargo" "git" "sed")
for tool in "${REQUIRED_TOOLS[@]}"; do
    if ! command -v $tool &> /dev/null; then
        print_fail "$tool is required but not installed"
    fi
done
print_check "All required tools installed"

# Check cargo plugins
RECOMMENDED_TOOLS=("cargo-publish-all" "cargo-outdated" "cargo-audit" "git-cliff")
for tool in "${RECOMMENDED_TOOLS[@]}"; do
    if ! command -v $tool &> /dev/null; then
        print_info "$tool is recommended but not installed"
    fi
done

# Check git status
if [[ -n $(git status -s) ]]; then
    print_fail "Working directory has uncommitted changes"
fi
print_check "Working directory is clean"

# Check branch
BRANCH=$(git branch --show-current)
if [[ "$BRANCH" != "main" ]]; then
    print_info "Currently on branch: $BRANCH (releases should be from main)"
fi

# Check tests
print_info "Running tests..."
if ! cargo test --all --quiet; then
    print_fail "Tests failed"
fi
print_check "All tests passed"

# Check formatting
print_info "Checking formatting..."
if ! cargo fmt --all -- --check; then
    print_fail "Code is not properly formatted. Run 'cargo fmt --all'"
fi
print_check "Code formatting is correct"

# Check clippy
print_info "Running clippy..."
if ! cargo clippy --all --all-features -- -D warnings; then
    print_fail "Clippy found issues"
fi
print_check "Clippy checks passed"

# Check documentation
print_info "Checking documentation..."
if ! RUSTDOCFLAGS="-D warnings" cargo doc --all --no-deps --all-features; then
    print_fail "Documentation has errors"
fi
print_check "Documentation builds successfully"

# Check package metadata
for crate in "hanzo-message-primitives" "hanzo-pqc" "hanzo-kbs"; do
    print_info "Checking $crate package..."
    cd "crates/$crate"
    if ! cargo publish --dry-run &> /dev/null; then
        print_fail "$crate package verification failed"
    fi
    cd ../..
done
print_check "All packages are ready for publication"

# Check dependencies
print_info "Checking for outdated dependencies..."
if command -v cargo-outdated &> /dev/null; then
    cargo outdated --root-deps-only || true
fi

# Security audit
print_info "Running security audit..."
if command -v cargo-audit &> /dev/null; then
    if ! cargo audit; then
        print_info "Security vulnerabilities found - review before release"
    fi
fi

echo ""
echo "=============================="
echo -e "${GREEN}All pre-release checks passed!${NC}"
echo ""
echo "Next steps:"
echo "1. Review and update CHANGELOG.md"
echo "2. Run: ./scripts/release.sh <version>"
echo "3. Monitor GitHub Actions for the release workflow"