PACKAGE=lcmh

libname = lcmh
libsdirectory = c_lib/libs/
libdynamic = lib$(libname).so
libstatic = lib$(libname).a
dest_libdynamic = $(libsdirectory)$(libdynamic)
dest_libstatic = $(libsdirectory)$(libstatic)

rust_directory = ./lcmh/
rust_build_directory = $(rust_directory)target/release/
orig_libdynamic = $(rust_build_directory)$(libdynamic)
orig_libstatic = $(rust_build_directory)$(libstatic)
rust_source_files = $(wildcard $(rust_directory)src/*.rs)

.PHONY: all build clean c_libraries $(PACKAGE)
all: build
build: $(PACKAGE)

c_libraries: $(dest_libdynamic) $(dest_libstatic)

$(PACKAGE): $(dest_libdynamic)
	cd python_wrapper; $(MAKE) build

$(dest_libdynamic): $(orig_libdynamic) | $(libsdirectory)
	cp $< $@

$(orig_libdynamic): $(rust_source_files)
	cd $(rust_directory); cargo build --release

$(dest_libstatic): $(orig_libstatic) | $(libsdirectory)
	cp $< $@

$(orig_libstatic): $(rust_source_files)
	cd $(rust_directory); cargo build --release

%/:
	mkdir -p $@

clean:
	rm -rf $(libsdirectory) $(rust_build_directory)
	cd python_wrapper; $(MAKE) clean
