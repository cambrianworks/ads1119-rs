use std::{error::Error, time::Duration};

use linux_embedded_hal::I2cdev;

use ads1119::{single_ended_rdata_to_scaled_voltage, Ads1119, MuxFlags, STATUS_CONV_RDY};

/// This main is an example of using the library to read single-ended data
/// off each of the four inputs.
fn main() -> Result<(), Box<dyn Error>> {
    let dev = I2cdev::new("/dev/i2c-7").unwrap();
    let mut driver = Ads1119::new(dev, 0x40);

    // Reset the device to a known state (default)
    let _ = driver.reset().unwrap();
    println!("Reset called.");

    // default config shoul be 0x0
    let config = driver.read_config().unwrap();
    println!("Read config value: {:X}", config);

    // default status should not have the MSBit set
    let status = driver.read_status().unwrap();
    println!("Read status value: {:#010b}", status);

    // loop forever
    loop {
        // read each input on the ADS1119
        for mux in [
            MuxFlags::AN0_SINGLE_ENDED,
            MuxFlags::AN1_SINGLE_ENDED,
            MuxFlags::AN2_SINGLE_ENDED,
            MuxFlags::AN3_SINGLE_ENDED,
        ] {
            // write the config to set the input we want. Leave other fields unset (default)
            // println!("writing config...");
            driver.write_config(mux).unwrap();

            // read back the config as a sanity check
            let config = driver.read_config().unwrap();
            println!("Read config value: {:X}", config);

            // start a "one-shot" converstion on the selected input
            let _ = driver.start_sync().unwrap();

            // wait until the status register tells us there is data to read
            loop {
                let status = driver.read_status().unwrap();
                // println!("CURRENT STATUS: {:#010b} ", status);
                if status & STATUS_CONV_RDY != 0 {
                    println!("--> Data is READY: {:#010b} ", status);
                    break;
                }
                std::thread::sleep(Duration::from_millis(10))
            }

            // read the conversion data
            let raw_value = driver.read_data().unwrap();
            println!(
                "[{:X}] Read (conv) value: {:.5}V",
                mux,
                // convert the data to a voltage
                single_ended_rdata_to_scaled_voltage(raw_value)
            );

            // wait a bit before reading the next input
            std::thread::sleep(Duration::from_millis(1000));
        }
    }
}

#[cfg(test)]
mod test {

    const EPS: f32 = 0.0001;
    const V_MAX: f32 = 2.048;

    use super::*;
    #[test]
    fn rdata_to_voltage_0() {
        let data = [0b0000_0000, 0b0000_0000];
        assert_eq!(single_ended_rdata_to_scaled_voltage(&data), 0.0f32);
    }

    #[test]
    fn rdata_to_voltage_max_pos() {
        let data = [0b0111_1111, 0b1111_1111];
        assert!(single_ended_rdata_to_scaled_voltage(&data) - 2.048f32 < EPS);
    }

    #[test]
    fn rdata_to_voltage_lt_max() {
        let data = [0b0111_1111, 0b1111_1110];
        assert!(single_ended_rdata_to_scaled_voltage(&data) < V_MAX);
    }

    #[test]
    fn rdata_to_voltage_max_neg() {
        let data = [0b1000_0000, 0b0000_0000];
        assert!(dbg!((single_ended_rdata_to_scaled_voltage(&data) - -2.048f32).abs()) < EPS);
    }

    #[test]
    fn rdata_to_voltage_gt_max_neg() {
        let data = [0b1000_0000, 0b0000_0001];
        assert!(dbg!(single_ended_rdata_to_scaled_voltage(&data)) > -V_MAX);
    }
}
