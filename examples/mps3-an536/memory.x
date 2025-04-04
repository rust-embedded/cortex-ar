/*
Memory configuration for the MPS3-AN536 machine.

See https://github.com/qemu/qemu/blob/master/hw/arm/mps3r.c
*/

MEMORY {
    QSPI  : ORIGIN = 0x08000000, LENGTH = 8M
    DDR   : ORIGIN = 0x20000000, LENGTH = 1024M
    ATCM0 : ORIGIN = 0xee000000, LENGTH = 32K
    BTCM0 : ORIGIN = 0xee100000, LENGTH = 32K
    CTCM0 : ORIGIN = 0xee200000, LENGTH = 32K
    ATCM1 : ORIGIN = 0xee400000, LENGTH = 32K
    BTCM1 : ORIGIN = 0xee500000, LENGTH = 32K
    CTCM1 : ORIGIN = 0xee600000, LENGTH = 32K
}

REGION_ALIAS("CODE", QSPI);
REGION_ALIAS("DATA", DDR);

/*
Put Core 0's stack in ATMC0

This overrides the default _stack_top value (which would be the top of the
'DATA' region)
*/
_stack_top = ORIGIN(ATCM0) + LENGTH(ATCM0);

/*
Put Core 0's stack in ATMC1
*/
_core1_stack_top = ORIGIN(ATCM1) + LENGTH(ATCM1);
