/* Default memory layout for ESP32 */
MEMORY
{
  /* These values are defined in the ESP-IDF linker script */
  IRAM:  ORIGIN = 0x40080000, LENGTH = 0x20000
  DRAM:  ORIGIN = 0x3FFB0000, LENGTH = 0x50000
  RTCFAST: ORIGIN = 0x40070000, LENGTH = 0x1000
  RTCDATA: ORIGIN = 0x50000000, LENGTH = 0x2000
}

_stack_start = ORIGIN(DRAM) + LENGTH(DRAM);
