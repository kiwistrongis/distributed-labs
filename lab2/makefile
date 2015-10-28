#globals
default: build
freshen: clean build
clean:
	rm -rf bin/*

#vars
warnings = -A dead_code -A unused_variable
options = $(warnings) -g -L bin
#options = $(warnings) -g --extern libasdf=bin/libasdf.so
lib_opt = --crate-type=rlib

#includes
#include deps.mk
include lists.mk

#compilation definitions
$(binaries): bin/% : src/%.rs
	rustc $(options) $< -o $@
$(libraries): bin/lib%.rlib : src/%.rs
	rustc $(options) $(lib_opt) $< -o $@

#commands
build: $(libraries) $(binaries)

#tests
test-server: bin/election_server
	$<

test-client: bin/election_client
	$<
