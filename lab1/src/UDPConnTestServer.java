//standard library imports
import java.net.*;
import java.io.*;
import java.nio.ByteBuffer;

public class UDPConnTestServer extends Thread {
	//entry-point method
	public static void main( String args[]){
		new UDPConnTestServer().start();}

	//fields
	private static final int port = 6789;
	private static final int maxsize = 4096;
	private boolean terminate_requested = false;
	DatagramSocket socket = null;
	private int mcount = 0;

	//constructor
	public UDPConnTestServer(){
		try {
			socket = new DatagramSocket( port);}
		catch( SocketException exception){
			throw new RuntimeException(
				"failed to create server socket",
				exception);}}

	//private methods
	@Override
	public void run(){
		while( ! terminate_requested){
			byte[] buffer = new byte[ maxsize];
			DatagramPacket message = new DatagramPacket(
				buffer, buffer.length);
			try {
				socket.receive( message);}
			catch( IOException exception){
				System.out.printf(
					"failed to receive message: %s\n",
					exception.getMessage());}
			new MessageReplyThread( message).start();
			mcount ++;
			if( ( mcount % 100 ) == 0)
				System.out.printf("\rmcount: %d", mcount);}}

	private class MessageReplyThread extends Thread{
		//fields
		private DatagramPacket message;
		//constructor
		public MessageReplyThread( DatagramPacket message){
			this.message = message;}
		//methods
		@Override
		public void run(){
			//parse message
			byte[] message_data = message.getData();
			//produce reply
			byte[] packet_data = new byte[4];
			System.arraycopy( message_data, 0, packet_data, 0, 4);
			DatagramPacket reply = new DatagramPacket(
				packet_data, packet_data.length,
				message.getAddress(), message.getPort());
			//send reply
			try {
				socket.send( reply);}
			catch( IOException exception){
				System.out.printf(
					"failed to send a reply: %s\n",
					exception.getMessage());}}
	}
}
