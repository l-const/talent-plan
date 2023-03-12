use std::net::{TcpStream, ToSocketAddrs};
use std::error::Error;
use std::io::prelude::*;
#[derive(Debug)]
pub struct Client {
    client: TcpStream,
}


impl Client {
    
    pub fn new<T: ToSocketAddrs>(to_socker_addrs: T) -> Result<Client, Box<dyn Error>> {
        let tcpstream = TcpStream::connect(to_socker_addrs)?;
        tcpstream.set_nonblocking(true)?;
        Ok(Self {
            client: tcpstream,
        })
    }
}


impl Write for Client {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.client.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.client.flush()
    }
}

impl Read for Client {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.client.read(buf)
    }
}