use std::collections::HashMap;
use failure::Error;

use super::texture_image::TextureImage;

pub struct TextureAtlas {
    size: (u32, u32),
    texture_blocks: HashMap<u64, [u32; 4]>,
}
impl TextureAtlas {
    pub fn new(size: (u32, u32)) -> TextureAtlas {
        TextureAtlas {
            size: size,
            texture_blocks: HashMap::<u64, [u32; 4]>::new(),
        }
    }
    pub fn get_texture_block(&mut self, texture: &TextureImage) -> Option<[u32; 4]> {
        self.texture_blocks.get(&texture.id()).map(|r| *r)
    }
    pub fn add_texture(&mut self, texture: &TextureImage) -> Result<[u32; 4], Error> {
        let pad = |rect: [u32; 4]| [rect[0] - 1, rect[1] - 1, rect[2] + 1, rect[3] + 1];
        let unpad = |rect: [u32; 4]| [rect[0] + 1, rect[1] + 1, rect[2] - 1, rect[3] - 1];
        let size = (texture.width(), texture.height());
        let tex_coords = {
            let mut y = 1;
            let mut x = 1;
            let mut coords = None;
            'outer: while y < self.size.1 - size.1 {
                let mut next_y = self.size.1;
                while x < self.size.0 - size.0 {
                    let t1 = pad([x, y, x + size.0, y + size.1]);
                    let overlap = self.texture_blocks.values().filter(|t2| {
                        !(t1[0] >= t2[2] || t2[2] <= t2[0] || t1[1] >= t2[3] || t1[3] <= t2[1])
                    });
                    let mut any_intersect = false;
                    // on the x axis, skip past any overlapping textures
                    // on the y axis, jump up to the lowest top edge in the row
                    for rect in overlap {
                        if rect[3] < next_y {
                            next_y = rect[3] + 1;
                        }
                        if rect[2] > x {
                            x = rect[2] + 1;
                        }
                        any_intersect = true;
                    }
                    if !any_intersect {
                        coords = Some(unpad(t1));
                        break 'outer;
                    }
                }
                x = 0;
                y = next_y;
            }
            coords
        };

        match tex_coords {
            Some(coords) => {
                self.texture_blocks.insert(texture.id(), coords);
                Ok(coords)
            }
            None => Err(format_err!("Texture atlas overflow")),
        }
    }
}
