#pragma once
#include "rust/cxx.h"

namespace radio_tool::radio {

void list_devices();
void flash_radio();
void reboot_radio();

} // namespace radio_tool::radio
