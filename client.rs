use std::net::UdpSocket;
fn main() {
    let buffer = String::from("Hello, world!");
    let haddr = "127.0.0.1:8081";
    let addr = "127.0.0.1:8080";
    let socket = UdpSocket::bind(haddr).unwrap();

    send_packet(buffer,socket,addr);
}
fn send_packet(buffer: String, socket: UdpSocket, addr: &str) {
    let buff = buffer.as_bytes();
    socket.send_to(buff, addr).expect("Not sent");
}
