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

#[link_section = "K_TEXT_START"]
#[naked]
#[no_mangle]
pub extern "C" fn start() -> ! {
    unsafe {
        asm!("
             cli\n
             cld\n
        ");
    }
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}
