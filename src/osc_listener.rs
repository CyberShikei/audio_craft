// src/osc_listener.rs

use rosc::decoder::decode;
use rosc::OscPacket;
use std::io;
use std::net::UdpSocket;
use std::sync::mpsc::Sender;
use std::time::Duration;

pub fn start_osc_listener(osc_tx: Sender<OscPacket>) {
    // Set the IP and port to listen on.
    let addr = "0.0.0.0:9000";
    let socket = UdpSocket::bind(addr).expect("Failed to bind to address");
    socket
        .set_read_timeout(Some(Duration::from_millis(100)))
        .expect("Failed to set timeout");

    println!("OSC listener running on {}", addr);

    let mut buf = [0u8; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                if let Ok(packet) = decode(&buf[..size]) {
                    println!("Received OSC packet from {}: {:?}", addr, packet);
                    // Send the packet to the main thread for processing.
                    if let Err(e) = osc_tx.send(packet) {
                        eprintln!("Failed to send OSC packet: {}", e);
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // No data received within the timeout; continue looping.
            }
            Err(e) => eprintln!("Error receiving from socket: {}", e),
        }
    }
}
