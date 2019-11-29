use crate::resource;

use glium::*;
use glium::backend::Facade;
use glium::index::IndexBufferAny;
use glium::index::PrimitiveType::TriangleStrip;

pub struct ImageSystem {
    pub program: program::Program,
    pub vertex_buffer: vertex::VertexBufferAny,
    pub index_buffer: index::IndexBufferAny,
}

impl ImageSystem {
    pub fn new<F>(display: &F) -> Self where F: Facade{

        let vertex_buffer = {
            #[derive(Clone, Copy)]
            struct Vertex {
                position: [f32; 2],
                tex_coords: [f32; 2],
            }

            implement_vertex!(Vertex, position, tex_coords);

            VertexBuffer::new(
                display,
                &[
                    Vertex {
                        position: [-1.0, -1.0],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex {
                        position: [-1.0, 1.0],
                        tex_coords: [0.0, 1.0],
                    },
                    Vertex {
                        position: [1.0, 1.0],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex {
                        position: [1.0, -1.0],
                        tex_coords: [1.0, 0.0],
                    },
                ],
            )
            .unwrap()
        };

        let index_buffer = IndexBuffer::new(
            display,
            TriangleStrip,
            &[1 as u16, 2, 0, 3],
        )
        .unwrap();

        let program = program!(display,
        100 => {
            vertex: resource::SHADER_VERTEX_100_STR,
            fragment: resource::SHADER_FRAGMENT_100_STR,
        },
        140 => {
            vertex: resource::SHADER_VERTEX_140_STR,
            fragment: resource::SHADER_FRAGMENT_140_STR,
        })
        .unwrap();

        let vertex_buffer = vertex_buffer.into_vertex_buffer_any();
        let index_buffer = IndexBufferAny::from(index_buffer);

        ImageSystem {
            program: program,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
        }
    }
}