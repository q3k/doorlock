use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    mono_font::{
        MonoTextStyle, MonoTextStyleBuilder,
        ascii::{
            FONT_6X10,
            FONT_10X20,
        },
    },
    text::{Text, Alignment},
};
use core::fmt::Debug;
use embedded_graphics::Drawable;

/// Display state as requested by higher-level logic.
pub enum State {
    Idle,
    PIN {
        digits: u32,
    },
    Wrong,
    Correct,
}

/// Trait which display backends which need to implement. It's an
/// embedded_graphics DrawTarget but with an explicit flush function (which
/// doesn't seem to be part of any embedded_graphics trait?).
pub trait Display: DrawTarget<Color = BinaryColor, Error: Debug> {
    fn flush(&mut self);
}

/// Implement Dispay for the SSD1306.
// TODO(q3k): move this to lockbsp?
impl<DI, SIZE> Display for ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>> 
where
    SIZE: ssd1306::size::DisplaySize,
    DI: ssd1306::prelude::WriteOnlyDataCommand,
{
    fn flush(&mut self) {
        Self::flush(self).unwrap();
    }
}

/// Main display controller logic.
pub struct Controller<D>
where
    D: Display,
{
    state: State,
    target: D,

    text_style_small: MonoTextStyle<'static, BinaryColor>,
    text_style_large: MonoTextStyle<'static, BinaryColor>,
}

impl <D: Display> Controller<D> {
    /// Create a new display controller for a given display 'target' (Display
    /// trait).
    pub fn new(target: D) -> Self {
        let text_style_small = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();
        let text_style_large = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(BinaryColor::On)
            .build();

        Self {
            state: State::Idle,
            target,
            text_style_small,
            text_style_large,
        }
    }

    /// Set display state from higher level logic.
    pub fn set_state(&mut self, st: State) {
        self.state = st;
    }
}

impl<D> crate::Component for Controller<D>
where
    D: Display,
{
    fn tick(&mut self, _us: u64) {
        self.target.clear(BinaryColor::Off).unwrap();

        let center = embedded_graphics::geometry::Point {
            x: 64,
            y: 32,
        };
        match self.state {
            State::Idle => {
                Text::with_alignment("You shall not pass!", center, self.text_style_small, Alignment::Center)
                    .draw(&mut self.target)
                    .unwrap();
            },
            State::PIN { digits } => {
                Text::with_alignment("_ _ _ _", center, self.text_style_large, Alignment::Center)
                    .draw(&mut self.target)
                    .unwrap();

                let mut stars = heapless::String::<8>::new();
                for i in 0..4 {
                    if digits > i {
                        stars.push_str("* ").ok();
                    } else {
                        stars.push_str("  ").ok();
                    }
                }
                stars.pop();

                Text::with_alignment(stars.as_str(), center, self.text_style_large, Alignment::Center)
                    .draw(&mut self.target)
                    .unwrap();
            },
            State::Correct => {
                Text::with_alignment("Welcome back!", center, self.text_style_small, Alignment::Center)
                    .draw(&mut self.target)
                    .unwrap();
            },
            State::Wrong => {
                Text::with_alignment("Wrong PIN,", center, self.text_style_small, Alignment::Center)
                    .draw(&mut self.target)
                    .unwrap();
            },
        }


        self.target.flush();
    }
}