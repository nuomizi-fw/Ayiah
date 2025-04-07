# Ayiah Monorepo Justfile
# This file contains common commands for the project, use just <command> to run them

# Set shell for non-Windows OSs:
set shell := ["sh", "-c"]

# Set shell for Windows OSs:
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

_list:
    @just --list

# Set default command list
default:
    @just --list

# Variable definitions
root_dir := justfile_directory()

# ===== Project-wide commands =====

# Start development environment
dev: dev-server
    @echo "🚀 All services started"

# Build all projects
build: build-server
    @echo "✅ All projects built successfully"

# Run all code checks
check: check-format check-lint
    @echo "✅ All checks passed"

# Format all code
format:
    @echo "🧹 Formatting all code..."
    cargo fmt

# Run all code checks and fix issues
fix: format
    @echo "🔧 Fixing code issues..."
    cargo clippy --fix --allow-dirty

# Clean all build artifacts
clean:
    @echo "🧹 Cleaning all build artifacts..."
    cargo clean

# ===== Backend commands =====

# Start backend development server
dev-server:
    @echo "🖥️ Starting backend development server..."
    cargo watch -x run

# Build backend project
build-server:
    @echo "🏗️ Building backend project..."
    cargo build --release

# Run backend tests
test-server:
    @echo "🧪 Running backend tests..."
    cargo test

# ===== Code quality commands =====

# Check code formatting
check-format:
    @echo "🔍 Checking code formatting..."
    cargo fmt --check

# Check code issues
check-lint:
    @echo "🔍 Checking code issues..."
    cargo clippy -- -D warnings

# ===== Docker commands =====

# Build Docker images
docker-build:
    @echo "🐳 Building Docker images..."
    docker-compose build

# Start Docker containers
docker-up:
    @echo "🐳 Starting Docker containers..."
    docker-compose up -d

# Stop Docker containers
docker-down:
    @echo "🐳 Stopping Docker containers..."
    docker-compose down
