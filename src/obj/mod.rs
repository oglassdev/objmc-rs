pub mod model;
pub mod frame;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::BufRead;
use tracing::{info, warn};
use model::{Position, Vector, Face};
use frame::Frame;

#[derive(Debug, Clone)]
pub struct FramedObj {
    pub vertices: Vec<Position<f64>>,
    pub uvs: Vec<Vector<f32>>,
    pub frames: Vec<Frame>
}

impl Hash for Position<f64> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x_bits = self.x.to_bits();
        let y_bits = self.y.to_bits();
        let z_bits = self.z.to_bits();

        let hash = x_bits
            .wrapping_mul(0x9E3779B1)
            ^ y_bits.wrapping_mul(0x85EBCA77)
            ^ z_bits.wrapping_mul(0xC2B2AE3D);

        state.write_u64(hash);
    }
}

impl Hash for Vector<f32> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x_bits = self.x.to_bits();
        let y_bits = self.y.to_bits();

        let hash = x_bits
            .wrapping_mul(0x9E3779B1)
            ^ y_bits.wrapping_mul(0x85EBCA77);

        state.write_u32(hash);
    }
}

impl Eq for Position<f64> {}

impl Eq for Vector<f32> {}

impl FramedObj {
    pub fn read<T: BufRead>(sources: Vec<T>) -> Self {
        let mut vertices: Vec<Position<f64>> = Vec::new();
        let mut uvs: Vec<Vector<f32>> = Vec::new();
        let mut frames: Vec<Frame> = Vec::new();

        let mut vertex_map: HashMap<Position<f64>, usize> = HashMap::new();
        let mut uv_map: HashMap<Vector<f32>, usize> = HashMap::new();

        for source in sources {
            let mut local_vertices: Vec<usize> = Vec::new();
            let mut local_uvs: Vec<usize> = Vec::new();
            let mut faces: Vec<Face> = Vec::new();

            // Parse each line, deduping as we go
            for line in source.lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => continue
                };

                let mut parts = line.split_whitespace();

                if let Some(prefix) = parts.next() {
                    match prefix {
                        "v" => {
                            // Parse positions
                            let coords: Vec<f64> = parts
                                .filter_map(|p| p.parse::<f64>().ok())
                                .collect();

                            if coords.len() != 3 {
                                warn!("Vertex didn't have the appropriate values; Expect errors");
                                continue
                            }

                            let vertex = Position::new(coords[0], coords[1], coords[2]);

                            // Dedup positions
                            local_vertices.push(
                                *vertex_map.entry(vertex)
                                    .or_insert_with(|| {
                                        vertices.push(vertex);
                                        vertices.len() - 1
                                    })
                            );
                        }
                        "vt" => {
                            // Parse UVs
                            let coords: Vec<f32> = parts
                                .filter_map(|p| p.parse::<f32>().ok())
                                .collect();

                            if coords.len() < 2 {
                                warn!("UV didn't have the appropriate values; Expect errors");
                                continue
                            }

                            let uv = Vector::new(coords[0], coords[1]);

                            // Dedup UVs
                            local_uvs.push(
                                *uv_map.entry(uv)
                                    .or_insert_with(|| {
                                        uvs.push(uv);
                                        uvs.len() - 1
                                    })
                            );
                        }
                        "f" => {
                            // We ignore the third value, as we just use v & vt
                            let mut vertex_normals: Vec<(u32, u32)> = Vec::new();

                            for element in parts {
                                let indices: Vec<usize> = element
                                    .split('/')
                                    .map(|value| value.parse::<usize>().expect("Index doesn't conform to usize"))
                                    .collect();

                                // Use the deduped addresses
                                vertex_normals.push((
                                    local_vertices[indices[0] - 1] as u32,
                                    local_uvs[indices.get(1).copied().unwrap_or(1) - 1] as u32
                                ));
                            }
                            faces.push(Face { vertex_normals });
                        }
                        _ => {}
                    }
                }
            }

            frames.push(Frame { faces })
        }

        info!("Read {:?} frame(s) totaling {:?} vertices, and {:?} uvs", frames.len(), vertices.len(), uv_map.len());

        Self {
            vertices,
            uvs,
            frames,
        }
    }
}