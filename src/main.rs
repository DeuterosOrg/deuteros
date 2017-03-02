// Copyright 2017 Jakub Jermář
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![feature(lang_items)]
#![feature(const_fn)]
#![feature(associated_consts)]
#![feature(naked_functions)]
#![feature(asm)]

#![no_std]
#![no_main]

extern crate rlibc;

extern "C" {
    static hardcoded_unmapped_load_address: u32;
}

type EntryPoint = extern "C" fn() -> !;

#[repr(C)]
pub struct MultibootHeader {
    magic: i32,
    flags: i32,
    checksum: i32,
    header: &'static MultibootHeader,
    load_adr: &'static u32,
    load_end_adr: u32,
    bss_end_adr: u32,
    entry: EntryPoint,
}

impl MultibootHeader {
    const MULTIBOOT_HEADER_MAGIC: i32 = 0x1badb002;
    const MULTIBOOT_HEADER_FLAGS: i32 = 0x00010003;

    const fn new(myself: &'static Self, ladr: &'static u32, entry: EntryPoint) -> Self {
        MultibootHeader {
            magic: Self::MULTIBOOT_HEADER_MAGIC,
            flags: Self::MULTIBOOT_HEADER_FLAGS,
            checksum: -(Self::MULTIBOOT_HEADER_MAGIC + Self::MULTIBOOT_HEADER_FLAGS),
            header: myself,
            load_adr: ladr,
            load_end_adr: 0,
            bss_end_adr: 0,
            entry: entry,
        }
    }
}

#[link_section = "multiboot"]
#[no_mangle]
pub static MULTIBOOT_HEADER: MultibootHeader =
    MultibootHeader::new(&MULTIBOOT_HEADER,
                         unsafe { &hardcoded_unmapped_load_address },
                         start);

#[repr(C)]
pub struct Des {
    limit_0_15: u16,
    base_0_15: u16,
    base_16_23: u8,
    access: u8,
    limit_16_19_s_g: u8,
    base_24_31: u8,
}

const PL_KERNEL: u8 = 0;
const PL_USER: u8 = 3;

impl Des {
    const AR_PRESENT: u8 = 1 << 7;
    const AR_DATA: u8 = 2 << 3;
    const AR_CODE: u8 = 3 << 3;
    const AR_WRITE: u8 = 1 << 1;

    const DPL_KERNEL: u8 = PL_KERNEL << 5;
    const DPL_USER: u8 = PL_USER << 5;

    const fn new(base: u32, limit: u32, access: u8, s: bool, g: bool) -> Self {
        Des {
            base_0_15: base as u16,
            base_16_23: (base >> 16) as u8,
            base_24_31: (base >> 24) as u8,
            limit_0_15: limit as u16,
            limit_16_19_s_g: (((limit >> 16) & 0xff) | (s as u32) << 6 | (g as u32) << 7) as u8,
            access: access,
        }
    }
}

const GDT_ITEMS: usize = 5;
type Gdt = [Des; GDT_ITEMS];

#[link_section = "K_DATA_START"]
#[no_mangle]
pub static mut BOOTSTRAP_GDT: Gdt =
    [// Null descriptor
     Des::new(0, 0, 0, false, false),
     // Kernel code
     Des::new(0,
              0xfffff,
              Des::AR_PRESENT | Des::AR_CODE | Des::DPL_KERNEL,
              true,
              true),
     // Kernel data
     Des::new(0,
              0xfffff,
              Des::AR_PRESENT | Des::AR_DATA | Des::AR_WRITE | Des::DPL_KERNEL,
              true,
              true),
     // User code
     Des::new(0,
              0xfffff,
              Des::AR_PRESENT | Des::AR_CODE | Des::DPL_USER,
              true,
              true),
     // User data
     Des::new(0,
              0xfffff,
              Des::AR_PRESENT | Des::AR_DATA | Des::AR_WRITE | Des::DPL_USER,
              true,
              true)];

#[repr(C,packed)]
pub struct Gdtr {
    limit: u16,
    gdt: &'static Gdt,
}

#[link_section = "K_DATA_START"]
#[no_mangle]
pub static BOOTSTRAP_GDTR: Gdtr = Gdtr {
    limit: (GDT_ITEMS * 8) as u16,
    gdt: unsafe { &BOOTSTRAP_GDT },
};

#[link_section = "K_TEXT_START"]
#[naked]
#[no_mangle]
pub extern "C" fn start() -> ! {
    unsafe {
        asm!(
        "
             cli
             cld
             lgdtl BOOTSTRAP_GDTR 
        " : : : : "volatile");
    }
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}
