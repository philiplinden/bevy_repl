use bevy::prelude::KeyCode as BevyKey;
use bevy_ratatui::crossterm::event::KeyCode as CtKey;

/// Convert a crossterm KeyCode to the closest Bevy KeyCode.
/// Returns None when no meaningful mapping exists.
pub fn crossterm_to_bevy(code: &CtKey) -> Option<BevyKey> {
    Some(match code {
        CtKey::Char('`') => BevyKey::Backquote,
        CtKey::Char(' ') => BevyKey::Space,
        CtKey::Char('-') => BevyKey::Minus,
        CtKey::Char('=') => BevyKey::Equal,
        CtKey::Char('[') => BevyKey::BracketLeft,
        CtKey::Char(']') => BevyKey::BracketRight,
        CtKey::Char('\\') => BevyKey::Backslash,
        CtKey::Char(';') => BevyKey::Semicolon,
        CtKey::Char('\'') => BevyKey::Quote,
        CtKey::Char(',') => BevyKey::Comma,
        CtKey::Char('.') => BevyKey::Period,
        CtKey::Char('/') => BevyKey::Slash,

        CtKey::Enter => BevyKey::Enter,
        CtKey::Tab => BevyKey::Tab,
        CtKey::Backspace => BevyKey::Backspace,
        CtKey::Esc => BevyKey::Escape,
        CtKey::Insert => BevyKey::Insert,
        CtKey::Delete => BevyKey::Delete,
        CtKey::Home => BevyKey::Home,
        CtKey::End => BevyKey::End,
        CtKey::PageUp => BevyKey::PageUp,
        CtKey::PageDown => BevyKey::PageDown,
        CtKey::Left => BevyKey::ArrowLeft,
        CtKey::Right => BevyKey::ArrowRight,
        CtKey::Up => BevyKey::ArrowUp,
        CtKey::Down => BevyKey::ArrowDown,

        CtKey::F(1) => BevyKey::F1,
        CtKey::F(2) => BevyKey::F2,
        CtKey::F(3) => BevyKey::F3,
        CtKey::F(4) => BevyKey::F4,
        CtKey::F(5) => BevyKey::F5,
        CtKey::F(6) => BevyKey::F6,
        CtKey::F(7) => BevyKey::F7,
        CtKey::F(8) => BevyKey::F8,
        CtKey::F(9) => BevyKey::F9,
        CtKey::F(10) => BevyKey::F10,
        CtKey::F(11) => BevyKey::F11,
        CtKey::F(12) => BevyKey::F12,

        _ => return None,
    })
}

/// Convert a Bevy KeyCode to the closest crossterm KeyCode.
/// Returns None when no meaningful mapping exists.
pub fn bevy_to_crossterm(key: &BevyKey) -> Option<CtKey> {
    Some(match key {
        BevyKey::Backquote => CtKey::Char('`'),
        BevyKey::Space => CtKey::Char(' '),
        BevyKey::Minus => CtKey::Char('-'),
        BevyKey::Equal => CtKey::Char('='),
        BevyKey::BracketLeft => CtKey::Char('['),
        BevyKey::BracketRight => CtKey::Char(']'),
        BevyKey::Backslash => CtKey::Char('\\'),
        BevyKey::Semicolon => CtKey::Char(';'),
        BevyKey::Quote => CtKey::Char('\''),
        BevyKey::Comma => CtKey::Char(','),
        BevyKey::Period => CtKey::Char('.'),
        BevyKey::Slash => CtKey::Char('/'),

        BevyKey::Enter => CtKey::Enter,
        BevyKey::Tab => CtKey::Tab,
        BevyKey::Backspace => CtKey::Backspace,
        BevyKey::Escape => CtKey::Esc,
        BevyKey::Insert => CtKey::Insert,
        BevyKey::Delete => CtKey::Delete,
        BevyKey::Home => CtKey::Home,
        BevyKey::End => CtKey::End,
        BevyKey::PageUp => CtKey::PageUp,
        BevyKey::PageDown => CtKey::PageDown,
        BevyKey::ArrowLeft => CtKey::Left,
        BevyKey::ArrowRight => CtKey::Right,
        BevyKey::ArrowUp => CtKey::Up,
        BevyKey::ArrowDown => CtKey::Down,

        BevyKey::F1 => CtKey::F(1),
        BevyKey::F2 => CtKey::F(2),
        BevyKey::F3 => CtKey::F(3),
        BevyKey::F4 => CtKey::F(4),
        BevyKey::F5 => CtKey::F(5),
        BevyKey::F6 => CtKey::F(6),
        BevyKey::F7 => CtKey::F(7),
        BevyKey::F8 => CtKey::F(8),
        BevyKey::F9 => CtKey::F(9),
        BevyKey::F10 => CtKey::F(10),
        BevyKey::F11 => CtKey::F(11),
        BevyKey::F12 => CtKey::F(12),

        _ => return None,
    })
}
