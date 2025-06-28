use arm_pl011_uart::{
    DataBits, LineConfig, PL011Registers, Parity, StopBits, Uart, UniqueMmioPointer,
};
use core::fmt;
use core::fmt::Write;
use core::ptr::NonNull;
use spin::Mutex;

struct UartWriter(Uart<'static>);
static UART: Mutex<Option<UartWriter>> = Mutex::new(None);

impl fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s).map_err(|_| fmt::Error)?;
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut guard = UART.lock();
    if let Some(writer) = &mut *guard {
        writer.write_fmt(args).unwrap();
    } else {
        // 如果未初始化，在这里初始化
        let uart_address: *mut PL011Registers = 0x0900_0000 as *mut PL011Registers;
        let uart_pointer = unsafe { UniqueMmioPointer::new(NonNull::new(uart_address).unwrap()) };
        let mut uart = Uart::new(uart_pointer);
        let line_config = LineConfig {
            data_bits: DataBits::Bits8,
            parity: Parity::None,
            stop_bits: StopBits::One,
        };
        let _ = uart.enable(line_config, 115_200, 16_000_000);
        let mut writer = UartWriter(uart);
        writer.write_fmt(args).unwrap();

        *guard = Some(writer);
    }
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::uart_console::_print(format_args!($($arg)*))
    };
}
