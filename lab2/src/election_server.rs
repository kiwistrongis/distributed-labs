// election client daemon

//extern declarations
extern crate election_rmi;
extern crate serialize;

//standard library imports
use std::collections::HashMap;
use std::io::{
	BufferedStream, BufferedWriter, File, Acceptor, Listener};
use std::io::net::tcp::TcpListener;
use std::sync::{ Arc, Mutex};
use serialize::Decodable;
use serialize::json;
use serialize::json::Json;

//local imports
use election_rmi::rmi::{
	VoteCommand, VoteResult, StatusCommand, StatusResult};

fn main(){
	//bind tcp server socket
	let mut acceptor = TcpListener::bind( "127.0.0.1", 8192).listen().unwrap();
	println!( "listening started.");

	//create vote db
	let vote_db = VoteDB::new();
	let db_arc = Arc::new( Mutex::new( vote_db));

	//listen for connections
	for stream_opt in acceptor.incoming() {
		let db_clone = db_arc.clone();
		spawn( proc(){
			let mut stream = BufferedStream::new( stream_opt.unwrap());
			println!( "client connected.");
			//demarshall data
			let mut command_str = stream.read_line().ok().unwrap();
			command_str.pop_char();
			let command_json : Json = json::from_str( command_str.as_slice()).unwrap();
			let command = command_json.find( &"command".to_string())
				.unwrap().as_string().unwrap();
			let mut decoder = json::Decoder::new( command_json.clone());
			//choose command
			match command {
				"vote" => {
					//demarshall further
					let vote_command : VoteCommand =
						Decodable::decode( &mut decoder).unwrap();
					//make vote
					let vote_result = db_clone.lock().cmd_vote( vote_command);
					drop( db_clone);
					//send back result
					let result_string = json::encode( &vote_result);
					stream.write_line( result_string.as_slice()).unwrap();
					stream.flush().unwrap();}
				"status" => {
					//demarshall further
					let status_command : StatusCommand =
						Decodable::decode( &mut decoder).unwrap();
					//get status
					let status_result = db_clone.lock().cmd_status( status_command);
					drop( db_clone);
					//send back result
					let result_string = json::encode( &status_result);
					stream.write_line( result_string.as_slice()).unwrap();
					stream.flush().unwrap();}
				_ => {
					println!("invalid command");}}
			println!("done with client.");})}}

struct VoteDB {
	votes : HashMap< String, u64>,
	voters : HashMap< u64, bool>,
	backup : BufferedWriter<File>,
}

impl VoteDB {
	fn new() -> VoteDB {
		//init voters
		let mut voters : HashMap< u64, bool> = HashMap::new();
		for &id in [ 0u64, 1, 10, 64, 666].iter() {
			voters.insert( id, false);}

		//open backup file
		let file = File::create( &Path::new( "data/backup.txt"));
		let writer = BufferedWriter::new( file.unwrap());

		//return
		VoteDB{
			votes: HashMap::new(),
			voters: voters,
			backup: writer}}

	fn cmd_vote( &mut self, command : VoteCommand) -> VoteResult {
		//check if id is valid and person hasn't voted already
		match self.voters.find( & command.voter_id) {
			//if they haven't voted, its all okay
			Some( &false) => (),
			//otherwise deny them
			_ => return VoteResult::new( false)}
		//make vote
		let count = 
			match self.votes.find( & command.selection) {
				Some( count) => count + 1,
				None => 1};
		self.votes.insert( command.selection.clone(), count);
		//set voter as having voted
		self.voters.insert( command.voter_id, true);
		//back up vote db to file
		self.backup_vote( &command.voter_id, &command.selection);
		//return
		VoteResult::new( true)}

	fn cmd_status( &self, command : StatusCommand) -> StatusResult {
		let count : u64 =
			match self.votes.find( &command.candidate) {
				Some( count) => count.clone(),
				None => 0};
		StatusResult::new( command.candidate, count)}

	fn backup_vote( &mut self, voter_id : &u64, selection : &String){
		self.backup.write_line(
			format!("id: {}, selection: {}",
				voter_id, selection).as_slice()).unwrap();
		self.backup.flush().unwrap();}
}