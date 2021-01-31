use sdl2::image::LoadTexture;
use sdl2::render::{Texture, WindowCanvas};

use crate::sdl::SdlResourceManager;

pub struct SdlTextureManager {
    resource_manager: SdlResourceManager<Texture>,
}

impl SdlTextureManager {
    pub fn new() -> Self {
        Self { resource_manager: SdlResourceManager::new() }
    }

    pub fn load(
        &mut self,
        canvas: &mut WindowCanvas,
        base_path: &str,
        filenames: &[&str],
    ) -> Result<(), String> {
        self.resource_manager.load(base_path, filenames, |path: &str| {
            let texture_creator = canvas.texture_creator();
            texture_creator.load_texture(path)
        })
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Texture> {
        self.resource_manager.get_mut(key)
    }
}
