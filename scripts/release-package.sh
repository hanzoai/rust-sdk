#!/bin/bash
set -e

# Per-package release script for Hanzo Rust SDK monorepo
# Follows the same pattern as Python SDK

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

# Function to show usage
usage() {
    echo "Usage: $0 <package> <version> [--dry-run]"
    echo ""
    echo "Packages:"
    echo "  hanzo-message-primitives"
    echo "  hanzo-pqc"
    echo "  hanzo-kbs"
    echo "  all                      - Release all packages with the same version"
    echo ""
    echo "Version:"
    echo "  Semantic version (e.g., 0.1.0, 0.2.0-beta.1)"
    echo ""
    echo "Options:"
    echo "  --dry-run               - Show what would be done without making changes"
    echo ""
    echo "Examples:"
    echo "  $0 hanzo-kbs 0.1.1"
    echo "  $0 hanzo-pqc 0.2.0-beta.1 --dry-run"
    echo "  $0 all 0.1.0"
    exit 1
}

# Parse arguments
if [[ $# -lt 2 ]]; then
    usage
fi

PACKAGE=$1
VERSION=$2
DRY_RUN=false

if [[ "$3" == "--dry-run" ]]; then
    DRY_RUN=true
fi

# Validate package name
VALID_PACKAGES=("hanzo-message-primitives" "hanzo-pqc" "hanzo-kbs" "all")
if [[ ! " ${VALID_PACKAGES[@]} " =~ " ${PACKAGE} " ]]; then
    print_error "Invalid package: $PACKAGE"
    usage
fi

# Validate version format
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$ ]]; then
    print_error "Invalid version format: $VERSION"
    echo "Version must follow semantic versioning (e.g., 0.1.0, 0.2.0-beta.1)"
    exit 1
fi

# Function to check if we're on main branch
check_branch() {
    BRANCH=$(git branch --show-current)
    if [[ "$BRANCH" != "main" ]]; then
        print_warning "Not on main branch. Current branch: $BRANCH"
        read -p "Continue anyway? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
}

# Function to check if working directory is clean
check_clean() {
    if [[ -n $(git status -s) ]]; then
        print_error "Working directory is not clean. Please commit or stash changes."
        exit 1
    fi
}

# Function to update version in a specific crate
update_crate_version() {
    local crate=$1
    local new_version=$2
    local cargo_file="crates/$crate/Cargo.toml"
    
    if [[ ! -f "$cargo_file" ]]; then
        print_error "Cargo.toml not found: $cargo_file"
        exit 1
    fi
    
    print_status "Updating $crate to version $new_version"
    
    # Update version
    sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" "$cargo_file"
    rm "${cargo_file}.bak"
    
    # Update inter-crate dependencies if needed
    if [[ "$crate" == "hanzo-kbs" ]]; then
        # Update hanzo-pqc dependency version if we're releasing all
        if [[ "$PACKAGE" == "all" ]]; then
            sed -i.bak "s/hanzo-pqc = { version = \"[^\"]*\"/hanzo-pqc = { version = \"$new_version\"/" "$cargo_file"
            sed -i.bak "s/hanzo-message-primitives = { version = \"[^\"]*\"/hanzo-message-primitives = { version = \"$new_version\"/" "$cargo_file"
            rm "${cargo_file}.bak"
        fi
    elif [[ "$crate" == "hanzo-pqc" ]]; then
        # Update hanzo-message-primitives dependency if releasing all
        if [[ "$PACKAGE" == "all" ]]; then
            sed -i.bak "s/hanzo-message-primitives = { version = \"[^\"]*\"/hanzo-message-primitives = { version = \"$new_version\"/" "$cargo_file" 2>/dev/null || true
            rm -f "${cargo_file}.bak"
        fi
    fi
}

# Function to create tag for a package
create_package_tag() {
    local package=$1
    local version=$2
    local tag="$package-v$version"
    
    print_status "Creating tag: $tag"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        print_info "[DRY RUN] Would create tag: $tag"
    else
        git tag -a "$tag" -m "Release $package version $version"
    fi
}

# Function to run tests for a specific crate
test_crate() {
    local crate=$1
    
    print_status "Testing $crate..."
    
    if [[ "$DRY_RUN" == "true" ]]; then
        print_info "[DRY RUN] Would run tests for $crate"
    else
        cargo test -p "$crate" --all-features
    fi
}

# Function to verify package can be published
verify_package() {
    local crate=$1
    
    print_status "Verifying $crate package..."
    
    if [[ "$DRY_RUN" == "true" ]]; then
        print_info "[DRY RUN] Would verify $crate"
    else
        cd "crates/$crate"
        cargo publish --dry-run
        cd ../..
    fi
}

# Main release function
release_package() {
    local package=$1
    local version=$2
    
    print_status "Starting release process for $package version $version"
    
    # Update version
    update_crate_version "$package" "$version"
    
    # Run tests
    test_crate "$package"
    
    # Verify package
    verify_package "$package"
    
    # Update Cargo.lock
    print_status "Updating Cargo.lock..."
    cargo update -p "$package"
    
    # Commit changes
    if [[ "$DRY_RUN" == "true" ]]; then
        print_info "[DRY RUN] Would commit: chore(release): $package v$version"
    else
        git add -A
        git commit -m "chore(release): $package v$version"
    fi
    
    # Create tag
    create_package_tag "$package" "$version"
}

# Main execution
print_status "Hanzo Rust SDK Per-Package Release Tool"
print_status "======================================="

# Pre-flight checks
check_branch
check_clean

# Pull latest changes
print_status "Pulling latest changes..."
if [[ "$DRY_RUN" == "false" ]]; then
    git pull origin main
fi

# Handle release based on package selection
if [[ "$PACKAGE" == "all" ]]; then
    print_status "Releasing all packages with version $VERSION"
    
    # Release in dependency order
    for crate in "hanzo-message-primitives" "hanzo-pqc" "hanzo-kbs"; do
        release_package "$crate" "$VERSION"
    done
    
    # Create a general version tag as well
    if [[ "$DRY_RUN" == "true" ]]; then
        print_info "[DRY RUN] Would create tag: v$VERSION"
    else
        git tag -a "v$VERSION" -m "Release all packages version $VERSION"
    fi
else
    # Release single package
    release_package "$PACKAGE" "$VERSION"
fi

# Push changes and tags
if [[ "$DRY_RUN" == "true" ]]; then
    print_info "[DRY RUN] Would push changes and tags to origin"
    print_info "[DRY RUN] Complete! No actual changes were made."
else
    print_status "Pushing changes and tags..."
    git push origin main
    git push origin --tags
    
    print_status "Release completed successfully!"
    print_status ""
    print_status "GitHub Actions will now:"
    print_status "  1. Run the full CI test suite"
    print_status "  2. Build release artifacts"
    print_status "  3. Publish to crates.io"
    print_status "  4. Create GitHub release"
    print_status ""
    print_status "Monitor the release workflow at:"
    print_status "  https://github.com/hanzoai/rust-sdk/actions"
fi

# Show next steps
echo ""
print_info "Next steps:"
if [[ "$PACKAGE" == "all" ]]; then
    print_info "  - Monitor GitHub Actions for all package publications"
    print_info "  - Verify all packages on crates.io"
else
    print_info "  - Monitor GitHub Actions for $PACKAGE publication"
    print_info "  - Verify package on https://crates.io/crates/$PACKAGE"
fi
print_info "  - Update downstream dependencies if needed"
print_info "  - Announce the release"