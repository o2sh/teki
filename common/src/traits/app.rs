use crate::traits::Renderer;
use crate::utils::pad::Key;

pub trait App<R: Renderer> {
    fn init(&mut self, renderer: &mut R);
    fn start_title_song(&mut self);
    fn update(&mut self) -> bool;
    fn draw(&mut self, renderer: &mut R);
    fn on_key(&mut self, keycode: Key, down: bool);
}
