use anyhow::Result;
use std::thread;
use std::time::Duration;

pub trait Connection {
    fn write(&mut self, command: &[u8]) -> Result<()>;
    fn read(&mut self) -> Result<Vec<u8>>;
    fn write_and_read(&mut self, command: &[u8]) -> Result<Vec<u8>>;
}

pub struct Serial {
    port: Box<dyn serialport::SerialPort>,
}

impl Serial {
    pub fn new(port: &str, baudrate: u32) -> Serial {
        let port = serialport::new(port, baudrate)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port.");
        Serial { port }
    }
}

impl Connection for Serial {
    fn write(&mut self, command: &[u8]) -> Result<()> {
        self.port.write_all(command)?;
        thread::sleep(Duration::from_millis(50));
        Ok(())
    }
    fn read(&mut self) -> Result<Vec<u8>> {
        let mut data = Vec::<u8>::new();
        loop {
            let mut buf = vec![0u8; 1];
            match self.port.read_exact(buf.as_mut_slice()) {
                Ok(_t) => data.extend(buf),
                Err(_e) => break,
            }
        }
        Ok(data)
    }
    fn write_and_read(&mut self, command: &[u8]) -> Result<Vec<u8>> {
        self.write(command)?;
        let res = self.read()?;
        Ok(res)
    }
}
