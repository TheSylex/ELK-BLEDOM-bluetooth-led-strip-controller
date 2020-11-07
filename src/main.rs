mod device;

use device::*;

pub fn main() {
    //////////Examples//////////
    //Initialize the device
    let device = BleLedDevice::new();

    //Power off the leds
    device.power_off();

    //Power on the leds
    device.power_on();

    //Set a static color
    device.set_color(255, 0, 0); // Red

    //Set led brightness (0-100)
    device.set_brightness(100);

    //Set an effect
    device.set_effect(EFFECTS.crossfade_red_green_blue_yellow_cyan_magenta_white);

    //Set effect speed (0-100)
    device.set_effect_speed(0);
    ////////////////////////////
}
