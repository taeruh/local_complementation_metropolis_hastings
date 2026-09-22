rust_build_directory = ./extern_c_interface/target/debug
dynamic_library = $(rust_build_directory)/libextern_c_interface.so
static_library = $(rust_build_directory)/libextern_c_interface.a

libdynamic = liblcmh.so
libstatic = liblcmh.a

build: $(libdynamic) $(libstatic)

$(libdynamic): $(dynamic_library)
	cp $< $@

$(libstatic): $(static_library)
	cp $< $@

$(dynamic_library):
	cd ./extern_c_interface; cargo build

$(static_library):
	cd ./extern_c_interface; cargo build

clean:
	rm -f $(libdynamic) $(libstatic) $(dynamic_library) $(static_library)

.phony: build clean
