//standard library imports
use std::io::{
	BufferedReader, BufferedWriter, File};
use std::rt::thread::Thread;
use std::sync::Arc;
use std::sync::atomic::{ AtomicBool, Relaxed};
use std::sync::spsc_queue;
use std::sync::spsc_queue::{ Consumer, Producer};

fn main(){
	//open files
	let in_file = File::open( &Path::new( "data/in.txt"));
	let out_file = File::create( &Path::new( "data/out.txt"));
	let mut reader = BufferedReader::new( in_file.unwrap());
	let mut writer = BufferedWriter::new( out_file.unwrap());

	//make queues
	let ( mut cx0, mut px0 ) :
		( Consumer<String>, Producer<String>) =
		spsc_queue::queue( 0);
	let ( mut cx1, mut px1 ) :
		( Consumer<String>, Producer<String>) =
		spsc_queue::queue( 0);

	//make atomic flags
	let prod_done = Arc::new( AtomicBool::new( false));
	let cons0_done = Arc::new( AtomicBool::new( false));

	//spawn producer
	println!("starting producer");
	let mut prod_done2 = prod_done.clone();
	let producer = Thread::start(
		proc(){
			//read from file into queue 0
			read_to_queue( &mut reader, px0);
			prod_done2.swap( true, Relaxed);});

	//spawn consumer0
	let prod_done3 = prod_done.clone();
	let mut cons0_done2 = cons0_done.clone();
	println!("starting consumer 0");
	let consumer0 = Thread::start(
		proc(){
			//move from queue 0 into queue 1
			consume_to_queue( &mut cx0, px1, prod_done3.deref());
			cons0_done2.swap( true, Relaxed);});

	//spawn consumer1
	println!("starting consumer 1");
	let mut cons0_done3 = cons0_done.clone();
	let consumer1 = Thread::start(
		proc(){
			//consume from queue 1 into file
			consume_to_file( &mut cx1,
				&mut writer, cons0_done3.deref());});

	//finish up
	println!("joining threads");
	producer.join();
	consumer0.join();
	consumer1.join();
	println!("threads joined");
}

fn read_to_queue(
		reader : &mut BufferedReader<File>, mut px : Producer<String>){
	//for each line in file
	for line in reader.lines() {
		//unwrap result and remove trailing newline
		let mut line = line.unwrap();
		line.pop_char();
		//write line into queue
		px.push( line.to_string());}
	//finish up
	println!("read to queue done");}

fn consume_to_queue(
		cx : &mut Consumer<String>, mut px: Producer<String>,
		done : &AtomicBool){
	//until producer paired with consumer is done
	while ! done.load( Relaxed) {
		match cx.pop() {
			Some( line) => {
				//push each line to queue
				if ! line.as_slice().contains( "distributed systems"){
					px.push( line);}}
			_ => ()};}
	//until consumer is empty
	loop {
		match cx.pop() {
			Some( line) => {
				//push each line to queue
				if ! line.as_slice().contains( "distributed systems"){
					px.push( line);}}
			None => { break;}};}
	//finish up
	println!("consume to queue done");}

fn consume_to_file(
		cx : &mut Consumer<String>,
		writer : &mut BufferedWriter<File>,
		done : &AtomicBool){
	//until producer paired with consumer is done
	while ! done.load( Relaxed) {
		match cx.pop() {
			Some( line) => {
				//write each line to file
				writer.write_line( line.as_slice()).unwrap();}
			None => ()};}
	//until consumer is empty
	loop {
		match cx.pop() {
			Some( line) => {
				//write each line to file
				writer.write_line( line.as_slice()).unwrap();}
			None => { break;}};}
	//finish up
	println!("consume print done");}

fn consume_print(
		cx : &mut Consumer<String>, done : &AtomicBool){
	while ! done.load( Relaxed) {
	//until producer paired with consumer is done
		match cx.pop() {
			Some( line) => {
				//print each line
				println!("line: {}", line);}
			None => ()};}
	//until consumer is empty
	loop {
		match cx.pop() {
			Some( line) => {
				//print each line
				println!("line: {}", line);}
			None => { break;}};}
	//finish up
	println!("consume print done");}
