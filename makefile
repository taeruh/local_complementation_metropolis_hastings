libname = lcmh

target_directory = libs

libdynamic = lib$(libname).so
libstatic = lib$(libname).a

dest_libdynamic = $(target_directory)/$(libdynamic)
dest_libstatic = $(target_directory)/$(libstatic)

rust_directory = ./lcmh
rust_build_directory = $(rust_directory)/target/debug
orig_libdynamic = $(rust_build_directory)/$(libdynamic)
orig_libstatic = $(rust_build_directory)/$(libstatic)
rust_source_files = $(wildcard $(rust_directory)/src/*.rs)


build: $(dest_libdynamic) $(dest_libstatic)

$(dest_libdynamic): $(orig_libdynamic) | $(target_directory)
	cp $< $@

$(dest_libstatic): $(orig_libstatic) | $(target_directory)
	cp $< $@

$(orig_libdynamic): $(rust_source_files)
	cd $(rust_directory); cargo build

$(origi_libstatic): $(rust_source_files)
	cd $(rust_directory); cargo build

$(target_directory):
	mkdir -p $@

clean:
	rm -rf $(target_directory) $(dynamic_library) $(static_library) $(rust_build_directory)

.phony: build clean
