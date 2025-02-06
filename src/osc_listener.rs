// src/osc_listener.rs

use rosc::decoder::decode;
use rosc::OscPacket;
use std::io;
use std::net::UdpSocket;
use std::sync::mpsc::Sender;
use std::time::Duration;

pub fn start_osc_listener(osc_tx: Sender<OscPacket>) {
    let addr = "127.0.0.1:9000";
    let socket = UdpSocket::bind(addr).expect("Failed to bind to address");
    socket
        .set_read_timeout(Some(Duration::from_millis(100)))
        .expect("Failed to set timeout");

    println!("OSC listener running on {}", addr);

    // Create a buffer as an array of u32 to guarantee 4-byte alignment.
    let mut buf_u32 = [0u32; 256]; // 256 * 4 = 1024 bytes.
                                   // Create a byte slice from the u32 buffer.
    let buf: &mut [u8] =
        unsafe { std::slice::from_raw_parts_mut(buf_u32.as_mut_ptr() as *mut u8, 1024) };

    loop {
        match socket.recv_from(buf) {
            Ok((size, addr)) => {
                if let Ok(packet) = decode(&buf[..size]) {
                    println!("Received OSC packet from {}: {:?}", addr, packet);
                    if let Err(e) = osc_tx.send(packet) {
                        eprintln!("Failed to send OSC packet: {}", e);
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // No data available in the allotted timeout.
            }
            Err(e) => eprintln!("Error receiving from socket: {}", e),
        }
    }
}
