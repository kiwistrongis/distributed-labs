//standard library imports
import java.io.*;
import java.net.*;
import java.util.Scanner;

public class TCPMessageTestClient {
	public static void main( String args[]){
		Scanner console = new Scanner( System.in);
		Socket socket = null;
		try{
			int port = 7896;
			socket = new Socket( args[0], port);
			BufferedReader in = new BufferedReader(
				new InputStreamReader( socket.getInputStream()));
			PrintWriter out = new PrintWriter(
				socket.getOutputStream(), true);
			while( true){
				String line = console.nextLine();
				out.println( line);
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

