#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {
    use bsp::board;
    use bsp::hal::{
        gpio::{Input, Output},
        iomuxc,
    };
    use bsp::pins::common::*;
    #[allow(unused_imports)]
    use bsp::pins::t41::*;
    use teensy4_bsp as bsp;

    use imxrt_log as logging;

    use board::t41 as my_board;

    use rtic_monotonics::systick::{Systick, *};

    // const INPUT_CONFIG: iomuxc::Config =
    //     iomuxc::Config::zero().set_pull_keeper(Some(iomuxc::PullKeeper::Pulldown100k));

    /// There are no resources shared across tasks.
    #[shared]
    struct Shared {}

    /// These resources are local to individual tasks.
    #[local]
    struct Local {
        /// The onboard LED on pin 13.
        board_led: board::Led,
        button_led: Output<P24>,
        /// LED output pins:
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
        /// Button input pins:
        in33: Input<P33>,
        /// A poller to control USB logging.
        poller: logging::Poller,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            mut gpio1,
            mut gpio2,
            mut gpio4,
            mut pins,
            usb,
            ..
        } = my_board(cx.device);

        // iomuxc::configure(&mut pins.p38, INPUT_CONFIG);

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
        let in33 = gpio4.input(pins.p33);
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
                in33,
                poller,
            },
        )
    }

    #[task(local = [board_led, button_led, out0, out1, out2, out3, out4, out5, out6, out7, out8, out9, in33])]
    async fn bin_count(cx: bin_count::Context) {
        let mut count = 0u32;
        cx.local.button_led.clear();
        loop {
            count = count.wrapping_add(1);

            let leds = [
                (count >> 0) & 1, // LED 0
                (count >> 1) & 1, // LED 1
                (count >> 2) & 1, // LED 2
                (count >> 3) & 1, // LED 3
                (count >> 4) & 1, // LED 4
                (count >> 5) & 1, // LED 5
                (count >> 6) & 1, // LED 6
                (count >> 7) & 1, // LED 7
                (count >> 8) & 1, // LED 8
                (count >> 9) & 1, // LED 9
            ];

            if cx.local.in33.is_set() {
                cx.local.button_led.set();
            } else {
                cx.local.button_led.clear();
            }

            if leds[0] == 1 {
                cx.local.out0.set();
            } else {
                cx.local.out0.clear();
            }
            if leds[1] == 1 {
                cx.local.out1.set();
            } else {
                cx.local.out1.clear();
            }
            if leds[2] == 1 {
                cx.local.out2.set();
            } else {
                cx.local.out2.clear();
            }
            if leds[3] == 1 {
                cx.local.out3.set();
            } else {
                cx.local.out3.clear();
            }
            if leds[4] == 1 {
                cx.local.out4.set();
            } else {
                cx.local.out4.clear();
            }
            if leds[5] == 1 {
                cx.local.out5.set();
            } else {
                cx.local.out5.clear();
            }
            if leds[6] == 1 {
                cx.local.out6.set();
            } else {
                cx.local.out6.clear();
            }
            if leds[7] == 1 {
                cx.local.out7.set();
            } else {
                cx.local.out7.clear();
            }
            if leds[8] == 1 {
                cx.local.out8.set();
            } else {
                cx.local.out8.clear();
            }
            if leds[9] == 1 {
                cx.local.out9.set();
            } else {
                cx.local.out9.clear();
            }

            Systick::delay(30.millis()).await;
            count = count.wrapping_add(1);
        }
    }

    #[task(binds = USB_OTG1, local = [poller])]
    fn log_over_usb(cx: log_over_usb::Context) {
        cx.local.poller.poll();
    }
}
