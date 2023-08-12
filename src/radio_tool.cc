/**
 * This file contains all the code needed to execute radio_tool wrap and flash operations
 */

#include <memory>
#include "rtxflash/include/radio_tool.h"
#include "rtxflash/radio_tool/include/radio_tool/radio/radio_factory.hpp"
#include "rtxflash/radio_tool/include/radio_tool/radio/tyt_radio.hpp"
#include "rtxflash/radio_tool/include/radio_tool/radio/yaesu_radio.hpp"

namespace radio_tool::radio
{

    // List compatible connected devices
    void list_devices(){
        auto rdFactory = RadioFactory();
        const auto &d = rdFactory.ListDevices();
        for(auto i : d)
        {
            std::wcout << i->ToString() << std::endl;
        }
    }

    // Flash the first connected radio
    void flash_radio(){
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
