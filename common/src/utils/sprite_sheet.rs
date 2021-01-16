use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
pub struct SpriteSheet {
    sprite_sheets: Vec<Rc<SpriteSheet1>>,
    sheet_map: HashMap<String, Rc<SpriteSheet1>>,
}

impl SpriteSheet {
    pub fn load_sprite_sheet(&mut self, text: &str) -> bool {
        if let Some(sprite_sheet) = SpriteSheet1::load(text) {
            let sprite_sheet = Rc::new(sprite_sheet);
            self.sprite_sheets.push(sprite_sheet.clone());
            for (key, _sheet) in sprite_sheet.as_ref().sheets.iter() {
                self.sheet_map.insert(key.clone(), sprite_sheet.clone());
            }
            true
        } else {
            false
        }
    }

    pub fn get(&self, key: &str) -> Option<(&Sheet, &str)> {
        self.sheet_map
            .get(key)
            .map(|ss| ss.get(key).map(|sheet| (sheet, ss.texture_name.as_str())))
            .flatten()
    }
}

#[derive(Clone)]
struct SpriteSheet1 {
    texture_name: String,
    sheets: HashMap<String, Sheet>,
}

#[derive(Clone)]
pub struct Sheet {
    pub frame: Rect,
    pub rotated: bool,
    pub trimmed: Option<Trimmed>,
}

#[derive(Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

#[derive(Clone)]
pub struct Trimmed {
    pub sprite_source_size: Rect,
    pub source_size: Size,
}

impl SpriteSheet1 {
    pub fn load(text: &str) -> Option<Self> {
        let deserialized_opt = serde_json::from_str(text);
        if let Err(_err) = deserialized_opt {
            return None;
        }
        let deserialized: Value = deserialized_opt.unwrap();

        let texture_name = get_mainname(deserialized["meta"]["image"].as_str()?);

        let mut sheets = HashMap::new();
        for (key, frame) in deserialized["frames"].as_object()? {
            let sheet = convert_sheet(frame)?;
            sheets.insert(get_mainname(key), sheet);
        }
        Some(Self { texture_name, sheets })
    }

    pub fn get(&self, key: &str) -> Option<&Sheet> {
        self.sheets.get(key)
    }
}

fn convert_sheet(sheet: &Value) -> Option<Sheet> {
    let frame = convert_rect(&sheet["frame"])?;
    let rotated = sheet["rotated"].as_bool()?;
    let trimmed = if sheet["trimmed"].as_bool() == Some(true) {
        let sprite_source_size = convert_rect(&sheet["spriteSourceSize"])?;
        let source_size = convert_size(&sheet["sourceSize"])?;
        Some(Trimmed { sprite_source_size, source_size })
    } else {
        None
    };

    Some(Sheet { frame, rotated, trimmed })
}

fn convert_rect(value: &Value) -> Option<Rect> {
    Some(Rect {
        x: value["x"].as_i64()? as i32,
        y: value["y"].as_i64()? as i32,
        w: value["w"].as_i64()? as u32,
        h: value["h"].as_i64()? as u32,
    })
}

fn convert_size(value: &Value) -> Option<Size> {
    Some(Size { w: value["w"].as_i64()? as u32, h: value["h"].as_i64()? as u32 })
}

fn get_mainname(filename: &str) -> String {
    let re = Regex::new(r"^(.*)\.\w+").unwrap();
    re.captures(filename)
        .map_or_else(|| filename.to_string(), |caps| caps.get(1).unwrap().as_str().to_string())
}
