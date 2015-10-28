#globals
default: build
freshen: clean build
clean:
	rm -rf bin/* jar/* pkg/*

#variables
version = 0.0.1
cp = bin
dest = -d bin
warnings =
#warnings = -Xlint:deprecation

#includes
include lists.mk

#compilation definitions
$(class_files): bin/%.class : src/%.java
	javac -cp $(cp) $(dest) $(warnings) $<

#commands
build: $(class_files)

#run: build
#	java -cp $(cp)

#test commands
#test: run

test-udp-server:
	java -cp $(cp) UDPServer

test-tcp-server:
	java -cp $(cp) TCPServer

test-udp-client:
	java -cp $(cp) UDPClient 10.20.8.73 "hello there :)"

test-tcp-client:
	java -cp $(cp) TCPClient 10.20.8.73 "hello there :)"
