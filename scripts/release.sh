#!/bin/bash
set -e

# Release automation script for Hanzo Rust SDK

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# Function to check if we're on main branch
check_branch() {
    BRANCH=$(git branch --show-current)
    if [[ "$BRANCH" != "main" ]]; then
        print_error "Releases must be made from the main branch. Current branch: $BRANCH"
        exit 1
    fi
}

# Function to check if working directory is clean
check_clean() {
    if [[ -n $(git status -s) ]]; then
        print_error "Working directory is not clean. Please commit or stash changes."
        exit 1
    fi
}

# Function to update version in Cargo.toml
update_version() {
    local crate=$1
    local new_version=$2
    local cargo_file="crates/$crate/Cargo.toml"
    
    print_status "Updating $crate to version $new_version"
    
    # Update version
    sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" "$cargo_file"
    rm "${cargo_file}.bak"
    
    # Update dependencies if needed
    if [[ "$crate" == "hanzo-kbs" ]]; then
        # Update hanzo-pqc dependency
        sed -i.bak "s/hanzo-pqc = { version = \"[^\"]*\"/hanzo-pqc = { version = \"$new_version\"/" "$cargo_file"
        # Update hanzo-message-primitives dependency  
        sed -i.bak "s/hanzo-message-primitives = { version = \"[^\"]*\"/hanzo-message-primitives = { version = \"$new_version\"/" "$cargo_file"
        rm "${cargo_file}.bak"
    elif [[ "$crate" == "hanzo-pqc" ]]; then
        # Update hanzo-message-primitives dependency if present
        sed -i.bak "s/hanzo-message-primitives = { version = \"[^\"]*\"/hanzo-message-primitives = { version = \"$new_version\"/" "$cargo_file" 2>/dev/null || true
        rm -f "${cargo_file}.bak"
    fi
}

# Function to run tests
run_tests() {
    print_status "Running tests..."
    cargo test --all --all-features
    cargo clippy --all --all-features -- -D warnings
    cargo fmt --all -- --check
}

# Function to build documentation
build_docs() {
    print_status "Building documentation..."
    cargo doc --all --no-deps --all-features
}

# Function to create git tag
create_tag() {
    local version=$1
    local tag="v$version"
    
    print_status "Creating tag $tag"
    git add -A
    git commit -m "chore(release): prepare for $version"
    git tag -a "$tag" -m "Release version $version"
}

# Function to update changelog
update_changelog() {
    local version=$1
    print_status "Updating changelog for version $version"
    
    # Generate changelog using git-cliff if available
    if command -v git-cliff &> /dev/null; then
        git-cliff --tag "v$version" --output CHANGELOG.md
        git add CHANGELOG.md
    else
        print_warning "git-cliff not found. Please update CHANGELOG.md manually."
    fi
}

# Main release function
release() {
    local version=$1
    local dry_run=${2:-false}
    
    if [[ -z "$version" ]]; then
        print_error "Version not specified"
        echo "Usage: $0 <version> [--dry-run]"
        exit 1
    fi
    
    print_status "Starting release process for version $version"
    
    # Pre-flight checks
    check_branch
    check_clean
    
    # Pull latest changes
    print_status "Pulling latest changes..."
    git pull origin main
    
    # Update versions
    update_version "hanzo-message-primitives" "$version"
    update_version "hanzo-pqc" "$version"
    update_version "hanzo-kbs" "$version"
    
    # Update workspace version
    sed -i.bak "s/^version = \".*\"/version = \"$version\"/" Cargo.toml
    rm Cargo.toml.bak
    
    # Update lock file
    print_status "Updating Cargo.lock..."
    cargo update
    
    # Run tests
    run_tests
    
    # Build docs
    build_docs
    
    # Update changelog
    update_changelog "$version"
    
    if [[ "$dry_run" == "true" ]]; then
        print_warning "Dry run mode - not creating tag or pushing"
        git reset --hard HEAD
        exit 0
    fi
    
    # Create tag and push
    create_tag "$version"
    
    print_status "Pushing changes and tag..."
    git push origin main
    git push origin "v$version"
    
    print_status "Release $version completed successfully!"
    print_status "GitHub Actions will now handle the crates.io publication."
}

# Parse arguments
VERSION=$1
DRY_RUN=false

if [[ "$2" == "--dry-run" ]]; then
    DRY_RUN=true
fi

# Run release
release "$VERSION" "$DRY_RUN"