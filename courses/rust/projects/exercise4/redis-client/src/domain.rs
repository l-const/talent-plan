use std::net::{TcpStream, ToSocketAddrs};
use std::error::Error;
use std::time::Duration;
use std::io::Write;
use std::io::Read;
use log::{info, trace};


#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
}


impl Client {
    pub fn new<T: ToSocketAddrs>(to_socker_addrs: T) -> Result<Client, Box<dyn Error>> {
        let tcpstream = TcpStream::connect(to_socker_addrs)?;
        tcpstream.set_nonblocking(true)?;
        let client = Self {
            stream: tcpstream,
        };
        client.set_read_timeout(Some(Duration::from_millis(200)))?;
        Ok(client)
    }

    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> std::io::Result<()> {
        self.stream.set_read_timeout(timeout)
    }

    pub fn send<const M: usize>(&mut self, mut buf: [u8; M]) -> Result<String, Box<dyn Error>> {
        loop {
            if let Err(e) = self.read(&mut buf) {
                trace!("Error reading {:?}", e);
            }
            if let Ok(()) = self.read_exact(&mut buf) {
                info!("Read bytes!");
                break;
            }
        }
        let parsed_str = std::str::from_utf8(&buf)?;
        trace!("Parsed str: {parsed_str}");
        Ok(parsed_str.to_string())
    }
}


impl Write for Client {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}

impl Read for Client {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}