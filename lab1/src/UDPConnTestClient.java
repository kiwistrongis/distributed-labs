//standard library imports
import java.net.*;
import java.io.*;
import java.nio.ByteBuffer;

public class UDPConnTestClient extends Thread {
	//entry-point method
	public static void main( String[] args){
		if( args.length < 1 )
			throw new RuntimeException( "no hostname given");
		new UDPConnTestClient( args[0]).start();}

	//members
	ReplyListenerThread reply_thread = null;
	//fields
	private static final int host_port = 6789;
	private static final int maxmsize = 4096;
	private DatagramSocket socket = null;
	private InetAddress host_addr = null;
	//test parameters
	private static final int[] test_mpses = 
		new int[]{ 64, 128, 256, 1024};
	private static final int[] test_msizes =
		new int[]{ 64, 256, 1024, 4096, 8192, 16384};
	private static final int test_mcount = 512;
	//test results
	private int tests_count;
	private int[] test_results = null;
	private int reply_count;

	//constructor
	public UDPConnTestClient( String hostname){
		//prepare for the tests
		try{ socket = new DatagramSocket();}
		catch( SocketException exception){
			throw new RuntimeException(
				"failed to create socket: %s\n", exception);}
		try{
			host_addr = InetAddress.getByName( hostname);}
		catch( UnknownHostException exception){
			throw new RuntimeException(
				"failed to resolve hostname: %s\n", exception);}
		tests_count = test_mpses.length * test_msizes.length;
		test_results = new int[ tests_count];
		reply_count = 0;}

	@Override
	public void run(){
		//start the reply listener
		reply_thread = new ReplyListenerThread();
		reply_thread.start();
		//start the tests
		int id = 0;
		for( int mps_i = 0; mps_i < test_mpses.length; mps_i++)
			for( int msize_i = 0; msize_i < test_msizes.length; msize_i++){
				int mps = test_mpses[ mps_i];
				int msize = test_msizes[ msize_i];
				System.out.printf(
					"starting test id %02d, mps: %04d, msize: %05d\n",
					id, mps, msize);
				conn_test( id, test_mcount, mps, msize);
				//give the replies some time
				reply_wait();
				id++;}
		//print results
		id = 0;
		for( int mps_i = 0; mps_i < test_mpses.length; mps_i++)
			for( int msize_i = 0; msize_i < test_msizes.length; msize_i++){
				int mps = test_mpses[ mps_i];
				int msize = test_msizes[ msize_i];
				System.out.printf(
					"result for test id: %02d mps: %04d, msize: %05d :: ",
					id, mps, msize);
				double percent = (double) test_results[id] / test_mcount;
				System.out.printf(
					"%03d / %03d ( %05.2f %% )\n",
					test_results[id], test_mcount, 100 * percent);
				conn_test( id++, test_mcount, mps, msize);}
		//clean up
		close();}

	public void close(){
		reply_thread.request_stop();
		try{ reply_thread.join();}
		catch( InterruptedException exception){}
		if( socket != null) socket.close();}

	public void conn_test( int id, int count, int mps, int msize){
		int mspm = (int) Math.round( (double) 1000 / mps);
		ByteBuffer buffer = ByteBuffer.allocate( 4);
		buffer.putInt( id);
		byte[] packet_data = new byte[msize];
		System.arraycopy(
			buffer.array(), 0, packet_data, 0, 4);
		DatagramPacket packet = new DatagramPacket(
			packet_data, msize, host_addr, host_port);
		for( int i = 0; i < count; i++){
			try{ socket.send( packet);}
			catch( IOException exception){
				System.out.printf(
					"failed to send packet: %s\n",
					exception.getMessage());}
			try{ Thread.sleep( mspm);}
			catch( InterruptedException exception){}}}

	public void reply_wait(){
		System.out.println("giving replies some time");
		int old_count = reply_count;
		int staleness = 0;
		int wait_time = 50;
		int max_staleness = 3200 / wait_time;
		while( staleness < max_staleness){
			try{ Thread.sleep( wait_time);}
			catch( InterruptedException exception){}
			if( old_count < reply_count){
				System.out.printf(
					"\rreply count: %d", reply_count);
				old_count = reply_count;
				staleness = 0;}
			else
				staleness += 1;}
		System.out.println();
		System.out.println("done waiting for replies");}

	private class ReplyListenerThread extends Thread {
		//fields
		private boolean terminate_requested = false;
		//constructor
		public ReplyListenerThread(){}
		//methods
		public void run(){
			try{ socket.setSoTimeout( 20);}
			catch( SocketException exception){
				System.out.printf(
					"failed to set socket timeout: %s\n",
					exception.getMessage());}
			//get reply
			while( ! terminate_requested){
				//prepare to receive reply
				byte[] buffer = new byte[4];
				DatagramPacket reply = new DatagramPacket(
					buffer, buffer.length);
				try{
					socket.receive( reply);}
				catch( SocketTimeoutException exception){
					continue;}
				catch( IOException exception){
					System.out.printf(
						"failed to receive a packet: %s\n",
						exception.getMessage());}
				reply_count += 1;
				int id = ByteBuffer.wrap( buffer).getInt();
				test_results[id] += 1;}}
		public void request_stop(){
			terminate_requested = true;}
	}
}

