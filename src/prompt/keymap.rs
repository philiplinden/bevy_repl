use bevy::prelude::*;
use bevy_ratatui::crossterm::event::{KeyCode, KeyModifiers};
use bevy_ratatui::event::KeyEvent;

use crate::repl::ReplBufferEvent;

pub struct PromptKeymapPlugin;

impl Plugin for PromptKeymapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PromptKeymap::default());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Binding {
    pub code: KeyCode,
    pub mods: KeyModifiers,
}
impl Binding {
    fn matches(&self, ev: &KeyEvent) -> bool {
        // For non-character keys, SHIFT is often inconsistently reported by terminals.
        // Treat SHIFT as a no-op for non-char keys by normalizing it away for comparison.
        match self.code {
            KeyCode::Char(_) => ev.code == self.code && ev.modifiers == self.mods,
            _ => {
                ev.code == self.code
                    && normalize_nonchar_mods(ev.modifiers) == normalize_nonchar_mods(self.mods)
            }
        }
    }
}

fn normalize_nonchar_mods(mods: KeyModifiers) -> KeyModifiers {
    // Ignore SHIFT for non-character keys (Enter, arrows, etc.). Keep CONTROL/ALT.
    use KeyModifiers as M;
    let mut m = mods;
    m.set(M::SHIFT, false);
    m
}

/// Keymap for mapping exact (key code, modifiers) to REPL actions.
///
/// Examples of common combinations (bind one per action as needed):
///
/// - v: `Binding { code: KeyCode::Char('v'), mods: KeyModifiers::NONE }`
/// - Shift+v: `Binding { code: KeyCode::Char('V'), mods: KeyModifiers::SHIFT }`
/// - Ctrl+v: `Binding { code: KeyCode::Char('v'), mods: KeyModifiers::CONTROL }`
/// - Ctrl+Shift+v: `Binding { code: KeyCode::Char('V'), mods: KeyModifiers::CONTROL | KeyModifiers::SHIFT }`
/// - Ctrl+Alt+Shift+v: `Binding { code: KeyCode::Char('V'), mods: KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SHIFT }`
///
/// Notes:
/// - Crossterm typically reports shifted alphabetic keys as an uppercase `Char`, and may also set the `SHIFT` modifier. Match both `code` and `mods` exactly.
/// - Printable character insertion (no modifiers) is handled by `allow_plain_char_insert` when no explicit binding matches.
///
/// Handling capital letters (Shift-only):
/// - If you want Shift-only characters to insert normally (e.g., typing "V" inserts 'V'), you can either:
///   1) Add an explicit binding for Shift+V (takes precedence over fallback):
///      `Binding { code: KeyCode::Char('V'), mods: KeyModifiers::SHIFT }`
///   2) Relax the fallback policy in `map()` to also accept `KeyModifiers::SHIFT` (example below).
///
/// Example: extend fallback to insert when only SHIFT is pressed
/// ```ignore
/// // inside PromptKeymap::map fallback
/// use bevy_ratatui::crossterm::event::KeyModifiers as M;
/// if self.allow_plain_char_insert {
///     if let KeyCode::Char(c) = event.code {
///         if event.modifiers.is_empty() || event.modifiers == M::SHIFT {
///             return Some(ReplBufferEvent::Insert(c));
///         }
///     }
/// }
/// ```
#[derive(Resource, Debug, Clone)]
pub struct PromptKeymap {
    pub submit: Option<Binding>,
    pub backspace: Option<Binding>,
    pub left: Option<Binding>,
    pub right: Option<Binding>,
    pub home: Option<Binding>,
    pub end: Option<Binding>,
    pub delete: Option<Binding>,
    pub clear: Option<Binding>,
    // whether to insert plain chars (no modifiers) into buffer
    pub allow_plain_char_insert: bool,
}

impl Default for PromptKeymap {
    fn default() -> Self {
        use KeyCode as K;
        use KeyModifiers as M;
        Self {
            submit:    Some(Binding { code: K::Enter,     mods: M::NONE }),
            backspace: Some(Binding { code: K::Backspace, mods: M::NONE }),
            left:      Some(Binding { code: K::Left,      mods: M::NONE }),
            right:     Some(Binding { code: K::Right,     mods: M::NONE }),
            home:      Some(Binding { code: K::Home,      mods: M::NONE }),
            end:       Some(Binding { code: K::End,       mods: M::NONE }),
            delete:    Some(Binding { code: K::Delete,    mods: M::NONE }),
            clear:     Some(Binding { code: K::Esc,       mods: M::NONE }),
            allow_plain_char_insert: true,
        }
    }
}

impl PromptKeymap {
    pub fn map(&self, event: &KeyEvent) -> Option<ReplBufferEvent> {
        // Explicit bindings (exact key + modifiers), ordered by precedence
        if let Some(ev) = [
            (self.submit.as_ref(),    ReplBufferEvent::Submit),
            (self.backspace.as_ref(), ReplBufferEvent::Backspace),
            (self.left.as_ref(),      ReplBufferEvent::MoveLeft),
            (self.right.as_ref(),     ReplBufferEvent::MoveRight),
            (self.home.as_ref(),      ReplBufferEvent::JumpToStart),
            (self.end.as_ref(),       ReplBufferEvent::JumpToEnd),
            (self.delete.as_ref(),    ReplBufferEvent::Delete),
            (self.clear.as_ref(),     ReplBufferEvent::Clear),
        ]
        .into_iter()
        .find_map(|(b, out)| b.and_then(|b| b.matches(event).then_some(out)))
        {
            return Some(ev);
        }
        if self.allow_plain_char_insert {
            if let KeyCode::Char(c) = event.code {
                // Allow insertion when no modifiers or only SHIFT are pressed.
                if event.modifiers.is_empty() || event.modifiers == KeyModifiers::SHIFT {
                    return Some(ReplBufferEvent::Insert(c));
                }
            }
        }
        None
    }

    pub fn none() -> Self {
        Self {
            submit:    None,
            backspace: None,
            left:      None,
            right:     None,
            home:      None,
            end:       None,
            delete:    None,
            clear:     None,
            allow_plain_char_insert: false,
        }
    }
}
