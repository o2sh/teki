extern crate teki_ecs;
extern crate web_sys;

mod wasm_app;
mod wasm_audio;
mod wasm_renderer;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
