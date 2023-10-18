use std::thread;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("Failed to clone socket");

        match socket.recv_from(&mut buf) {
            Ok((bytes_read, src)) => {  // Changed '(_, src)' to '(bytes_read, src)' to get the number of bytes read.
                thread::spawn(move || {
                    println!("Handling connection from {}", src);

                    sock.send_to(&buf[..bytes_read], &src) // Send only the actual data received.
                        .expect("Failed to send a response");

                    let x = std::str::from_utf8(&buf[..bytes_read]).expect("Could not write on client.");
                    println!("Message from client {}: {}", src, x); // Used 'src' as the source address of the sender.
                });
            },
            Err(e) => {
                eprintln!("Couldn't receive a datagram: {}", e);
            }
        }
    }
}
