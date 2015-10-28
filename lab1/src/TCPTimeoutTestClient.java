//standard library imports
import java.io.*;
import java.net.*;
import java.util.Scanner;

public class TCPTimeoutTestClient {
	public static void main( String args[]){
		Scanner console = new Scanner( System.in);
		Socket socket = null;
		try{
			int port = 7896;
			socket = new Socket( args[0], port);
			socket.setSoTimeout( 50);
			BufferedReader in = new BufferedReader(
				new InputStreamReader( socket.getInputStream()));
			PrintWriter out = new PrintWriter(
				socket.getOutputStream(), true);
			while( true){
				//send a line
				String line = console.nextLine();
				out.println( line);
				//wait for reply
				String reply = null;
				try{ reply = in.readLine();}
				catch( SocketTimeoutException exception){}
				if( reply != null)
					System.out.printf(
						"server replied: %s\n", reply);
				else
					System.out.println("server failed to reply.");
				//break on terminating char
				if( line.equals( ".")) break;}}
		//handle exceptions
		catch( UnknownHostException exception){
			System.out.println("error:" + exception.getMessage());}
		catch( EOFException exception){
				System.out.println("error:" + exception.getMessage());}
		catch( IOException exception){
				System.out.println("error:" + exception.getMessage());}
		finally {
			if( socket != null)
				try { socket.close();}
				catch( IOException exception){
					System.out.println( "error:" + exception.getMessage());}}
	}
}

