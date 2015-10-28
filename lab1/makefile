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

push:
	scp -r * kurotetsuka@whitebox:~/distributed/lab1/

#run: build
#	java -cp $(cp)

#test commands
#test: run

test-udp-connection-server:
	java -cp $(cp) UDPConnTestServer
test-udp-connection-client:
	java -cp $(cp) UDPConnTestClient localhost

test-tcp-messaging-server:
	java -cp $(cp) TCPMessageTestServer
test-tcp-messaging-client:
	java -cp $(cp) TCPMessageTestClient localhost

test-tcp-timeout-server:
	java -cp $(cp) TCPTimeoutTestServer
test-tcp-timeout-client:
	java -cp $(cp) TCPTimeoutTestClient localhost
