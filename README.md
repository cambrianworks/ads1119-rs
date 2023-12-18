# ads1119

[TI ADS1119](https://www.ti.com/lit/gpn/ADS1119) Rust Driver

This is a partial driver and does not implement all of the ADS1119's interface (yet).  

## Supported Functionality 
- read the CONFIG and STATUS registers
- write to the CONFIG register. The only functionality being changed is selecting the desired input (AN0, AN1, AN2, AN3) in single-ended mode.
- start a new one-shot data conversion
  - read the selected input in single-ended mode
- reset the device
- read the data and convert it to a voltage. Supported range 0 -> 2.048V

## Not supported (partial list)
- other configuration values, including setting gain, conversion rate, continuous conversion, 
- reading differential voltage, and utilizing an external reference voltage
- power down. (I think this only makes sense in continuous mode)
- calibration
- utilizing the built-in noise filtering

# Running the examples

The example assumes you have an ADS119 on I2C bus 7 at address 0x40.

```sh
cargo run --example simple_read
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.



