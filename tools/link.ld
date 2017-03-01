ENTRY(start)

SECTIONS {
	.unmapped 0x108000: AT (0) {
		hardcoded_unmapped_load_address = .;
		unmapped_ktext_start = .;
		KEEP(*(multiboot));
		*(K_TEXT_START);
		unmapped_ktext_end = .;

		unmapped_kdata_start = .;
		*(K_DATA_START);
		unmapped_kdata_end = .;
	}

	.mapped 0x80108000 + SIZEOF(.unmapped): AT (SIZEOF(.unmapped)) {
		ktext_start = .;
		*(.text .text.*);
		ktext_end = .;

		kdata_start = .;
		*(.data);
		*(.rodata .rodata.*);
		*(COMMON);
		*(.bss);
		kdata_end = .;
	}

	/DISCARD/ : {
		*(*);
	}

}
