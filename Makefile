# ---------- CONFIG ----------
DIFF_DIR = diff-macro
JSON_DIR = json-macro
MAP_DIR  = to-map

# ---------- RUN ----------
run-diff:
	cd $(DIFF_DIR) && cargo run -p app

run-json:
	cd $(JSON_DIR) && cargo run -p app

run-map:
	cd $(MAP_DIR) && cargo run -p app

run-all:
	cd $(DIFF_DIR) && cargo run -p app
	cd $(JSON_DIR) && cargo run -p app
	cd $(MAP_DIR) && cargo run -p app

# ---------- BUILD ----------
build-diff:
	cd $(DIFF_DIR) && cargo build

build-json:
	cd $(JSON_DIR) && cargo build

build-map:
	cd $(MAP_DIR) && cargo build

build-all:
	cd $(DIFF_DIR) && cargo build
	cd $(JSON_DIR) && cargo build
	cd $(MAP_DIR) && cargo build

# ---------- CHECK ----------
check-all:
	cd $(DIFF_DIR) && cargo check
	cd $(JSON_DIR) && cargo check
	cd $(MAP_DIR) && cargo check

# ---------- CLEAN ----------
clean-all:
	cd $(DIFF_DIR) && cargo clean
	cd $(JSON_DIR) && cargo clean
	cd $(MAP_DIR) && cargo clean

# ---------- HELP ----------
help:
	@echo "Available commands:"
	@echo "  make run-diff   - Run diff-macro app"
	@echo "  make run-json   - Run json-macro app"
	@echo "  make run-map    - Run to-map app"
	@echo "  make run-all    - Run all apps"
	@echo ""
	@echo "  make build-all  - Build all projects"
	@echo "  make check-all  - Check all projects"
	@echo "  make clean-all  - Clean all targets"