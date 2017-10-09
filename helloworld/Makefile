TARGET = debug
TARGET_DIR = ./target/$(TARGET)

C_SRC_DIR = ./src
C_OBJ_DIR = $(TARGET_DIR)

PD_RESOURCE := ../libpd/pure-data
PD_SRC = $(PD_RESOURCE)/src
CFLAGS = -I$(PD_SRC) -fPIC -arch x86_64 -c
LFLAGS = -dynamic -bundle -undefined dynamic_lookup -L$(TARGET_DIR) -l$(RUST_LIB_NAME)

RUST_LIB_NAME = helloworld
NAME := helloworld

.PHONY: cargo_build
	cargo build

.PHONY: cargo_test
test:
	@DYLD_LIBRARY_PATH=../libpd/libs cargo test

.PHONY: debug
debug:
	$(eval TARGET := debug)
	$(MAKE) $(C_OBJ_DIR)/$(NAME).pd_darwin

$(C_OBJ_DIR)/$(NAME).pd_darwin: $(C_OBJ_DIR)/$(NAME).o $(TARGET_DIR)/lib$(RUST_LIB_NAME).a
	$(CC) $(LFLAGS) -o $@ $^

$(C_OBJ_DIR)/$(NAME).o: $(C_SRC_DIR)/$(NAME).c
	$(MAKE) target
	$(CC) $(CFLAGS) -o $@ $^

$(TARGET_DIR)/lib$(RUST_LIB_NAME).a:
	cargo build

target:
	mkdir target

.PHONY: clean
clean: target
	rm -f target/debug/*.a
	rm -f target/debug/*.o
	rm -f target/release/*.a
	rm -f target/release/*.o

.PHONY: all_clean
all_clean:
	cargo clean