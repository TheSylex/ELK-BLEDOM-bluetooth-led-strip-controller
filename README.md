# ELK-BLEDOM-bluetooth-led-strip-controller
Controller for sending commands to Chinese generic Bluetooth receiver ELK-BLEDOM on most RGB LED strips.

### `Initialize the device`
`let device = BleLedDevice::new();`

### `Power options`
`device.power_on();` Power ON

`device.power_off();` Power OFF

### `Time options`
`device.set_schedule_on(WEEK_DAYS.monday+WEEK_DAYS.thursday, 08, 30, true);` Set schedule for powering the leds on at a given time

`device.set_schedule_off(WEEK_DAYS.week_days, 23, 45, true);` Set schedule for powering the leds off at a given time

`device.set_custom_time(17, 00, 00, 3);` Set time (Hour, Minute, Second, Day_of_week(1-7)) ||| Hour: 17:00:00 <--> Day: 3|Wednesday

The time of the device syncs automatically with the system time when intializing a device anyway, so generally speaking, don't use this.

### `Modes`
`device.set_color(255, 150, 100);` Set static color (R,G,B)

`device.set_brightness(100);` Set led brightness (0-100)

`device.set_effect(EFFECTS.crossfade_red_green_blue_yellow_cyan_magenta_white);` Set an effect (EFFECT)

`device.set_effect_speed(0);` Set effect speed (0-100)

### `Effects`
EFFECTS.jump_red_green_blue

EFFECTS.jump_red_green_blue_yellow_cyan_magenta_white

EFFECTS.crossfade_red

EFFECTS.crossfade_green

EFFECTS.crossfade_blue

EFFECTS.crossfade_yellow

EFFECTS.crossfade_cyan

EFFECTS.crossfade_magenta

EFFECTS.crossfade_white

EFFECTS.crossfade_red_green

EFFECTS.crossfade_red_blue

EFFECTS.crossfade_green_blue

EFFECTS.crossfade_red_green_blue

EFFECTS.crossfade_red_green_blue_yellow_cyan_magenta_white

EFFECTS.blink_red

EFFECTS.blink_green

EFFECTS.blink_blue

EFFECTS.blink_yellow

EFFECTS.blink_cyan

EFFECTS.blink_magenta

EFFECTS.blink_white

EFFECTS.blink_red_green_blue_yellow_cyan_magenta_white

### `Day Options`
WEEK_DAYS.monday

WEEK_DAYS.tuesday

WEEK_DAYS.wednesday

WEEK_DAYS.thursday

WEEK_DAYS.friday

WEEK_DAYS.saturday

WEEK_DAYS.sunday

WEEK_DAYS.all

WEEK_DAYS.week_days

WEEK_DAYS.weekend_days

WEEK_DAYS.none
