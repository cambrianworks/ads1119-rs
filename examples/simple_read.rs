use std::{error::Error, time::Duration};

use linux_embedded_hal::I2cdev;

use ads1119::{single_ended_rdata_to_scaled_voltage, Ads1119, MuxFlags, STATUS_CONV_RDY};

// Example of reading from the ADS1119's 4 inputs
fn main() -> Result<(), Box<dyn Error>> {
    let dev = I2cdev::new("/dev/i2c-7").unwrap();
    let mut driver = Ads1119::new(dev, 0x40);
    // loop forever
    loop {
        // read each input on the ADS1119
        for mux in [
            MuxFlags::AN0_SINGLE_ENDED,
            MuxFlags::AN1_SINGLE_ENDED,
            MuxFlags::AN2_SINGLE_ENDED,
            MuxFlags::AN3_SINGLE_ENDED,
        ] {
            let raw_value = driver.read_input(&mux)?;
            println!(
                "[{:X}] Read (conv) value: {:.5}V",
                mux.bits(),
                // convert the data to a voltage
                single_ended_rdata_to_scaled_voltage(raw_value)
            );
            // wait a bit before reading the next input
            std::thread::sleep(Duration::from_millis(500));
        }
    }
}
