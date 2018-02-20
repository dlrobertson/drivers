use std::ops::DerefMut;

use super::{PciDev, PciHeader};
use super::header::PciHeader;

pub struct PciFunc<'pci> {
    pub dev: &'pci PciDev<'pci>,
    pub num: u8
}

impl<'pci> PciFunc<'pci> {
    pub fn header(&self) -> Option<PciHeader> {
        if unsafe { self.read(0) } != 0xFFFFFFFF {
            let mut header_base = [0x00; 0x10];
            let tail_offset = {
                let dwords = header_base.deref_mut();
                dwords.iter_mut().fold(0usize, |offset, dword| {
                    *dword = unsafe { self.read(offset as u8) };
                    offset + 4
                })
            };

            let mut header_tail = RawPciHeaderTail::default();
            {
                let dwords = header_tail.deref_mut();
                dwords.iter_mut().fold(tail_offset, |offset, dword| {
                    *dword = unsafe { self.read(offset as u8) };
                    offset + 4
                });
            }
            Some(header)
        } else {
            None
        }
    }

    pub unsafe fn read(&self, offset: u8) -> u32 {
        self.dev.read(self.num, offset)
    }

    pub unsafe fn read_range(&self, start: u8, len: u8) -> Vec<u8> {
        let mut vec = Vec::with_capacity(len);
        // populate the vec here
    }
}
