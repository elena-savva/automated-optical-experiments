#![allow(unused)]

mod cld1015_osa;
mod n77_wavelength_sweep;
mod n77_wavelength_check;
mod n77_osa;
mod visa_error;

use std::ffi::CString;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;
use visa_rs::prelude::*;
use visa_error::io_to_vs_err;

fn main() -> visa_rs::Result<()> {
    // Initialize the VISA resource manager
    let rm = DefaultRM::new()?;

    // Define the VISA resource string for the laser diode CLD1015
    let cld1015_resource = CString::new("USB::4883::32847::M01053290::0::INSTR").unwrap();
    // Define the VISA resource string for the tunable laser N7714A
    let n77_resource = CString::new("GPIB0::21::INSTR").unwrap();
    // Define the VISA resource string for the power meter MPM210-H
    let power_meter_resource = CString::new("GPIB0::16::INSTR").unwrap();
    // Define the VISA resource string for the HP-70952B OSA
    let osa_resource = CString::new("GPIB0::23::INSTR").unwrap();
    
    // Open a session to the CLD1015
    let mut cld1015 = rm.open(
        &cld1015_resource.into(),
        AccessMode::NO_LOCK,
        Duration::from_secs(1),
    )?;
    
    // Open a session to the N7714A
    let mut n77 = rm.open(
        &n77_resource.into(),
        AccessMode::NO_LOCK,
        Duration::from_secs(1),
    )?;
    
    // Open a session to the power meter
    let mut power_meter = rm.open(
        &power_meter_resource.into(),
        AccessMode::NO_LOCK,
        Duration::from_secs(1),
    )?;

    // Open a session to the OSA
    let mut osa = rm.open(
        &osa_resource.into(),
        AccessMode::NO_LOCK,
        Duration::from_secs(1),
    )?;

    // Send the *CLS command to the CLD1015 to clear errors
    cld1015.write_all(b"*CLS\n").map_err(io_to_vs_err)?;
    
    // Send the *CLS command to the laser to clear errors
    n77.write_all(b"*CLS\n").map_err(io_to_vs_err)?;
    
    // Clear the OSA and perform instrument preset
    osa.write_all(b"CLS;IP;\n").map_err(io_to_vs_err)?;

    // Send the *IDN? command to verify CLD1015 connection
    cld1015.write_all(b"*IDN?\n").map_err(io_to_vs_err)?;
    // Read the response from the CLD1015
    let mut response = String::new();
    {
        // Create a new scope to ensure the BufReader is dropped before we use cld1015 again
        let mut reader = BufReader::new(&cld1015);
        reader.read_line(&mut response).map_err(io_to_vs_err)?;
    }
    // Print the CLD1015 response
    println!("CLD1015 Response: {}", response);

    // Send the *IDN? command to verify laser connection
    n77.write_all(b"*IDN?\n").map_err(io_to_vs_err)?;
    // Read the response from the laser
    let mut response = String::new();
    {
        // Create a new scope to ensure the BufReader is dropped before we use laser again
        let mut reader = BufReader::new(&n77);
        reader.read_line(&mut response).map_err(io_to_vs_err)?;
    }
    // Print the laser response
    println!("Laser Response: {}", response);
    
    // Send the *IDN? command to verify power meter connection
    power_meter.write_all(b"*IDN?\n").map_err(io_to_vs_err)?;
    // Read the response from the power meter
    let mut response = String::new();
    {
        let mut reader = BufReader::new(&power_meter);
        reader.read_line(&mut response).map_err(io_to_vs_err)?;
    }
    // Print the power meter response
    println!("Power Meter Response: {}", response);

    // Check the OSA identity
    osa.write_all(b"ID?;\n").map_err(io_to_vs_err)?;
    let mut response = String::new();
    {
        let mut reader = BufReader::new(&osa);
        reader.read_line(&mut response).map_err(io_to_vs_err)?;
    }
    // Print the OSA response
    println!("Optical Spectrum Analyzer Response: {}", response);
    
    // Configure the current sweep
    let start_ma = 0.0;     // Start at 0 mA
    let stop_ma = 100.0;    // End at 100 mA
    let step_ma = 0.1;      // 1 mA steps
    let dwell_time_ms = 50; // 100ms stabilization delay

    // Configure the wavelength sweep
    let start_nm = 1528.00;        // minimum 1527.60 nm
    let stop_nm = 1570.00;         // maximum 1570.01 nm
    let step_nm = 1.0;             // 1 nm steps
    let stabilization_time_ms = 3000; // 3s stabilization delay
    let wavelength = 1560.00; // example
    
    cld1015_osa::run_current_sweep(&mut cld1015, &mut osa, start_ma, stop_ma, step_ma, dwell_time_ms)?;
    //n77_wavelength_check::run_wavelength_check(&mut n77, &mut power_meter, wavelength, stabilization_time_ms)?;
    //n77_wavelength_sweep::run_wavelength_sweep(&mut n77, &mut power_meter, start_nm, stop_nm, step_nm, stabilization_time_ms)?;
    //n77_osa::run_wavelength_sweep_osa(&mut n77, &mut osa, start_nm, stop_nm, step_nm, stabilization_time_ms)?;
    
    Ok(())
}