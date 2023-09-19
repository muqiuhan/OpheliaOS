#![no_std]
#![no_main]

use core::arch::{global_asm};
mod basic_components;

global_asm!(include_str!("./asm/boot.S"));
