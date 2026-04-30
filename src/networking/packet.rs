use std::net::{IpAddr, UdpSocket};

use crossbeam_queue::SegQueue;

use crate::TICK_RATE;

pub const TOO_LATE: u64 = (0.5*TICK_RATE as f32) as u64;

/// represents all possible player inputs from client
#[repr(u8)]
pub enum Input {
    Forward,
    Backward,
    Left,
    Right,
    Shoot,
    None,
}
pub enum PacketVariant {
    HeartBeat,
    /// current_rotation stores yaw and pitch, in that order, in radians.
    /// 
    /// recieved from clients.
    Input {
        input: Input,
        current_rotation: (f32, f32),
    },
}
/// tick_num is the tick counter value at the time the packet was sent.
pub struct Packet {
    pub tick_num: u64,
    pub variant: PacketVariant,
}
impl Packet {
    pub fn new_input_packet(tick_num: impl Into<u64>, current_rotation: (f32,f32), input: Input) -> Self {
        Self {
            tick_num: tick_num.into(),
            variant: PacketVariant::Input {
                input,
                current_rotation
            },
        }
    }
}
pub struct ServerPacketManager {
    pub received: SegQueue<Packet>,
    pub socket: UdpSocket,
    pub server_ip: IpAddr,
}
impl ServerPacketManager {
    pub fn new(ip: IpAddr) -> Self {
        Self {
            received: SegQueue::new(),
            socket: UdpSocket::bind("1.0.0.0").unwrap(),
            server_ip: ip,
        }
    }
    pub fn receive(&mut self, current_tick: u64) {
        let mut buf = [0u8; 1024];
        let (msg_len, sender_addr) = self.socket.recv_from(buf.as_mut_slice()).unwrap();
        let packet = Self::buf_to_packet(&buf, msg_len);
        if current_tick - packet.tick_num < TOO_LATE {
            self.received.push(packet);
        }
    }
    pub fn buf_to_packet(buf: &[u8], msg_len: usize) -> Packet {
        let tick_num_buf = &buf[0..8];
        let variant = match buf[8] {
            0 => {
                if msg_len < 8 {panic!();}

                PacketVariant::HeartBeat
            }
            1 => {
                if msg_len < 18 {panic!();}

                unsafe {
                    PacketVariant::Input {
                        input: std::mem::transmute(buf[9]),
                        current_rotation: (
                            f32::from_le_bytes([buf[10],buf[11],buf[12],buf[13]]),
                            f32::from_le_bytes([buf[14],buf[15],buf[16],buf[17]]),
                        )
                    }
                }
            },
            _ => {
                println!("invalid msg received");
                panic!()
            }
        };
        Packet {
            tick_num: u64::from_le_bytes([
                buf[0],buf[1],buf[2],buf[3],
                buf[4],buf[5],buf[6],buf[7],
            ]),
            variant
        }
    }
}