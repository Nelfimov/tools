HOOKS_DIR := .config/.git-hooks

.PHONY: build test lint

prepare:
	@git config --local core.hooksPath "$(HOOKS_DIR)"
	@chmod +x $(HOOKS_DIR)/* || true
	@echo "✅ Git hooks подключены (core.hooksPath = $(HOOKS_DIR))"

build: test
	cargo build

test:
	cargo test

lint:
	cargo clippy -- -D warnings
