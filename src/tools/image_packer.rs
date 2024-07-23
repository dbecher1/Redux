
use macroquad::{
    color::Color,
    logging::*,
    math::Rect,
    texture::{
        load_image,
        Image,
        Texture2D,
    },
};
use ahash::AHashMap;
use walkdir::WalkDir;
use std::cmp::Ordering;

static FILE_PATH: &str = "resources/";
static SUPPORTED_FILE_TYPES: [&str; 1] = ["png"];
static SAVE_IMAGE: bool = true;


pub struct ImagePacker {
    texture: Texture2D,
    texture_rects: AHashMap<String, Rect>,
}

#[derive(Debug)]
struct ImageData {
    data: Image,
    file_name: String,
    width: usize,
    height: usize,
}

impl ImagePacker {

    pub fn texture_atlas(&self) -> &Texture2D {
        &self.texture
    }

    pub async fn new() -> Self {
        let mut texture_rects = AHashMap::new();

        let mut images: Vec<ImageData> = Vec::new();

        for entry in WalkDir::new(FILE_PATH)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file()) {
                let path = entry.path();

                // info!("Attempting to load file as image: {}", path.display());

                let ext = match path.extension() {
                    Some(p) => match p.to_str() {
                        Some(p_) => p_,
                        None => {
                            warn!("Something went wrong trying to load an image!");
                            continue
                        },
                    },
                    None => {
                        warn!("Something went wrong trying to load an image!");
                        continue
                    },
                };
                if !SUPPORTED_FILE_TYPES.contains(&ext) {
                    continue;
                }
                let file_name = String::from(path.file_stem().unwrap_or_default().to_str().unwrap_or_default());

                let data = match load_image(path.to_str().unwrap_or_default()).await {
                    Ok(img) => img,
                    Err(_) => {
                        error!("Error loading image in image packer!");
                        continue;
                    }
                };
                let (width, height) = (data.width(), data.height());
                let img_data = ImageData {
                    data,
                    file_name,
                    width,
                    height
                };
                images.push(img_data);
            }
            images.sort_by(|lhs, rhs| {
                let try_cmp = lhs.height.cmp(&rhs.height);
                match try_cmp {
                    Ordering::Equal => lhs.width.cmp(&rhs.width),
                    _ => try_cmp,
                }
            });

            // Images are sorted, packing begins

            let mut x = 0;
            let mut y = 0;
            let mut max_height = 0;
            let mut final_height = 0;
            let mut boundary = 512.; // TODO I forgot why this arbitrary value
            const BORDER: usize = 0;

            'packing: loop {
                for img in images.iter() {
                    // Check the edge case where the image is larger than the bondary
                    if img.width as f32 > boundary {
                        // Reset
                        texture_rects.clear();
                        x = 0;
                        y = 0;
                        boundary = boundary * 1.5;
                        continue 'packing;
                    }

                    // Test the width against the boundary
                    if (x + img.width) as f32 > boundary {
                        if max_height == 0 {
                            max_height = img.height;
                        }
                        // Reset to move to next line
                        y += max_height + BORDER;
                        x = 0;
                        max_height = 0;
                    }

                    // Test if we've exceeded the threshold we've set for height
                    if (y + img.height) as f32 > boundary {
                        // Reset
                        texture_rects.clear();
                        x = 0;
                        y = 0;
                        boundary = boundary * 1.5;
                        continue 'packing;
                    }
                    let rect = Rect::new(
                        x as f32,
                        y as f32,
                        img.width as f32,
                        img.height as f32
                    );
                    texture_rects.insert(img.file_name.clone(), rect);

                    x += img.width + BORDER;

                    if img.height > max_height {
                        max_height = img.height;
                        final_height = y + max_height + BORDER;
                    }
                }
                // If we've reached this, we're done with the packing, it's successful!
                break 'packing;
            }

            let mut final_image = Image::gen_image_color(boundary as u16 + 1, final_height as u16 + 1, Color::from_rgba(0, 0, 0, 0xFF));

            const ZERO_RECT: Rect = Rect{x: 0., y: 0., w: 0., h: 0.};

            // Manually copy the pixels... not too bad
            for img in images {
                let rect = texture_rects.get(&img.file_name).unwrap_or_else(|| &ZERO_RECT);
                for (yidx, y) in (rect.y as u32..(rect.y + rect.h - 1.) as u32).enumerate() {
                    for (xidx, x) in (rect.x as u32..(rect.x + rect.w - 1.) as u32).enumerate() {
                        let pixel_color = img.data.get_pixel(xidx as u32, yidx as u32);
                        final_image.set_pixel(x, y, pixel_color);
                    }
                }
            }

            if SAVE_IMAGE {
                final_image.export_png("image.png");
            }

            let texture = Texture2D::from_image(&final_image);

        Self {
            texture,
            texture_rects
        }
    }
}