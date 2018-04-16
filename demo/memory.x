MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 128K
    RAM   : ORIGIN = 0x1FFFF0C0, LENGTH = 16K - 0xC0
}

SECTIONS
{
    .flash_protect 0x400 :
    {
        LONG(0xFFFFFFFF);
        LONG(0xFFFFFFFF);
        LONG(0xFFFFFFFF);
        LONG(0xFFFFFFFE);
    } > FLASH
}

_stext = 0x410;


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
/* _stext = ORIGIN(FLASH) + 0x400 ; */

/* Size of the heap (in bytes) */
/* _heap_size = 1024; */
