#![no_std]
#![feature(start)]

mod display;
use display::Display;

extern crate alloc;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Point, Primitive, RgbColor, Size, Transform},
    primitives::{PrimitiveStyle, Rectangle},
    Drawable,
};
use ogc_rs::prelude::*;

#[start]
fn _start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

fn main() {
    let mut video = Video::init();

    Input::init(ControllerType::Wii);
    let mut wii_ctrl = Input::new(ControllerType::Wii, ControllerPort::One);
    wii_ctrl.set_data_fmt(DataFmt::ButtonsAccelIR);

    Console::init(&video);
    Video::configure(Video::get_preferred_mode());
    Video::set_next_framebuffer(video.framebuffer);
    Video::set_black(true);
    Video::flush();
    Video::wait_vsync();

    let mut wii_display = Display::new(256 * 1024);
    wii_display.setup(&mut video.render_config);
    Video::set_black(false);

    const BACKGROUND: Rectangle = Rectangle::new(Point::zero(), Size::new(640, 480));
    const POINTER: Rectangle = Rectangle::new(Point::zero(), Size::new_equal(20));
    'main_loop: loop {
        let mut pointer_style = PrimitiveStyle::with_fill(Rgb888::new(85, 26, 139));
        wii_ctrl.update();

        if wii_ctrl.is_button_down(Button::Home) {
            break 'main_loop;
        }
        if wii_ctrl.is_button_held(Button::A) {
            pointer_style.fill_color = Some(Rgb888::GREEN);
        }

        Gx::set_viewport(
            0.,
            0.,
            video.render_config.framebuffer_width.into(),
            video.render_config.embed_framebuffer_height.into(),
            0.,
            0.,
        );
        BACKGROUND
            .into_styled(PrimitiveStyle::with_fill(Rgb888::WHITE))
            .draw(&mut wii_display)
            .unwrap();

        POINTER
            .translate(Point::new(wii_ctrl.ir().0 as i32, wii_ctrl.ir().1 as i32))
            .into_styled(pointer_style)
            .draw(&mut wii_display)
            .unwrap();

        wii_display.flush(video.framebuffer);
        Video::set_next_framebuffer(video.framebuffer);
        Video::flush();
        Video::wait_vsync();
    }
}
