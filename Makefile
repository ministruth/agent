SHELL = /bin/bash
OUTPUT_DIR = $$(pwd)/bin
ID = `cat config.yml | head -n 1 | cut -d \" -f 2`
NAME = `cat config.yml | head -n 2 | cut -d \" -f 2 | tail -n 1`
STATIC_DIR = /assets/$(ID)
BIN_DIR = /$(NAME)
BUILD_TYPE = debug
TARGET_DIR = $$(pwd)/target/$(BUILD_TYPE)
PLUGIN_SUFFIX =
PLATFORM =
MACHINE =
SUFFIX =

ifeq ($(OS),Windows_NT)
	PLATFORM = windows
	SUFFIX = .exe
	PLUGIN_SUFFIX = .dll
	ifeq ($(PROCESSOR_ARCHITECTURE),AMD64)
		MACHINE = x64
	endif
	ifeq ($(PROCESSOR_ARCHITECTURE),x86)
		MACHINE = x86
	endif
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		PLATFORM = linux
		PLUGIN_SUFFIX = .so
	endif
	ifeq ($(UNAME_S),Darwin)
		PLATFORM = osx
		PLUGIN_SUFFIX = .dylib
	endif
	UNAME_M := $(shell uname -m)
	ifeq ($(UNAME_M),x86_64)
		MACHINE = x64
	endif
	ifneq ($(filter %86,$(UNAME_M)),)
		MACHINE = x86
	endif
	ifeq ($(UNAME_M),arm64)
		MACHINE = arm64
	endif
	ifeq ($(UNAME_M),aarch64)
		MACHINE = arm64
	endif
	ifeq ($(UNAME_M),arm)
		MACHINE = arm
	endif
endif

.PHONY: check output clean help

## check: Check code and style.
check:
	@cargo clippy -- -D clippy::all
	@cargo fmt --all -- --check

## output: Copy build files for production.
output:
	@rm -rf $(OUTPUT_DIR)$(BIN_DIR) && mkdir -p $(OUTPUT_DIR)$(BIN_DIR) && mkdir -p $(OUTPUT_DIR)$(BIN_DIR)/bin
	@cp $(TARGET_DIR)/*$(NAME)$(PLUGIN_SUFFIX) $(OUTPUT_DIR)$(BIN_DIR)
	@cp config.yml $(OUTPUT_DIR)$(BIN_DIR)
	@cp $(TARGET_DIR)/agent$(SUFFIX) $(OUTPUT_DIR)$(BIN_DIR)/bin/agent_$(PLATFORM)_$(MACHINE)$(SUFFIX)

## clean: Clean all build files.
clean:
	@rm -rf data
	@rm -rf $(OUTPUT_DIR)
	@cargo clean

## help: Show this help.
help: Makefile
	@echo Usage: make [command]
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
