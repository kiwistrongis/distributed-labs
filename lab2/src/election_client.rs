// election client program

//extern declarations
extern crate election_rmi;
extern crate serialize;

//standard library imports
use std::io::{ BufferedStream, TcpStream};
use serialize::json;

//local imports
use election_rmi::rmi;

fn main(){
	//open connection to server
	let mut stream = BufferedStream::new(
		TcpStream::connect( "localhost", 8192));
	//get command from user
	println!( "connected to election server, do what?");
	let mut line = std::io::stdin().read_line().unwrap();
	//remove newline char
	line.pop_char();
	//parse input
	let split : Vec<&str> = line.as_slice().split( ' ').collect();
	if split.len() < 1 {
		fail!( "no command given");}
	let command = split[0];

	match command {
		"vote" => {
			//error check
			if split.len() < 3 {
				fail!( "too few arguments");}
			//parse stuff
			let candidate = split[1];
			let voter_id : u64 = from_str( split[2]).unwrap();
			let command_string = json::encode(
				& rmi::VoteCommand::new( candidate.to_string(), voter_id));
			//send command
			stream.write_line( command_string.as_slice()).unwrap();
			stream.flush().unwrap();}
		"status" => {
			//error check
			if split.len() < 2 {
				fail!( "too few arguments");}
			//parse stuff
			let candidate = split[1];
			let command_string = json::encode(
				& rmi::StatusCommand::new( candidate.to_string()));
			//send command
			stream.write_line( command_string.as_slice()).unwrap();
			stream.flush().unwrap();}
		_ => fail!( "invalid command"),}

	//get result of command
	let mut result = stream.read_line().unwrap();
	result.pop_char();
	println!( "server returned: {}", result);
	drop( stream);}
