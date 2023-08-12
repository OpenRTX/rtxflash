/**
 * This file contains all the code needed to execute radio_tool wrap and flash operations
 */

#include "rtxflash/include/radio_tool.h"
#include "rtxflash/radio_tool/include/radio_tool/radio/radio.hpp"
#include "rtxflash/radio_tool/include/radio_tool/radio/radio_factory.hpp"
#include "rtxflash/radio_tool/include/radio_tool/radio/tyt_radio.hpp"
#include "rtxflash/radio_tool/include/radio_tool/radio/yaesu_radio.hpp"
#include <typeinfo>

namespace radio_tool::radio {

// List compatible connected devices
rust::Vec<CxxDeviceInfo> get_devices() {
    auto rdFactory = RadioFactory();
    // Copy fields from radio_tool::RadioInfo to shared struct CxxDeviceInfo
    rust::Vec<CxxDeviceInfo> devices;
    for (const auto &d : rdFactory.ListDevices())
    {
        CxxDeviceInfo info{};
        info.index = d->index;
        // Convert wstring to string to be compatible with rust::String
        info.manufacturer = std::string(d->manufacturer.begin(), d->manufacturer.end());
        info.model = std::string(d->model.begin(), d->model.end());
        info.port = std::string(d->port.begin(), d->port.end());
        devices.push_back(info);
    }
    return devices;
}

void get_device_info(uint16_t index) {
    auto rdFactory = RadioFactory();
    auto radio = rdFactory.OpenDevice(index);
    std::cout << radio->ToString() << std::endl;
    std::cout << typeid(radio).name() << std::endl;
}

void flash_device(){
    auto rdFactory = RadioFactory();
    const auto &d = rdFactory.ListDevices();
    if(d.size() <= 0)
        throw std::runtime_error("No radio detected");
    // We flash the first radio
    uint16_t index = 0;
    auto radio = rdFactory.OpenDevice(index);
    auto in_file = "./test.bin";
    radio->WriteFirmware(in_file);
}

} // namespace radio_tool::radio
