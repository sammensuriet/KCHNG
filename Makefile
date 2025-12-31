.PHONY: help install build test lint clean dev contract-build contract-test

help: ## Show this help message
	@echo "KCHNG Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  make install      Install all dependencies"
	@echo ""
	@echo "Building:"
	@echo "  make build        Build all packages"
	@echo "  make contract-build  Build Soroban contract"
	@echo ""
	@echo "Testing:"
	@echo "  make test         Run all tests"
	@echo "  make contract-test    Run contract tests"
	@echo ""
	@echo "Development:"
	@echo "  make dev          Start frontend dev server"
	@echo ""
	@echo "Utilities:"
	@echo "  make lint         Run linter"
	@echo "  make clean        Clean build artifacts"

install: ## Install dependencies
	pnpm install

build: ## Build all packages
	pnpm build

contract-build: ## Build Soroban contract
	cd packages/contracts && cargo build --release --target wasm32-unknown-unknown

contract-test: ## Run contract tests
	cd packages/contracts && cargo test

test: ## Run all tests
	pnpm test

lint: ## Run linter
	pnpm lint

dev: ## Start development server
	pnpm dev

clean: ## Clean build artifacts
	pnpm clean
	cd packages/contracts && cargo clean
