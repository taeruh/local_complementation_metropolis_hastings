target_directory = target/

lib = lcmh
lib_directory = ..
lib_header_path := $(lib_directory)
lib_path := $(lib_directory)/

lib_dynamic = -l$(lib) -Wl,-rpath=$(lib_path)
lib_static = -Wl,-Bstatic -l$(lib) -Wl,-Bdynamic

CXX := clang

err := -Wall -Wextra -pedantic
std := -std=c17

HEADER_PATH := -I $(lib_header_path)
LD_LIBRARY_PATH := -L $(lib_path)


CFLAGS := $(err) $(std) $(HEADER_PATH)

# LDLIBS := $(lib_static)
LDLIBS := $(lib_dynamic)
LDFLAGS := $(LD_LIBRARY_PATH)

target/main: main.c | $(target_directory)
	$(CXX) -o $@ $^ $(CFLAGS) $(LDLIBS) $(LDFLAGS)

$(target_directory):
	mkdir -p $@

clean:
	rm -rf $(target_directory)

.phony: clean
