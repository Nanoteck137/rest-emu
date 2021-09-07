ENTRY(_start)

SECTIONS
{
  . = 0;

  .text : { *(.text) *(.text.*)}

  .data : { *(.data) *(.data.*)}

  .rodata : { *(.rodata) *(.rodata.*)}

  .bss : { *(.bss) *(.bss.*)}
}
