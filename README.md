# STM32WLE blinky without PAC

How to flash

Connect SWD using e.g. a stlink debugger. Make sure you have [cargo-flash](https://github.com/probe-rs/cargo-flash) installed.

Then run
```
cargo flash --chip stm32wle5jbix
```
