use std::net::UdpSocket;
fn main() {
    let buffer = String::new();
    let addr = "127.0.0.1:8080";
    let socket = UdpSocket::bind(addr).unwrap();

    recv_packet(&buffer, socket,);
}
fn recv_packet(buffer:&str, socket: UdpSocket,) {
    let mut buff = [0; 1024];
    let (amt, src) = socket.recv_from(&mut buff).expect("Not received");
    let data = String::from_utf8_lossy(&buff[..amt]);
    println!("Received {} bytes from {}", amt, src);
    println!("Data: {}", data);
}