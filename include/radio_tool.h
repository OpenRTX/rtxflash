#pragma once
#include <rtxflash/src/lib.rs.h>
#include "rust/cxx.h"

namespace radio_tool::radio {

struct CxxDeviceInfo;

rust::Vec<CxxDeviceInfo> get_devices();
void get_device_info(uint16_t index);
void flash_device(uint16_t index, rust::Str firmware_path);

} // namespace radio_tool::radio
