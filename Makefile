# COnfigure input/output directories
WASM_BINDGEN = wasm-bindgen
WASM_BINDGEN_FLAGS = --target web --no-typescript


WORKSPACE_DIR = ${CURDIR}/src

# The folder containing data for the game
DATA_DIR = ${CURDIR}/data
OUTPUT_FOLDER = ${CURDIR}/bin
EXECUTABLE_OUTPUT_FILE = $(OUTPUT_FOLDER)/gametoy
WEB_OUTPUT_FOLDER = $(OUTPUT_FOLDER)/web


# If RELEASE=1, add --release to the build flags
RELEASE ?= 1
ifeq ($(RELEASE), 1)
	BUILD_FLAGS += --release
    WASM_ARTIFACT_DIR = $(WORKSPACE_DIR)/target/wasm32-unknown-unknown/release/
	ENGINE_ARTIFACT_PATH = $(WORKSPACE_DIR)/target/release/desktop
else
	BUILD_FLAGS += 
    WASM_ARTIFACT_DIR = $(WORKSPACE_DIR)/target/wasm32-unknown-unknown/debug/
	ENGINE_ARTIFACT_PATH = $(WORKSPACE_DIR)/target/debug/desktop
endif

.DEFAULT_GOAL = game
.PHONY: $(WASM_ARTIFACT_DIR)webpage.wasm




# Build the wasm
$(WASM_ARTIFACT_DIR)webpage.wasm:
	cd $(WORKSPACE_DIR); cargo build --package webpage --target wasm32-unknown-unknown $(BUILD_FLAGS)

# Create JS Bindings
$(WEB_OUTPUT_FOLDER)/webpage.js $(SERVE_FOLDER)/webpage_bg.wasm: $(WASM_ARTIFACT_DIR)webpage.wasm
	rm -rf $(WEB_OUTPUT_FOLDER)
	cd $(WORKSPACE_DIR); wasm-bindgen $< $(WASM_BINDGEN_FLAGS) --out-dir $(WEB_OUTPUT_FOLDER)



web: $(WEB_OUTPUT_FOLDER)/webpage.js
	cp -r $(DATA_DIR)/serve/* $(WEB_OUTPUT_FOLDER)


engine:
	cd $(WORKSPACE_DIR); cargo build $(BUILD_FLAGS) --bin desktop
	mkdir -p $(OUTPUT_FOLDER)
	cp $(ENGINE_ARTIFACT_PATH) $(EXECUTABLE_OUTPUT_FILE)

game: web engine


play:
ifndef DEMO
	$(error You need to specify a demo. Eg: "make play DEMO=clock". Available demos are: $(shell ls ./demos))
else
	$(EXECUTABLE_OUTPUT_FILE) ./demos/$(DEMO)
endif


check: 
	cd $(WORKSPACE_DIR); cargo check

fmt:
	cd $(WORKSPACE_DIR); cargo fmt

clippy:
	cd $(WORKSPACE_DIR); cargo clippy

test:
	cd $(WORKSPACE_DIR); cargo test
