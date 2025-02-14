//! Prompt overlays API (dynamic prompt replacements)

/// This trait provides an extension interface for dynamic prompt
/// replacements (overlays) depending on leading prompt character.
pub trait Overlayer {
    /// Takes an optional first character of command line and returns
    /// modified prompt.
    /// Returns `None` if no prompt modification should happen.
    fn overlay_str(&self, ch: Option<char>) -> Option<&'static str>;
}

/// Default implementation of `Overlayer` which mimics Julia REPL behaviour.
pub struct JuliaOverlayer {}

/// Implementation of generic overlayer with configurable triggers
pub struct GenericOverlayer {
    /// Vector of (trigger, overlay)
    pub pairs: Vec<(char, &'static str)>,
}

impl Default for JuliaOverlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl JuliaOverlayer {
    /// Returns instance of `JuliaOverlay` which reacts on ']' and '?'
    /// characters.
    pub fn new() -> JuliaOverlayer {
        JuliaOverlayer {}
    }
}

impl Overlayer for () {
    fn overlay_str(&self, _ch: Option<char>) -> Option<&'static str> {
        None
    }
}

impl Overlayer for JuliaOverlayer {
    fn overlay_str(&self, ch: Option<char>) -> Option<&'static str> {
        match ch {
            Some(']') => Some("\x1b[36mpkg> \x1b[0m"),
            Some('?') => Some("\x1b[33mhelp> \x1b[0m"),
            _ => None,
        }
    }
}

impl Overlayer for GenericOverlayer {
    fn overlay_str(&self, ch: Option<char>) -> Option<&'static str> {
        self.pairs.iter().find_map(|(c, s)| {
            if ch == Some(*c) {
                return Some(*s);
            }
            None
        })
    }
}
