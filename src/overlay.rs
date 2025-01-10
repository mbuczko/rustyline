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
            Some(']') => Some("\x1b[35mpkg> \x1b[0m"),
            Some('?') => Some("help> "),
            _ => None,
        }
    }
}
