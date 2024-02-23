use ads1119::{Ads1119, CmdFlags, RegSelectFlags};
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock,Transaction as I2cTransaction};

const DEFAULT_CONFIG: u8 = 0b0000_0000;
const DEFAULT_STATUS: u8 = 0b0000_0001;
const DEVICE_ADDRESS: u8 = 0b0000_0000;

fn new_ads1119(transactions: &[I2cTransaction]) -> Ads1119<I2cMock> {
    let device_address = 0;
    Ads1119::new(I2cMock::new(transactions),device_address)
}

fn destroy_ads1119(device: Ads1119<I2cMock>) {
    device.destroy().done();
}

#[test]
fn can_read_config() {
    let mut device = new_ads1119(&[
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
                vec![CmdFlags::RREG | RegSelectFlags::CONFIG],
            vec![DEFAULT_CONFIG],
        )
    ]);
    assert_eq!(device.read_config().unwrap(),0b0000_0000);
    destroy_ads1119(device);
}

#[test]
fn can_write_config() {
    let value = 0_u8;
    let mut device = new_ads1119(&[
        I2cTransaction::write(
            DEVICE_ADDRESS,
                vec![CmdFlags::WREG | RegSelectFlags::CONFIG, value],
        )
    ]);
    device.write_config(value).unwrap();
    destroy_ads1119(device);
}

#[test]
fn can_read_status() {
    let mut device = new_ads1119(&[
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
                vec![CmdFlags::RREG | RegSelectFlags::STATUS],
                vec![DEFAULT_STATUS],
        )
    ]);
    assert_eq!(device.read_status().unwrap(),1);
    destroy_ads1119(device);
}