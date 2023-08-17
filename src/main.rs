#![no_main]
#![no_std]

use uefi_services::println;
use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    println!("\n\n");
    println!("Welcome to ktrOS UEFI Bootloader");
    println!("\nYou are always in my arms, sc.");
    println!("We are connected FOREVER.");
    system_table.boot_services().stall(10_000_000);
    loop {

    }
    Status::SUCCESS
}
