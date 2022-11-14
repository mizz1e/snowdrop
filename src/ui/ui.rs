use crate::anti_aim::Pitch;
use crate::State;
use core::ops::RangeInclusive;
use iced_glow::Renderer;
use iced_native::{widget, Command, Element, Length, Program};

pub struct Ui;

#[derive(Clone, Debug)]
pub enum Message {
    Thirdperson(bool),
    AntiUntrusted(bool),

    FogColor(u32),

    FogStart(f32),
    FogEnd(f32),
    FogClip(f32),

    Bloom(f32),

    ExposureMin(f32),
    ExposureMax(f32),

    FakeLag(u8),

    AntiAim(bool),
    Pitch(Pitch),
    YawOffset(f32),
    YawJitter(bool),
    Roll(bool),

    None,
}

impl Ui {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Program for Ui {
    type Renderer = Renderer;
    type Message = Message;

    #[inline]
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let state = State::get();

        match message {
            Message::AntiUntrusted(value) => state.anti_untrusted = value,
            Message::Thirdperson(value) => state.local.thirdperson.enabled = value,

            Message::FogStart(value) => state.fog_start = value,
            Message::FogEnd(value) => state.fog_end = value,
            Message::FogClip(value) => state.fog_clip = value,

            Message::Bloom(value) => state.bloom = value,

            Message::ExposureMin(value) => state.exposure_min = value,
            Message::ExposureMax(value) => state.exposure_max = value,

            Message::FakeLag(value) => state.fake_lag = value,

            Message::AntiAim(value) => state.anti_aim.enabled = value,
            Message::Pitch(value) => state.anti_aim.pitch = value,
            Message::YawJitter(value) => state.anti_aim.yaw_jitter = value,
            Message::YawOffset(value) => state.anti_aim.yaw_offset = value,
            Message::Roll(value) => state.anti_aim.roll = value,

            _ => {}
        }

        Command::none()
    }

    #[inline]
    fn view(&self) -> Element<'_, Self::Message, Self::Renderer> {
        let state = State::get();

        const COMPONENT_RANGE: RangeInclusive<f32> = 0.0..=1.0;
        const FOG_RANGE: RangeInclusive<f32> = 0.0..=10_000.0;
        const BLOOM_RANGE: RangeInclusive<f32> = 0.0..=5.0;
        const EXPOSURE_RANGE: RangeInclusive<f32> = 0.0..=10.0;
        const YAW_RANGE: RangeInclusive<f32> = -180.0..=180.0;

        // TODO: cl move client-side cap fix
        // TODO: check sv_maxusrcmdprocessticks
        const FAKE_LAG_RANGE: RangeInclusive<u8> = 0..=16;

        const PITCH_OPTIONS: &[Pitch] = &[Pitch::Default, Pitch::Up, Pitch::Down];

        let mut content = widget::Column::new();
        let anti_aim = widget::checkbox("anti-aim", state.anti_aim.enabled, Message::AntiAim);

        content = content.push(anti_aim);

        if state.anti_aim.enabled {
            let pitch = super::pick_list(
                "pitch",
                PITCH_OPTIONS,
                Some(state.anti_aim.pitch),
                Message::Pitch,
            );

            let yaw_jitter =
                widget::checkbox("yaw jitter", state.anti_aim.yaw_jitter, Message::YawJitter);

            let yaw_offset = super::slider(
                "yaw offset",
                YAW_RANGE,
                state.anti_aim.yaw_offset,
                Message::YawOffset,
            );

            let roll = widget::checkbox("roll", state.anti_aim.roll, Message::Roll);

            content = content
                .push(pitch)
                .push(yaw_jitter)
                .push(yaw_offset)
                .push(roll);
        }

        let fake_lag = super::slider("fake lag", FAKE_LAG_RANGE, state.fake_lag, Message::FakeLag);

        /*let fog_color = iced_native::row![
            widget::text("Fog Color"),
            widget::text_input("FF0000FA", "00000000", hex_color),
        ];*/

        let thirdperson = widget::checkbox(
            "thirdperson",
            state.local.thirdperson.enabled,
            Message::Thirdperson,
        );

        content = content
            .push(fake_lag)
            //.push(fog_color)
            .push(thirdperson)
            .spacing(15);

        let content = widget::scrollable(content);

        let menu = widget::container(content)
            .width(Length::Units(800))
            .height(Length::Units(640))
            .center_x()
            .center_y()
            .padding(20)
            .style(style::custom(style::menu));

        let overlay = widget::container(menu)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::custom(style::overlay));

        overlay.into()
    }
}

mod style {
    use iced_native::widget::container;
    use iced_native::{color, theme, Background, Color, Theme};

    #[inline]
    pub fn custom(f: fn(&Theme) -> container::Appearance) -> theme::Container {
        theme::Container::Custom(Box::from(f))
    }

    #[inline]
    pub fn menu(_theme: &Theme) -> container::Appearance {
        background(color!(0x000000, 0.7))
    }

    #[inline]
    pub fn overlay(_theme: &Theme) -> container::Appearance {
        background(color!(0x000000, 0.2))
    }

    #[inline]
    pub fn background(color: Color) -> container::Appearance {
        let appearance = container::Appearance {
            background: Some(Background::Color(color)),
            ..container::Appearance::default()
        };

        appearance
    }
}
