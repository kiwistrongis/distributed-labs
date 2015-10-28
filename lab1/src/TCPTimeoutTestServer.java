import java.io.*;
import java.net.*;

public class TCPTimeoutTestServer {
	public static void main( String args[]){
		try{
			int port = 7896;
			ServerSocket socket = new ServerSocket( port);
			while( true) {
				System.out.println("waiting for connection");
				Socket client = socket.accept();
				Connection connection = new Connection( client);
				System.out.println("starting client connection");
				connection.start();}}
		catch( IOException exception){
				System.out.println(
					"Listen :" + exception.getMessage());}}

	private static class Connection extends Thread {
		BufferedReader in;
		PrintWriter out;
		Socket client;

		public Connection( Socket client) {
			try {
				this.client = client;
				in = new BufferedReader(
					new InputStreamReader( client.getInputStream()));
				out = new PrintWriter( client.getOutputStream(), true);}
			catch( IOException exception){
				System.out.printf( "error: %s\n", exception.getMessage());}
		}
		public void run(){
			try {
				int i = 0;
				while( true){
					//read a line
					if( ! in.ready()){
						try{ Thread.sleep( 100);}
						catch( InterruptedException exception){}
						continue;}
					String data = in.readLine();
					//send the line back
					if( ( i++ % 2) == 0)
						out.println( data);
					//break on terminating char
					if( data.equals(".")) break;
					//print the received line
					System.out.printf( "recieved message %s\n", data);}}
			//handle exceptions
			catch( EOFException exception){
				System.out.printf( "error: %s\n", exception.getMessage());}
			catch( IOException exception){
				System.out.printf( "error: %s\n", exception.getMessage());}
			finally {
				try {
					System.out.println("closing client connection");
					client.close();}
				catch( IOException exception){}}
		}
	}
}
