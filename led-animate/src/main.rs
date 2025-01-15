#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {
    use bsp::board;
    use bsp::hal::gpio::{Input, Output};
    #[allow(unused_imports)]
    use bsp::hal::iomuxc;
    use bsp::pins::common::*;
    #[allow(unused_imports)]
    use bsp::pins::t41::*;
    use teensy4_bsp as bsp;

    use imxrt_log as logging;

    use board::t41 as my_board;

    use rtic_monotonics::systick::{Systick, *};

    /// There are no resources shared across tasks.
    #[shared]
    struct Shared {}

    /// These resources are local to individual tasks.
    #[local]
    struct Local {
        board_led: board::Led,
        button_led: Output<P24>,
        out0: Output<P0>,
        out1: Output<P1>,
        out2: Output<P2>,
        out3: Output<P3>,
        out4: Output<P4>,
        out5: Output<P5>,
        out6: Output<P6>,
        out7: Output<P7>,
        out8: Output<P8>,
        out9: Output<P9>,
        button: Input<P33>,
        poller: logging::Poller,
        counter_running: bool,
        counter_value: u32,
        last_button_state: bool,
        debounce_timer: u32,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            mut gpio1,
            mut gpio2,
            mut gpio4,
            pins,
            usb,
            ..
        } = my_board(cx.device);

        let board_led = board::led(&mut gpio2, pins.p13);
        let button_led = gpio1.output(pins.p24);
        let out0 = gpio1.output(pins.p0);
        let out1 = gpio1.output(pins.p1);
        let out2 = gpio4.output(pins.p2);
        let out3 = gpio4.output(pins.p3);
        let out4 = gpio4.output(pins.p4);
        let out5 = gpio4.output(pins.p5);
        let out6 = gpio2.output(pins.p6);
        let out7 = gpio2.output(pins.p7);
        let out8 = gpio2.output(pins.p8);
        let out9 = gpio2.output(pins.p9);
        let button = gpio4.input(pins.p33);
        let poller = logging::log::usbd(usb, logging::Interrupts::Enabled).unwrap();

        Systick::start(
            cx.core.SYST,
            board::ARM_FREQUENCY,
            rtic_monotonics::create_systick_token!(),
        );

        bin_count::spawn().unwrap();

        (
            Shared {},
            Local {
                board_led,
                button_led,
                out0,
                out1,
                out2,
                out3,
                out4,
                out5,
                out6,
                out7,
                out8,
                out9,
                button,
                poller,
                counter_running: false,
                counter_value: 0,
                last_button_state: false,
                debounce_timer: 0,
            },
        )
    }

    #[task(local = [board_led, button_led, out0, out1, out2, out3, out4, out5, out6, out7, out8, out9, button, counter_running, counter_value, last_button_state, debounce_timer])]
    async fn bin_count(cx: bin_count::Context) {
        const DEBOUNCE_THRESHOLD: u32 = 1; // Approximate debounce time in milliseconds
        const MAX_COUNT: u32 = 1023; // Maximum value for the 10-bit counter

        loop {
            let current_button_state = cx.local.button.is_set();

            // Debounce logic
            if current_button_state != *cx.local.last_button_state {
                *cx.local.debounce_timer += 1;
                if *cx.local.debounce_timer > DEBOUNCE_THRESHOLD {
                    *cx.local.last_button_state = current_button_state;
                    *cx.local.debounce_timer = 0;

                    // Button press event
                    if current_button_state {
                        if *cx.local.counter_running {
                            if *cx.local.counter_value >= MAX_COUNT {
                                *cx.local.counter_value = 0; // Reset counter
                                cx.local.button_led.clear();
                            } else {
                                *cx.local.counter_running = false; // Stop counter
                                *cx.local.counter_value = 0; // Reset counter
                                cx.local.button_led.clear();
                            }
                        } else {
                            *cx.local.counter_running = true; // Start counter
                        }
                    }
                }
            } else {
                *cx.local.debounce_timer = 0;
            }

            // Counter logic
            if *cx.local.counter_running {
                let count = *cx.local.counter_value;

                if (count >> 0) & 1 == 1 {
                    cx.local.out0.set();
                } else {
                    cx.local.out0.clear();
                }

                if (count >> 1) & 1 == 1 {
                    cx.local.out1.set();
                } else {
                    cx.local.out1.clear();
                }

                if (count >> 2) & 1 == 1 {
                    cx.local.out2.set();
                } else {
                    cx.local.out2.clear();
                }

                if (count >> 3) & 1 == 1 {
                    cx.local.out3.set();
                } else {
                    cx.local.out3.clear();
                }

                if (count >> 4) & 1 == 1 {
                    cx.local.out4.set();
                } else {
                    cx.local.out4.clear();
                }

                if (count >> 5) & 1 == 1 {
                    cx.local.out5.set();
                } else {
                    cx.local.out5.clear();
                }

                if (count >> 6) & 1 == 1 {
                    cx.local.out6.set();
                } else {
                    cx.local.out6.clear();
                }

                if (count >> 7) & 1 == 1 {
                    cx.local.out7.set();
                } else {
                    cx.local.out7.clear();
                }

                if (count >> 8) & 1 == 1 {
                    cx.local.out8.set();
                } else {
                    cx.local.out8.clear();
                }

                if (count >> 9) & 1 == 1 {
                    cx.local.out9.set();
                } else {
                    cx.local.out9.clear();
                }

                cx.local.button_led.clear();
                *cx.local.counter_value = count.wrapping_add(1);

                if *cx.local.counter_value > MAX_COUNT {
                    *cx.local.counter_running = false;
                    *cx.local.counter_value = 0; // Reset counter
                    cx.local.button_led.set();
                    cx.local.out0.clear();
                    cx.local.out1.clear();
                    cx.local.out2.clear();
                    cx.local.out3.clear();
                    cx.local.out4.clear();
                    cx.local.out5.clear();
                    cx.local.out6.clear();
                    cx.local.out7.clear();
                    cx.local.out8.clear();
                    cx.local.out9.clear();
                }
            }

            Systick::delay(30.millis()).await;
        }
    }

    #[task(binds = USB_OTG1, local = [poller])]
    fn log_over_usb(cx: log_over_usb::Context) {
        cx.local.poller.poll();
    }
}
