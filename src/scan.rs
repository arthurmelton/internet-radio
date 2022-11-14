use std::net::SocketAddr;
use tokio::net::TcpStream;
use crate::ARGS;
use tokio::time::{Duration, timeout};
use tokio::io::AsyncWriteExt;

pub async fn open() {
    let mut open = Vec::new();
    let mut connections = Vec::new();
    let mut i = 0;
    let sends = format!("{}{}", 200 as char, ARGS.chat);
    let sends = sends.as_bytes();
    for first in 1..=255 {
        for second in 1..=255 {
            for third in 1..=255 {
                for forth in 1..=255 {
                    connections.push(timeout(
                                Duration::from_millis(ARGS.timeout),
                                TcpStream::connect(SocketAddr::from(([first, second, third, forth], ARGS.port)))
                            ));
                    i+=1;
                    if i == ARGS.rate {
                        for x in connections {
                            match x.await {
                                Ok(x) => {
                                    match x {
                                        Ok(mut x) => {
                                            x.write_all(sends);
                                            let mut data = [0; 1];
                                            match x.try_read(&mut data) {
                                                Ok(_) => if data == [200] {
                                                    open.push(x.peer_addr());
                                                }
                                                _ => {}
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                        connections = Vec::new();
                        println!("new rate");
                        i=0;
                    }
                }
            }
        }
    }
}
