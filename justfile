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
server_dir := root_dir + "/apps/server"
web_dir := root_dir + "/apps/web"
docs_dir := root_dir + "/apps/docs"

# ===== Project-wide commands =====

# Install all dependencies
install:
    @echo "📦 Installing all dependencies..."
    bun install
    cd {{server_dir}} ; cargo fetch

# Start development environment (frontend, backend, and docs site)
dev: dev-server dev-web dev-docs
    @echo "🚀 All services started"

# Build all projects
build: build-server build-web build-docs
    @echo "✅ All projects built successfully"

# Run all code checks
check: check-format check-lint check-types
    @echo "✅ All checks passed"

# Format all code
format:
    @echo "🧹 Formatting all code..."
    bun format
    cd {{server_dir}} ; cargo fmt

# Run all code checks and fix issues
fix: format
    @echo "🔧 Fixing code issues..."
    bun lint --fix
    cd {{server_dir}} ; cargo clippy --fix --allow-dirty

# Clean all build artifacts
clean:
    @echo "🧹 Cleaning all build artifacts..."
    rm -rf node_modules
    find . -name "node_modules" -type d -prune -exec rm -rf '{}' +
    cd {{server_dir}} ; cargo clean

# ===== Frontend commands =====

# Start frontend development server
dev-web:
    @echo "🌐 Starting frontend development server..."
    cd {{web_dir}} ; bun dev

# Build frontend project
build-web:
    @echo "🏗️ Building frontend project..."
    cd {{web_dir}} ; bun build

# ===== Documentation commands =====

# Start documentation site development server
dev-docs:
    @echo "📚 Starting documentation site development server..."
    cd {{docs_dir}} ; bun dev

# Build documentation site
build-docs:
    @echo "📚 Building documentation site..."
    cd {{docs_dir}} ; bun build

# ===== Backend commands =====

# Start backend development server
dev-server:
    @echo "🖥️ Starting backend development server..."
    cd {{server_dir}} ; cargo watch -x run

# Build backend project
build-server:
    @echo "🏗️ Building backend project..."
    cd {{server_dir}} ; cargo build --release

# Run backend tests
test-server:
    @echo "🧪 Running backend tests..."
    cd {{server_dir}} ; cargo test

# ===== Code quality commands =====

# Check code formatting
check-format:
    @echo "🔍 Checking code formatting..."
    bun biome:check
    cd {{server_dir}} ; cargo fmt --check

# Check code issues
check-lint:
    @echo "🔍 Checking code issues..."
    bun biome:lint
    cd {{server_dir}} ; cargo clippy -- -D warnings

# Check types
check-types:
    @echo "🔍 Checking types..."
    bun check-types

# ===== Database commands =====

# Run database migrations
db-migrate:
    @echo "🗃️ Running database migrations..."
    cd {{server_dir}} ; cargo run --bin migration

# Reset database (dangerous operation)
db-reset:
    @echo "⚠️ Resetting database (this will delete all data)..."
    @echo "Confirm operation? [y/N]"
    @read -r response; \
    if [ "$$response" = "y" ] || [ "$$response" = "Y" ]; then \
        cd {{server_dir}} ; cargo run --bin reset_db; \
    else \
        echo "Operation cancelled"; \
    fi

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
