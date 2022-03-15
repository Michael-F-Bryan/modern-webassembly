wit_bindgen_rust::import!("../fornjot-v1.wit");
wit_bindgen_rust::export!("../model-v1.wit");

use crate::{
    fornjot_v1::Context,
    model_v1::{Error, Metadata, Shape, Vertex},
};
use wit_bindgen_rust::Handle;

struct ModelV1;

impl model_v1::ModelV1 for ModelV1 {
    fn on_load() -> Metadata {
        Metadata {
            name: env!("CARGO_MANIFEST_DIR").into(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
            version: env!("CARGO_PKG_VERSION").into(),
        }
    }

    fn generate(ctx: Handle<Context>) -> Result<Shape, Error> {
        let width: f32 = ctx
            .get_argument("width")
            .ok_or("The \"width\" argument is missing")?
            .parse()
            .map_err(|e| format!("Unable to parse the width: {}", e))?;

        let depth: f32 = ctx
            .get_argument("depth")
            .ok_or("The \"depth\" argument is missing")?
            .parse()
            .map_err(|e| format!("Unable to parse the depth: {}", e))?;

        let vertices = vec![
            Vertex::new(0.0, 0.0, 0.0),
            Vertex::new(width, 0.0, 0.0),
            Vertex::new(width, width, 0.0),
            Vertex::new(0.0, width, 0.0),
            Vertex::new(0.0, width, depth),
            Vertex::new(width, width, depth),
            Vertex::new(0.0, 0.0, depth),
        ];

        let faces = vec![
            (0, 2, 1), // face front
            (0, 3, 2),
            (2, 3, 4), // face top
            (2, 4, 5),
            (1, 2, 5), // face right
            (1, 5, 6),
            (0, 7, 4), // face left
            (0, 4, 3),
            (5, 4, 7), // face back
            (5, 7, 6),
            (0, 6, 7), // face bottom
            (0, 1, 6),
        ];

        Ok(Shape { faces, vertices })
    }
}

impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Self { Vertex { x, y, z } }
}

impl<S: Into<String>> From<S> for Error {
    fn from(s: S) -> Self { Error { message: s.into() } }
}
