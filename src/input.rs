//! Allow players to interact with your game.

mod event;
mod keyboard_and_mouse;

pub use event::{ButtonState, Event, KeyCode, MouseButton};
pub use keyboard_and_mouse::KeyboardAndMouse;

use crate::graphics::Point;

/// An input tracker.
pub trait Input {
    /// Creates a new [`Input`].
    ///
    /// [`Input`]: trait.Input.html
    fn new() -> Self;

    /// Processes an input event.
    ///
    /// This function may be called multiple times during event processing,
    /// before [`Game::interact`].
    ///
    /// [`Game::interact`]: ../trait.Game.html#method.interact
    fn update(&mut self, event: Event);

    /// Clears any temporary state that should be consumed by [`Game::interact`]
    /// and could accumulate otherwise.
    ///
    /// This method will be called after each [`Game::interact`].
    ///
    /// [`Game::interact`]: ../trait.Game.html#method.interact
    fn clear(&mut self);
}

impl Input for () {
    fn new() -> () {
        ()
    }

    fn update(&mut self, _event: Event) {}

    fn clear(&mut self) {}
}

/// A cursor position tracker.
pub trait HasCursorPosition {
    /// Returns the current cursor position.
    fn cursor_position(&self) -> Point;
}
