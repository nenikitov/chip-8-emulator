use ratatui::{layout::Size, prelude::*, widgets::*};

use super::*;

struct Registers<'a> {
    registers: &'a [u8],
}

impl<'a> WidgetSize for Registers<'a> {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        let registers: Vec<_> = self
            .registers
            .iter()
            .map(|r| Paragraph::new(format!("{:02X}", r)))
            .collect();

        LayoutLinear {
            direction: Direction::Vertical,
            children: registers
                .iter()
                .map(|r| (r as &dyn WidgetSize, None))
                .collect(),
            flex_main_axis: None,
            flex_cross_axis: false,
            spacing: 0,
        }
        .render_sized(area, buf)
    }

    fn minimum_size(&self) -> Size {
        Size {
            width: 2,
            height: self.registers.len() as u16,
        }
    }
}

pub struct MemoryScreen<'a> {
    pub app: &'a App,
}

impl<'a> WidgetSize for MemoryScreen<'a> {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        let first_registers = Registers {
            registers: &self.app.chip.memory().v[..8],
        };
        let last_registers = Registers {
            registers: &self.app.chip.memory().v[8..][..8],
        };

        let registers = LayoutLinear {
            direction: Direction::Horizontal,
            children: vec![(&first_registers, None), (&last_registers, None)],
            flex_main_axis: None,
            flex_cross_axis: false,
            spacing: 1,
        };

        let stack = self
            .app
            .chip
            .memory()
            .stack
            .iter()
            .map(|e| Paragraph::new(format!("{:04X}", e)))
            .collect::<Vec<_>>();
        let stack = LayoutLinear {
            direction: Direction::Vertical,
            children: stack.iter().map(|s| (s as &dyn WidgetSize, None)).collect(),
            flex_main_axis: None,
            flex_cross_axis: false,
            spacing: 0,
        };

        let make_title = |title: &'a str| {
            Paragraph::new(Span::styled(
                title,
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Magenta),
            ))
        };

        LayoutLinear {
            direction: Direction::Vertical,
            children: vec![
                (&make_title("MEM"), None),
                (
                    &Paragraph::new(format!("pc {:04X}", self.app.chip.memory().pc)),
                    None,
                ),
                (
                    &Paragraph::new(format!("dt {:02X}", self.app.chip.memory().dt)),
                    None,
                ),
                (
                    &Paragraph::new(format!("st {:02X}", self.app.chip.memory().st)),
                    None,
                ),
                (
                    &Paragraph::new(format!("i  {:04X}", self.app.chip.memory().i)),
                    None,
                ),
                (&make_title("REG"), None),
                (&registers, None),
                (&make_title("STK"), None),
                (&stack, None),
            ],
            flex_main_axis: None,
            flex_cross_axis: true,
            spacing: 0,
        }
        .render_sized(area, buf)
    }

    fn minimum_size(&self) -> Size {
        Size {
            width: 7,
            height: 15 + self.app.chip.memory().stack.len() as u16,
        }
    }
}

struct Key<'a> {
    app: &'a App,
    key: usize,
}

impl<'a> WidgetSize for Key<'a> {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        let mut style = Style::default();
        if self.app.chip.memory().keys[self.key] {
            style = style.add_modifier(Modifier::REVERSED);
        }

        Paragraph::new(format!("{:1X}", self.key))
            .style(style)
            .render_sized(area, buf)
    }

    fn minimum_size(&self) -> Size {
        Size {
            width: 1,
            height: 1,
        }
    }
}

struct KeyRow<'a> {
    app: &'a App,
    keys: Vec<usize>,
}

impl<'a> WidgetSize for KeyRow<'a> {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        let keys: Vec<_> = self
            .keys
            .iter()
            .map(|&key| Key { key, app: self.app })
            .collect();

        LayoutLinear {
            direction: Direction::Horizontal,
            children: keys.iter().map(|k| (k as &dyn WidgetSize, None)).collect(),
            flex_main_axis: None,
            flex_cross_axis: false,
            spacing: 0,
        }
        .render_sized(area, buf)
    }

    fn minimum_size(&self) -> Size {
        Size {
            width: self.keys.len() as u16,
            height: 1,
        }
    }
}

pub struct Keypad<'a> {
    pub app: &'a App,
}

impl<'a> WidgetSize for Keypad<'a> {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        LayoutLinear {
            direction: Direction::Vertical,
            children: vec![
                (
                    &KeyRow {
                        app: self.app,
                        keys: vec![0x1, 0x2, 0x3, 0xC],
                    },
                    None,
                ),
                (
                    &KeyRow {
                        app: self.app,
                        keys: vec![0x4, 0x5, 0x6, 0xD],
                    },
                    None,
                ),
                (
                    &KeyRow {
                        app: self.app,
                        keys: vec![0x7, 0x8, 0x9, 0xE],
                    },
                    None,
                ),
                (
                    &KeyRow {
                        app: self.app,
                        keys: vec![0xA, 0x0, 0xB, 0xF],
                    },
                    None,
                ),
            ],
            flex_main_axis: None,
            flex_cross_axis: false,
            spacing: 0,
        }
        .render_sized(area, buf)
    }

    fn minimum_size(&self) -> Size {
        Size {
            width: 4,
            height: 4,
        }
    }
}
