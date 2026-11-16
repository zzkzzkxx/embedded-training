//! Hardware wiring for the DM-MC02 teaching probe.

use embassy_stm32::{
    Peripherals, bind_interrupts,
    can::Can,
    gpio::{Flex, Level, Output, Speed},
    peripherals,
    usart::{BufferedInterruptHandler, BufferedUart, Config, DataBits, Parity, StopBits},
};

use static_cell::StaticCell;

use crate::swd::Swd;

// Bind UART8 interrupt to Embassy's buffered UART handler.
bind_interrupts!(struct Uart8Irqs {
    UART8 => BufferedInterruptHandler<peripherals::UART8>;
});

pub struct Ports {
    pub can: Can<'static>,
    pub uart: Option<BufferedUart<'static>>,
}

pub fn configure(p: Peripherals) -> (crate::board::Usb, Ports, Option<Swd>) {
    // USB configuration.
    let usb = crate::board::usb(p.USB_OTG_HS, p.PA12, p.PA11);

    // TODO 1: SWD GPIO configuration.
    // SWCLK: PC10, P16 pin 23.
    // SWDIO: PC11, P16 pin 21.
    let swd_clock = Output::new(p.PA0, Level::Low, Speed::Low);
    let swd_data = Flex::new(p.PA2);

    let swd = Some(Swd::new(swd_clock, swd_data));

    // TODO 2: UART8 configuration, 115200 baud, 8N1.
    // RX: PE0, P16 pin 15.
    // TX: PE1, P16 pin 17.

    static TX_BUFFER: StaticCell<[u8; 256]> = StaticCell::new();
    static RX_BUFFER: StaticCell<[u8; 256]> = StaticCell::new();

    let mut uart_config = Config::default();
    uart_config.baudrate = 115_200;
    uart_config.data_bits = DataBits::DataBits8;
    uart_config.parity = Parity::ParityNone;
    uart_config.stop_bits = StopBits::STOP1;

    let uart = Some(
        BufferedUart::new(
            p.UART8,
            p.PE0,
            p.PE1,
            TX_BUFFER.init([0; 256]),
            RX_BUFFER.init([0; 256]),
            Uart8Irqs,
            uart_config,
        )
        .expect("UART8 initialization failed"),
    );

    // Keep the original CAN1 configuration.
    let can = crate::board::can1(p.FDCAN1, p.PD0, p.PD1);

    (usb, Ports { can, uart }, swd)
}
