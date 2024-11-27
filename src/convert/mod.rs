mod texture;

use image::{ImageReader, Rgba, RgbaImage};

use crate::cli::Convert;
use std::{fs::File, io::BufWriter};
use std::error::Error;
use std::io::BufReader;
use serde_json::json;
use tracing::info;
use crate::convert::texture::settings::SettingsHeader;
use crate::obj::Obj;

pub fn convert(convert: &Convert) -> Result<(), Box<dyn Error>> {
    let obj_file = File::open(&convert.obj)?;

    let buf_reader = BufReader::new(obj_file);

    let image_reader = ImageReader::open(&convert.texture)?;

    let image = image_reader.decode()?;

    let obj = Obj::read(buf_reader).dedup();

    info!("Read OBJ: {:?} faces, {:?} verticies, {:?} normals", obj.faces.len(), obj.vertices.len(), obj.uvs.len());

    let mut buf = RgbaImage::new(image.width(), image.height());

    let settings_header = SettingsHeader::from_config(
        convert,
        &obj,
        [image.width() as u16, image.height() as u16]
    )?;

    settings_header.draw(&mut buf);

    texture::face_id_header::create_face_uv_id_header(&mut buf, obj.faces.len() as u32);

    // TODO: replace with actual output
    let new_file = File::create("test/output.png")?;

    let mut writer = BufWriter::new(new_file);

    buf.write_to(&mut writer, image::ImageFormat::Png)?;

    info!("Reading input...");

    /*
    for input in &convert.input {
        let obj_path = input.get(0).unwrap();
        let tex_path = input.get(1).unwrap();

        let obj_file = File::open(obj_path)?;
        let tex_file = File::open(tex_path).ok();

        let buf_reader = BufReader::new(obj_file);

        let obj = obj::read(buf_reader);

        for [x, y, z] in &obj.positions {

            elements.push(element);
        }
    }

     */

    // Write JSON

    Ok(())
}

fn get_header(out: &str, index: usize, x: usize, y: usize, ty: usize) -> String {
    "hello".to_string()
}