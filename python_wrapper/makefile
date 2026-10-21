PACKAGE=lcmh

core_lib = libs/lib$(PACKAGE).so
orig_core_lib = ../c_lib/$(core_lib)
dest_core_lib = $(PACKAGE)/$(core_lib)

.PHONY: all build clean $(PACKAGE) install_cabaliser

all: build
build: $(PACKAGE)

$(PACKAGE): $(dest_core_lib)
	pip install .

$(dest_core_lib): $(orig_core_lib) | $(PACKAGE)/libs/
	cp $< $@

$(orig_core_lib): 
	cd ..; $(MAKE) c_lib/$(core_lib)

install_cabaliser: cabaliser
	cd cabaliser; $(MAKE)

cabaliser:
	git clone git@github.com:taeruh/cabaliser.git
	cd cabaliser; git checkout lcmh_patch

%/:
	mkdir -p $@

clean:
	rm -rf build/ cabaliser/ $(PACKAGE)/libs/ *.egg-info
