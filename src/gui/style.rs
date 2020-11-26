use iced::{ button, container, Color, Vector};

pub struct ButtonStyle;

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(0x4A, 0x1B, 0x46).into()),
            border_radius: 2.0,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xA8, 0xA3, 0x6B).into(),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active()
        }
    }
}

pub struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb8(0x3A, 0x38, 0x5B).into()),
            text_color: Color::from_rgb8(0xA8, 0xA3, 0x6B).into(),
            border_color: Color::from_rgb8(0xA8, 0xA3, 0x6B).into(),
            ..container::Style::default()
        }
    }
}
