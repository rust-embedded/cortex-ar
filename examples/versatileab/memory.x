/*
Memory configuration for the Arm Versatile Peripheral Board.

See https://github.com/qemu/qemu/blob/master/hw/arm/versatilepb.c
*/

MEMORY {
    SDRAM : ORIGIN = 0, LENGTH = 128M
}

REGION_ALIAS("CODE", SDRAM);
REGION_ALIAS("DATA", SDRAM);
