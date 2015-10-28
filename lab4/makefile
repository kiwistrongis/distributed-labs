#globals
default: build
freshen: clean build
clean:
	rm -rf bin/*

#vars
warnings = -A dead_code -A unused_variable -A unused_mut
options = $(warnings) -g -L bin
lib_opt = --crate-type=rlib
dylib_opt = --crate-type=dylib
#other vars
package_file = pkg/kalev_100425828_lab4.zip

#includes
#include deps.mk
include lists.mk

#compilation definitions
$(binaries): bin/% : src/%.rs
	rustc $(options) $< -o $@
$(libraries): bin/lib%.rlib : src/%.rs
	rustc $(options) $(lib_opt) $< -o $@
$(dylibs): bin/lib%.so : src/%.rs
	rustc $(options) $(dylib_opt) $< -o $@

#commands
build: $(libraries) $(binaries)

package: $(package_file)
$(package_file): $(binaries)
	zip -FSr $(package_file) \
		bin data src \
		deps.mk lists.mk makefile readme.md \
		pkg/.gitignore

#tests
test: test-lab4

test-test: bin/test
	$<

test-lab4: bin/lab4
	$<
