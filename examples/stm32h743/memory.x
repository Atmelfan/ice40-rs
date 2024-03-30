MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* STM32H743ZIT6 */
  /* STM32H742xI/743xI/753xI       */
    FLASH  : ORIGIN = 0x08000000, LENGTH = 2M

    /* DTCM  */
    RAM    : ORIGIN = 0x20000000, LENGTH = 128K

    /* AXISRAM */
    AXISRAM : ORIGIN = 0x24000000, LENGTH = 384K

    /* SRAM */
    SRAM1 : ORIGIN = 0x30000000, LENGTH = 32K
    SRAM2 : ORIGIN = 0x30020000, LENGTH = 16K
    SRAM4 : ORIGIN = 0x38000000, LENGTH = 64K

    /* Backup SRAM */
    BSRAM : ORIGIN = 0x38800000, LENGTH = 4K

    /* Instruction TCM */
    ITCM  : ORIGIN = 0x00000000, LENGTH = 64K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Example of putting non-initialized variables into custom RAM locations. */
/* This assumes you have defined a region RAM2 above, and in the Rust
   sources added the attribute `#[link_section = ".ram2bss"]` to the data
   you want to place there. */
/* Note that the section will not be zero-initialized by the runtime! */
/* SECTIONS {
     .ram2bss (NOLOAD) : ALIGN(4) {
       *(.ram2bss);
       . = ALIGN(4);
     } > RAM2
   } INSERT AFTER .bss;
*/
