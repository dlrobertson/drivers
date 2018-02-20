use std::ops::{Deref, DerefMut};
use std::{slice, mem};

use super::class::PciClass;

bitflags! {
    /// Flags found in the status register of a PCI device
    pub struct HeaderType: u8 {
        /// A general PCI device (Type 0x01).
        const GENERAL       = 0b00000000;
        /// A PCI-to-PCI bridge device (Type 0x01).
        const PCITOPCI      = 0b00000001;
        /// A PCI-to-PCI bridge device (Type 0x02).
        const CARDBUSBRIDGE = 0b00000010;
        /// A multifunction device.
        const MULTIFUNCTION = 0b01000000;
        /// Mask used for fetching the header type.
        const HEADER_TYPE   = 0b00000011;
    }
}

#[derive(Default)]
pub enum PciHeader {
    General {
        pub vendor_id: u16,
        pub device_id: u16,
        pub command: u16,
        pub status: u16,
        pub revision: u8,
        pub interface: u8,
        pub subclass: u8,
        pub class: PciClass,
        pub cache_line_size: u8,
        pub latency_timer: u8,
        pub header_type: HeaderType,
        pub bist: u8,
        pub bars: [u32; 6],
        pub cardbus_cis_ptr: u32,
        pub subsystem_vendor_id: u16,
        pub subsystem_id: u16,
        pub expansion_rom_bar: u32,
        pub capabilities: u8,
        pub interrupt_line: u8,
        pub interrupt_pin: u8,
        pub min_grant: u8,
        pub max_latency: u8
    },
    PciToPci {
        pub vendor_id: u16,
        pub device_id: u16,
        pub command: u16,
        pub status: u16,
        pub revision: u8,
        pub interface: u8,
        pub subclass: u8,
        pub class: PciClass,
        pub cache_line_size: u8,
        pub latency_timer: u8,
        pub header_type: HeaderType,
        pub bist: u8,
        pub bars: [u32; 2],
        pub primary_bus_num: u8,
        pub secondary_bus_num: u8,
        pub subordinate_bus_num: u8,
        pub secondary_latency_timer: u8,
        pub io_base: u8,
        pub io_limit: u8,
        pub secondary_status: u16,
        pub mem_base: u16,
        pub mem_limit: u16,
        pub prefetch_base: u16,
        pub prefetch_limit: u16,
        pub prefetch_base_upper: u32,
        pub prefetch_limit_limit: u32,
        pub io_base_upper: u16,
        pub io_limit_upper: u16,
        pub cap_pointer: u8,
        pub expansion_rom: u32,
        pub interrupt_line: u8,
        pub interrupt_pin : u8,
        pub bridge_control: u16
    }
}

impl PciHeader {
    // TODO: Don't be lazy and use a String as the error type.
    /// Parse the bytes found in the Configuration Space of the PCI device into
    /// a more usable PciHeader.
    pub fn from_bytes(bytes: &[u8], len: usize) -> Result<PciHeader, String> {
        match HeaderType::from_bits_truncate(bytes[0x0e]) & HeaderType::HEADER_TYPE {
            GENERAL => {
            },
            PCITOPCI => {
                Err("Could not determine the header type: {:?}".to_owned())
            },
            _ => {
                Err("Could not determine the header type: {:?}".to_owned())
            }
        }
    }
}

impl Deref for RawPciHeader {
    type Target = [u32];
    fn deref(&self) -> &[u32] {
        unsafe {
            slice::from_raw_parts(self as *const RawPciHeader as *const u32,
                                  mem::size_of::<RawPciHeader>()/4) as &[u32]
        }
    }
}

impl DerefMut for PciHeader {
    fn deref_mut(&mut self) -> &mut [u32] {
        unsafe { slice::from_raw_parts_mut(self as *mut PciHeader as *mut u32, mem::size_of::<PciHeader>()/4) as &mut [u32] }
    }
}
