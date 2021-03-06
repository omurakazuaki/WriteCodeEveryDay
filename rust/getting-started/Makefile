OUT_DIR=bin

pre-build:
	mkdir -p $(OUT_DIR)

build:
	$(MAKE) pre-build
	rustc -o $(OUT_DIR)/bf src/main.rs
