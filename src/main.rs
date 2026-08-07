#![deny(unsafe_code)]
#![no_main]
#![no_std]

use micromath::F32Ext;
#[allow(unused_imports)]
use auxiliary::{entry, iprint, iprintln, prelude::*};
use auxiliary::Direction;
use auxiliary::switch_hal::OutputSwitch;

use core::f32::consts::PI;
#[entry]
fn main() -> ! {
    let (_leds, mut lsm303agr, mut delay, mut itm) = auxiliary::init();
    let mut leds = _leds.into_array();
    const XYZ_GAIN: f32 = 1.5;  // Gain factor for the LSM303AGR magnetometer in microteslas per LSB (uT/LSB)
    // (mgauss/LSB)

    loop {
        // Read magnetometer field data
        let data = lsm303agr.mag_data().unwrap();

        // 1. Replicate Rust Discovery Book format (Approximate scale)
        let raw_x = (data.x / 100) as i16;
        let raw_y = (data.y / 100) as i16;
        let raw_z = (data.z / 100) as i16;
        iprintln!(&mut itm.stim[0], "I16x3 {{ x: {}, y: {}, z: {} }}", raw_x, raw_y, raw_z);

        // 2. Compute heading angle in Radians and Degrees
        let angle_rad = (data.y as f32).atan2(data.x as f32);
        let angle_deg = angle_rad * (180.0 / PI);

        iprintln!(&mut itm.stim[0], "Heading (angle_rad): {} radians", angle_rad);
        iprintln!(&mut itm.stim[0], "Heading (angle_deg): {} degrees", angle_deg);

        // Look at the signs of the x and y components to determine in which
        // quadrant the magnetic field is
        let dir = match angle_deg as f32 {
            // Quadrant I
            deg if(deg >= 22.5 && deg <= 67.5) => {
                iprintln!(&mut itm.stim[0], "Quadrant I: Southeast");
                Direction::Southeast
            },
            deg if(deg > 67.5 && deg < 112.5) => {
                iprintln!(&mut itm.stim[0], "East");
                Direction::East
            },
            // Quadrant II
            deg if(deg >= 112.5 && deg <= 157.5) => {
                iprintln!(&mut itm.stim[0], "Quadrant II: Northeast");
                Direction::Northeast
            },
            deg if(deg > 157.5 || deg < -157.5) => {
                iprintln!(&mut itm.stim[0], "North");
                Direction::North
            },
            // Quadrant III
            deg  if(deg >= -157.5 && deg <= -112.5) => {
                iprintln!(&mut itm.stim[0], "Quadrant III: Northwest");
                Direction::Northwest
            },
            deg if(deg > -112.5 && deg < -67.5) => {
                iprintln!(&mut itm.stim[0], "West");
                Direction::West
            },
            // Quadrant IV
            deg if(deg >= -67.5 && deg <= -22.5) => {
                iprintln!(&mut itm.stim[0], "Quadrant IV: Southwest");
                Direction::Southwest
            },
            deg if(deg > -22.5 && deg < 22.5) => {
                iprintln!(&mut itm.stim[0], "South");
                Direction::South
            },
            _ => {
                iprintln!(&mut itm.stim[0], "North");
                Direction::North
            }

        };

        leds.iter_mut().for_each(|led| led.off().unwrap());
        leds[dir as usize].on().unwrap();

        let x_mg = (data.x as f32) * XYZ_GAIN;
        let y_mg = (data.y as f32) * XYZ_GAIN;
        let z_mg = (data.z as f32) * XYZ_GAIN;

        // Calculate magnitude in mG
        let magnitude_mg = (x_mg * x_mg + y_mg * y_mg + z_mg * z_mg).sqrt();

        iprintln!(&mut itm.stim[0], "Magnitude: {} mG", magnitude_mg);

        delay.delay_ms(1_000_u16);
    }
}
