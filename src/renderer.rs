use failure::Error;

use embla::graphics::{TextureAtlas, TextureImage};
use embla::math::Vec2;
use embla::rendering::{
    Program, Renderer, Texture, TextureFiltering, Uniform, Vertex, VertexAttributeType,
    VertexBuffer,
};

use render_interface::RenderInterface;

static VERTEX_SHADER: &'static str = include_str!("../shaders/vertex.glsl");
static FRAGMENT_SHADER: &'static str = include_str!("../shaders/fragment.glsl");

pub struct TexturedVertex {
    pub position: (f32, f32),
    pub tex_coord: (f32, f32),
    pub color: (f32, f32, f32, f32),
}

impl Vertex for TexturedVertex {
    fn attributes() -> Vec<(String, usize, VertexAttributeType)> {
        vec![
            ("position".into(), 2, VertexAttributeType::Float),
            ("tex_coord".into(), 2, VertexAttributeType::Float),
            ("color".into(), 4, VertexAttributeType::Float),
        ]
    }
}

pub struct GameRenderer {
    program: Program<TexturedVertex>,
    vertex_buffer: VertexBuffer,
    vertices: Vec<TexturedVertex>,
    atlas: TextureAtlas,
    texture: Texture,
}

impl GameRenderer {
    pub fn new(renderer: &Renderer) -> Result<GameRenderer, Error> {
        let mut program = renderer.create_program(VERTEX_SHADER, FRAGMENT_SHADER)?;

        let texture_size = (4096, 4096);

        let texture = renderer.create_texture(texture_size, Some(TextureFiltering::Linear))?;

        let screen_size = renderer.screen_size();
        program.set_uniform(
            "screen_size",
            Uniform::Vec2((screen_size.0 as f32, screen_size.1 as f32)),
        );
        program.set_uniform(
            "texture_size",
            Uniform::Vec2((texture_size.0 as f32, texture_size.1 as f32)),
        );
        program.set_uniform("texture", Uniform::Texture(texture.clone()));

        Ok(GameRenderer {
            program: program,
            vertex_buffer: renderer.create_vertex_buffer()?,
            vertices: Vec::new(),
            atlas: TextureAtlas::new(texture_size),
            texture: texture,
        })
    }

    pub fn draw_texture(
        &mut self,
        texture: &TextureImage,
        position: Vec2<f32>,
        scale: f32,
        rotation: f32,
    ) -> Result<(), Error> {
        let tex_region = match self.atlas.get_texture_block(texture) {
            Some(region) => region,
            None => {
                let region = self.atlas.add_texture(texture)?;
                self.texture
                    .set_region(texture.image(), (region[0], region[1]));
                region
            }
        };
        let size = (tex_region[2] - tex_region[0], tex_region[3] - tex_region[1]);

        let rect = (
            size.0 as f32 / -2.0 * scale,
            size.1 as f32 / -2.0 * scale,
            size.0 as f32 / 2.0 * scale,
            size.1 as f32 / 2.0 * scale,
        );

        let rotate = |(x, y), a: f32| (x * a.cos() - y * a.sin(), x * a.sin() + y * a.cos());
        let quad = [
            rotate((rect.0, rect.1), rotation),
            rotate((rect.0, rect.3), rotation),
            rotate((rect.2, rect.3), rotation),
            rotate((rect.2, rect.1), rotation),
        ];

        let ll = (position.x + quad[0].0, position.y + quad[0].1);
        let ul = (position.x + quad[1].0, position.y + quad[1].1);
        let ur = (position.x + quad[2].0, position.y + quad[2].1);
        let lr = (position.x + quad[3].0, position.y + quad[3].1);
        let verts = [
            (ll, (tex_region[0], tex_region[1])),
            (ul, (tex_region[0], tex_region[3])),
            (lr, (tex_region[2], tex_region[1])),
            (ul, (tex_region[0], tex_region[3])),
            (ur, (tex_region[2], tex_region[3])),
            (lr, (tex_region[2], tex_region[1])),
        ];
        for &(pos, tex_coord) in verts.iter() {
            self.vertices.push(TexturedVertex {
                position: pos,
                tex_coord: (tex_coord.0 as f32, tex_coord.1 as f32),
                color: (1.0, 1.0, 1.0, 1.0),
            })
        }

        Ok(())
    }

    pub fn do_render(&mut self, renderer: &Renderer) -> Result<(), Error> {
        renderer.clear(Some((0.0, 0.0, 0.0, 1.0)));

        renderer.render_vertices(&self.vertex_buffer, &self.program, &self.vertices)?;

        self.vertices.clear();

        Ok(())
    }
}

impl RenderInterface for GameRenderer {
    fn draw_texture(
        &mut self,
        texture: &TextureImage,
        position: Vec2<f32>,
        scale: f32,
        rotation: f32,
    ) -> Result<(), Error> {
        self.draw_texture(texture, position, scale, rotation)
    }
}
