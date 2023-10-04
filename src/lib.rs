use embedded_hal::i2c::I2c;

pub struct Ads1119<I2C> {
    i2c: I2C,
    // I2C address
    address: u8,
}

impl<I2C> Ads1119<I2C>
where
    I2C: I2c,
{
    pub fn new(i2c: I2C, i2c_address: u8) -> Self {
        Ads1119 {
            i2c,
            address: i2c_address,
        }
    }

    /// Read the config register
    /// See 8.5.3.6 RREG
    ///
    /// Returns the config register as a u8.
    /// See 8.6.1
    ///
    /// Means of returned byte
    /// | Bit 7 | Bit 6 | Bit 5 | Bit 4 | Bit 3 | Bit 2 | Bit 1     | Bit 0 |
    /// | MUX Selection         | GAIN  | Data Rate     | Conv Mode | VREF  |
    ///
    /// See [MuxFlags]
    pub fn read_config(&mut self) -> Result<u8, I2C::Error> {
        let mut read_buffer = [0];
        self.i2c
            .write_read(
                self.address,
                // set the config register bit
                &[CmdFlags::RREG | RegSelectFlags::CONFIG],
                &mut read_buffer,
            )
            .and(Ok(read_buffer[0]))
    }

    /// Write the config register with the given value. See [read_config] for u8 structure.
    ///
    /// See [MuxFlags]
    pub fn write_config(&mut self, value: u8) -> Result<(), I2C::Error> {
        self.i2c.write(
            self.address,
            &[CmdFlags::WREG | RegSelectFlags::CONFIG, value],
        ) //A0
          // .write(self.address, &[CmdFlags::WREG | RegFlags::CONFIG, 0xA0]) //A2
    }

    /// Read the status register.
    ///
    /// See 8.5.3.6 RREG
    /// See 8.6.1 and 8.6.2.2
    /// See [STATUS_CONV_RDY]
    ///
    /// The only bit that matters is the MSB. If set, a new conversion is ready to be read
    /// with [read_data]. If it isn't set, the application should wait and check the status register again.
    pub fn read_status(&mut self) -> Result<u8, I2C::Error> {
        let mut read_buffer = [0];
        self.i2c
            .write_read(
                self.address,
                &[CmdFlags::RREG | RegSelectFlags::STATUS],
                &mut read_buffer,
            )
            .and(Ok(read_buffer[0]))
    }

    /// In single-shot conversion mode (the only one currently supported), this starts a conversion.
    /// Before reading a result, use [read_status] to check the the conversion has finished.
    /// See 8.5.3.3
    pub fn start_sync(&mut self) -> Result<(), I2C::Error> {
        self.i2c.write(self.address, &[CmdFlags::START_SYNC])
    }

    /// Resets the device to a default state.
    /// See 8.5.3.2
    pub fn reset(&mut self) -> Result<(), I2C::Error> {
        self.i2c.write(self.address, &[CmdFlags::RESET])
    }

    /// Reads data from the currently selected input.
    /// The u16 result is encoded as Two's Complement.
    ///
    /// Currently, the library has only been used to read positive, single-ended values.
    ///
    /// See 8.5.3.5 RDATA
    /// See 8.5.2 Data Format
    pub fn read_data(&mut self) -> Result<u16, I2C::Error> {
        let mut read_buffer = [0u8, 0u8];
        self.i2c
            .write_read(self.address, &[CmdFlags::RDATA], &mut read_buffer)
            .and(Ok(u16::from_be_bytes(read_buffer)))
    }
}

/// Interpret the raw data read from one of the inputs as a voltage
/// Currently, this function assumes the reference voltage is the internal 2.048V source
/// See 8.3.3 Voltage Reference
///     8.5.2 Data Format
pub fn single_ended_rdata_to_scaled_voltage(raw_data: u16) -> f32 {
    // Check the sign bit (Bit 15)
    let unscaled_voltage = if (raw_data & 0x8000) != 0 {
        // // Two's complement conversion for negative values
        raw_data.wrapping_neg() as i16
    } else {
        raw_data as i16
    };

    // Positive value, directly scale based on the ADS1119's configuration
    // In this case, the reference voltage is 2.048V
    const REFERENCE_VOLTAGE: f32 = 2.048;

    // Scale the voltage to the desired range (e.g., 0V to 2.048V)
    (unscaled_voltage as f32 / 0x7FFF as f32) * REFERENCE_VOLTAGE
}
/// Command Flags
/// See 8.5.3
pub struct CmdFlags;
impl CmdFlags {
    pub const RESET: u8 = 0b0000_0110;
    pub const START_SYNC: u8 = 0b0000_1000;
    pub const POWER_DOWN: u8 = 0b0000_0010;
    pub const RDATA: u8 = 0b0001_0000;
    pub const RREG: u8 = 0b0010_0000;
    pub const WREG: u8 = 0b0100_0000;
}

/// Input Mux flags
/// See 8.6.2.1 Configuration Register
/// See 8.3.1 Multiplexer
pub struct MuxFlags;
impl MuxFlags {
    pub const AN0_SINGLE_ENDED: u8 = 0b0110_0000;
    pub const AN1_SINGLE_ENDED: u8 = 0b1000_0000;
    pub const AN2_SINGLE_ENDED: u8 = 0b1010_0000;
    pub const AN3_SINGLE_ENDED: u8 = 0b1100_0000;
}

/// Register flags meant to be to combined with eh RREG command to select
/// the correct register
/// See 8.5.3 (RREG)
/// See 8.6.1 - Table 8 (Register column)
pub struct RegSelectFlags;
impl RegSelectFlags {
    pub const CONFIG: u8 = 0b0000_0000;
    pub const STATUS: u8 = 0b0000_0100;
}

/// Status register bit mask for checking the status register for a "conversion result ready" value
/// See 8.6.2.2
pub const STATUS_CONV_RDY: u8 = 0b1000_0000;
