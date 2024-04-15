use core::{fmt, ptr::addr_of_mut};
use microbit::hal::uarte::{Error, Instance, Uarte, UarteRx, UarteTx};

static mut TX_BUF: [u8; 1] = [0; 1];
static mut RX_BUF: [u8; 1] = [0; 1];

pub struct UartePort<T: Instance>(UarteTx<T>, UarteRx<T>);

impl<T: Instance> UartePort<T> {
    pub fn new(serial: Uarte<T>) -> UartePort<T> {
        let (tx, rx) = serial
            .split(unsafe { addr_of_mut!(TX_BUF).as_mut().unwrap() }, unsafe {
                addr_of_mut!(RX_BUF).as_mut().unwrap()
            })
            .unwrap();
        UartePort(tx, rx)
    }
}

impl<T: Instance> fmt::Write for UartePort<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}

impl<T: Instance> embedded_io::ErrorType for UartePort<T> {
    type Error = Error;
}

impl<T: Instance> embedded_io::Write for UartePort<T> {
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> {
        self.0.write(buffer)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.0.flush()
    }
}

impl<T: Instance> embedded_io::Read for UartePort<T> {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        self.1.read(buffer)
    }
}
