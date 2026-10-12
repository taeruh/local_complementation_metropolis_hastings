package = lcmh
core_lib = $(package)
core_lib_path = ../c_lib/libs/lib$(core_lib).so
libcore_lib = $(package)/lib/lib$(core_lib).so

.PHONY: $(libcore_lib) $(package)

$(package): $(libcore_lib)
	pip install .

$(libcore_lib):
	mkdir -p $(package)/lib
	cp $(core_lib_path) $@
