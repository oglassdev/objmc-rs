use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::BufRead;

#[derive(Debug)]
pub struct Obj {
    pub vertices: Vec<Vertex>,
    pub uvs: Vec<Normal>,
    pub faces: Vec<Face>
}

#[derive(Debug)]
pub struct Face {
    pub vertex_normals: Vec<(u32, u32)>,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Normal {
    pub x: f32,
    pub y: f32
}

impl Hash for Vertex {
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

impl Hash for Normal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x_bits = self.x.to_bits();
        let y_bits = self.y.to_bits();

        let hash = x_bits
            .wrapping_mul(0x9E3779B1)
            ^ y_bits.wrapping_mul(0x85EBCA77);

        state.write_u32(hash);
    }
}

impl Eq for Vertex {}

impl Eq for Normal {}

impl Obj {
    pub fn read<T: BufRead>(reader: T) -> Obj {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut uvs: Vec<Normal> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        for line in reader.lines() {
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
                        if coords.len() == 3 {
                            vertices.push(
                                Vertex {
                                    x: coords[0],
                                    y: coords[1],
                                    z: coords[2]
                                }
                            );
                        }
                    }
                    "vt" => {
                        // Parse texture coordinates
                        let coords: Vec<f32> = parts
                            .filter_map(|p| p.parse::<f32>().ok())
                            .collect();
                        if coords.len() >= 2 {
                            uvs.push(
                                Normal {
                                    x: coords[0],
                                    y: coords[1]
                                }
                            );
                        }
                    }
                    "f" => {
                        let mut vertex_normals: Vec<(u32, u32)> = Vec::new();
                        for face in parts {
                            let indices: Vec<u32> = face
                                .split('/')
                                .map(|value| value.parse::<u32>().expect("Index doesn't conform to u32"))
                                .collect();

                            vertex_normals.push(
                                (indices[0] - 1, indices.get(1).copied().unwrap_or(1) - 1)
                            );
                        }
                        faces.push(Face { vertex_normals });
                    }
                    _ => {}
                }
            }
        }

        Obj {
            vertices,
            uvs,
            faces,
        }
    }

    pub fn dedup(mut self) -> Obj {
        let mut unique_vertices: Vec<Vertex> = Vec::new();
        let mut unique_uvs: Vec<Normal> = Vec::new();

        let mut vertex_map: HashMap<Vertex, usize> = HashMap::new();
        let mut uv_map: HashMap<Normal, usize> = HashMap::new();

        for vertex in self.vertices.clone() {
            vertex_map.entry(vertex)
                .or_insert_with(|| {
                    unique_vertices.push(vertex);
                    unique_vertices.len() - 1
                });
        }

        for uv in self.uvs.clone() {
            uv_map.entry(uv)
                .or_insert_with(|| {
                    unique_uvs.push(uv);
                    unique_uvs.len() - 1
                });
        }

        let mut new_faces: Vec<Face> = Vec::new();

        for face in self.faces {
            let new_vertex_normals: Vec<(u32, u32)> = face.vertex_normals.into_iter()
                .map(|(vertex_idx, uv_idx)| {
                    let new_vertex_idx = vertex_map[&self.vertices[vertex_idx as usize]] as u32;
                    let new_uv_idx = uv_map[&self.uvs[uv_idx as usize]] as u32;
                    (new_vertex_idx, new_uv_idx)
                })
                .collect();

            new_faces.push(Face { vertex_normals: new_vertex_normals });
        }

        Obj {
            vertices: unique_vertices,
            uvs: unique_uvs,
            faces: new_faces,
        }
    }
}