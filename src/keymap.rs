use bevy::prelude::*;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::repl::{ReplBufferEvent, ReplLifecycleEvent};

pub struct InputKeymapPlugin;

impl Plugin for InputKeymapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReplKeymap::default());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Binding {
    pub code: KeyCode,
    pub mods: KeyModifiers,
}

impl Binding {
    pub const fn new(code: KeyCode, mods: KeyModifiers) -> Self {
        Self { code, mods }
    }

    pub const fn plain(code: KeyCode) -> Self {
        Self {
            code,
            mods: KeyModifiers::NONE,
        }
    }

    pub fn matches(&self, ev: &KeyEvent) -> bool {
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

/// Keymap for mapping exact (key code, modifiers) to REPL buffer editing actions.
#[derive(Resource, Debug, Clone)]
pub struct ReplKeymap {
    pub enable: Option<Binding>,
    pub disable: Option<Binding>,
    pub toggle: Option<Binding>,
    pub submit: Option<Binding>,
    pub newline: Option<Binding>,
    pub backspace: Option<Binding>,
    pub left: Option<Binding>,
    pub right: Option<Binding>,
    pub home: Option<Binding>,
    pub end: Option<Binding>,
    pub delete: Option<Binding>,
    pub clear: Option<Binding>,
    pub clear_to_start: Option<Binding>,
    pub clear_screen: Option<Binding>,
    /// Whether to insert plain characters (no modifiers or Shift only) into buffer
    pub allow_plain_char_insert: bool,
}

impl Default for ReplKeymap {
    fn default() -> Self {
        use KeyCode as K;
        use KeyModifiers as M;
        Self {
            enable: None,
            disable: None,
            toggle: Some(Binding::plain(K::Backquote)),
            submit: Some(Binding::plain(K::Enter)),
            newline: Some(Binding::new(K::Enter, M::SHIFT)),
            backspace: Some(Binding::plain(K::Backspace)),
            left: Some(Binding::plain(K::Left)),
            right: Some(Binding::plain(K::Right)),
            home: Some(Binding::plain(K::Home)),
            end: Some(Binding::plain(K::End)),
            delete: Some(Binding::plain(K::Delete)),
            clear: Some(Binding::new(K::Char('c'), M::CONTROL)),
            clear_to_start: Some(Binding::new(K::Char('u'), M::CONTROL)),
            clear_screen: Some(Binding::new(K::Char('l'), M::CONTROL)),
            allow_plain_char_insert: true,
        }
    }
}

impl ReplKeymap {
    /// Check if the key event matches any lifecycle transitions (enable, disable, toggle).
    pub fn check_lifecycle(&self, event: &KeyEvent) -> Option<ReplLifecycleEvent> {
        if let Some(b) = &self.toggle {
            if b.matches(event) {
                return Some(ReplLifecycleEvent::Toggle);
            }
        }
        if let Some(b) = &self.enable {
            if b.matches(event) {
                return Some(ReplLifecycleEvent::Enable);
            }
        }
        if let Some(b) = &self.disable {
            if b.matches(event) {
                return Some(ReplLifecycleEvent::Disable);
            }
        }
        None
    }

    /// Map key event to a buffer modification action.
    pub fn map(&self, event: &KeyEvent) -> Option<ReplBufferEvent> {
        // Explicit bindings (exact key + modifiers), ordered by precedence
        if let Some(ev) = [
            (self.newline.as_ref(), ReplBufferEvent::Insert('\n')),
            (self.submit.as_ref(), ReplBufferEvent::Submit),
            (self.backspace.as_ref(), ReplBufferEvent::Backspace),
            (self.left.as_ref(), ReplBufferEvent::MoveLeft),
            (self.right.as_ref(), ReplBufferEvent::MoveRight),
            (self.home.as_ref(), ReplBufferEvent::JumpToStart),
            (self.end.as_ref(), ReplBufferEvent::JumpToEnd),
            (self.delete.as_ref(), ReplBufferEvent::Delete),
            (self.clear.as_ref(), ReplBufferEvent::Clear),
            (self.clear_to_start.as_ref(), ReplBufferEvent::ClearToStart),
            (self.clear_screen.as_ref(), ReplBufferEvent::ClearScreen),
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

    pub fn unset() -> Self {
        Self {
            enable: None,
            disable: None,
            toggle: None,
            submit: None,
            newline: None,
            backspace: None,
            left: None,
            right: None,
            home: None,
            end: None,
            delete: None,
            clear: None,
            clear_to_start: None,
            clear_screen: None,
            allow_plain_char_insert: false,
        }
    }
}
