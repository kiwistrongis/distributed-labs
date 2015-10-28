#![crate_name="election_rmi"]

//extern declarations
extern crate serialize;

pub mod rmi {
	//use serialize::json;

	#[deriving( Decodable, Encodable)]
	pub struct VoteCommand {
		pub command : String,
		pub selection : String,
		pub voter_id : u64,
	}
	impl VoteCommand {
		pub fn new( selection : String, voter_id : u64) -> VoteCommand {
			VoteCommand {
				command : "vote".to_string(),
				selection : selection,
				voter_id : voter_id}}
	}

	#[deriving( Decodable, Encodable)]
	pub struct StatusCommand {
		pub command : String,
		pub candidate : String,
	}
	impl StatusCommand {
		pub fn new( candidate : String) -> StatusCommand {
			StatusCommand {
				command : "status".to_string(),
				candidate : candidate}}
	}

	#[deriving( Decodable, Encodable)]
	pub struct VoteResult {
		pub confirm : bool,
	}
	impl VoteResult {
		pub fn new( confirm : bool) -> VoteResult {
			VoteResult{
				confirm : confirm}}
	}

	#[deriving( Decodable, Encodable)]
	pub struct StatusResult {
		pub candidate : String,
		pub votes : u64,
	}
	impl StatusResult {
		pub fn new( candidate : String, votes : u64) -> StatusResult {
			StatusResult {
				candidate : candidate,
				votes : votes}}
	}
}
