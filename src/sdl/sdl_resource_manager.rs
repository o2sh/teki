use std::collections::HashMap;
use std::path::Path;

pub struct SdlResourceManager<T> {
    map: HashMap<String, T>,
}

impl<T> SdlResourceManager<T> {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn load<F: Fn(&str) -> Result<T, String>>(
        &mut self,
        base_path: &str,
        filenames: &[&str],
        loader: F,
    ) -> Result<(), String> {
        for filename in filenames {
            let resource = loader(&format!("{}/{}", base_path, filename))?;
            let key = Path::new(filename).file_stem().unwrap().to_str().unwrap();
            self.map.insert(String::from(key), resource);
        }

        Ok(())
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
        self.map.get_mut(key)
    }
}
