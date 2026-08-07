# LED Compass

A compass application for the STM32F3 Discovery microcontroller that uses the onboard LSM303AGR magnetometer to detect cardinal directions and display them on the 8 user LEDs.

   <img src="IMG_9308.gif" width="300" alt="LED Demo" />


## Overview

This project reads magnetic field data from the LSM303AGR sensor and calculates the heading angle. Based on the calculated angle, it determines the cardinal direction (N, NE, E, SE, S, SW, W, NW) and illuminates the corresponding LED to indicate the direction.

**LED Mapping:**
- LD3 (PE8) → North
- LD5 (PE9) → Northeast  
- LD7 (PE10) → East
- LD9 (PE11) → Southeast
- LD10 (PE12) → South
- LD8 (PE13) → Southwest
- LD6 (PE14) → West
- LD4 (PE15) → Northwest

## Hardware Requirements

- **STM32F3 Discovery** microcontroller board
- USB cable for programming and debugging
- OpenOCD debugger setup
- GDB (GNU Debugger) with ARM support

## Project Structure

```
led_compass/
├── Cargo.toml              # Main project manifest
├── Cargo.lock              # Dependency lock file
├── openocd.gdb             # GDB configuration script
├── src/
│   └── main.rs             # Main compass application
└── auxiliary/
    ├── Cargo.toml          # Auxiliary library manifest
    └── src/
        └── lib.rs          # Initialization code and utilities
```

## Building

### Prerequisites

1. Install Rust and the embedded toolchain:
   ```bash
   rustup target add thumbv7em-none-eabihf
   ```

2. Install build tools:
   ```bash
   cargo install cargo-binutils
   rustup component add llvm-tools-embedded
   ```

3. Install OpenOCD and GDB:
   ```bash
   # On Linux (Ubuntu/Debian):
   sudo apt-get install openocd gdb-multiarch
   
   # On macOS:
   brew install open-ocd arm-none-eabi-gdb
   ```

### Compile

```bash
# Build the project
cargo build --release

# For embedded target
cargo build --release --target thumbv7em-none-eabihf
```

## Running and Debugging

### Start OpenOCD

In a separate terminal, start the OpenOCD debugger:

```bash
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

### Run with GDB

```bash
cargo run --release
```

Or manually with GDB:

```bash
gdb-multiarch -q -x openocd.gdb target/thumbv7em-none-eabihf/release/led_compass
```

### View ITM Output

The program outputs debug information via the ITM (Instrumentation Trace Macrocell). View it with:

```bash
# In another terminal, monitor the ITM log
tail -f itm.log
```

## How It Works

1. **Initialization**: The `auxiliary::init()` function initializes:
   - GPIO ports for LEDs (GPIOE)
   - I2C bus (I2C1) on pins PB6 and PB7
   - LSM303AGR magnetometer
   - System delay and ITM for debugging

2. **Main Loop**: Continuously:
   - Reads magnetic field data (x, y, z components)
   - Calculates heading angle using `atan2(y, x)`
   - Determines cardinal direction based on angle ranges
   - Illuminates the corresponding LED
   - Outputs debug info via ITM

3. **Angle Ranges** (degrees):
   - North: (-157.5° to -112.5°) ∪ (157.5° to 180°)
   - Northeast: 112.5° to 157.5°
   - East: 67.5° to 112.5°
   - Southeast: 22.5° to 67.5°
   - South: -22.5° to 22.5°
   - Southwest: -67.5° to -22.5°
   - West: -112.5° to -67.5°
   - Northwest: -157.5° to -112.5°

## Dependencies

- **cortex-m**: ARM Cortex-M processor core abstractions
- **cortex-m-rt**: Runtime for ARM Cortex-M processors
- **stm32f3-discovery**: HAL and board support for STM32F3 Discovery
- **stm32f3xx-hal**: Hardware abstraction layer for STM32F3
- **lsm303agr**: Driver for LSM303AGR magnetometer/accelerometer
- **micromath**: Embedded math library
- **panic_itm**: Panic handler that uses ITM for output

## Troubleshooting

### "Protocol error with Rcmd: FC"
- Ensure OpenOCD is running in a separate terminal
- Check that OpenOCD is properly configured for your board

### "No such file or directory" for openocd.gdb
- Run commands from the project root directory
- Or use absolute path to the GDB script

### Code changes not being reflected
- Clean the build cache: `cargo clean`
- Rebuild: `cargo build --release`
- Ensure the device is being reprogrammed after each build

## License

MIT or Apache-2.0 (choose one or both)

