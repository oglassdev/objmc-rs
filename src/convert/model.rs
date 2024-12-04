use serde_json::{json, Value};
use crate::convert::config::ConvertConfig;

pub fn create_model_output(
    config: &ConvertConfig,
    height: u32
) -> Value {
    let mut elements = Vec::new();

    let (width, _) = config.texture_size;

    let height = if config.no_pow { height } else { height.next_power_of_two() } as f64;

    // TODO: Use proper height
    let width_fp = width as f64;

    for idx in 0..config.input.obj.frames[0].faces.len() {
        let x = (idx as u32 % width) as f64;

        let y = (idx as u32 / width + 1) as f64;

        elements.push(
            create_element(
                0,
                (x + 0.1) * 16.0 / width_fp,
                (y + 0.1) * 16.0 / height,
                (x + 0.9) * 16.0 / width_fp,
                (y + 0.9) * 16.0 / height
            )
        );
    }

    json!({
        "textures": {
            "0": config.texture_resource
        },
        "display":{
            "thirdperson_righthand": {
                "rotation": [85, 0, 0]
            },
            "thirdperson_lefthand": {
                "rotation": [85, 0, 0]
            }
        },
        "elements": elements
    })
}

fn create_element(texture: usize, x1: f64, y1: f64, x2: f64, y2: f64) -> Value {
    json!(
        {
            "from": [8, 0, 8],
            "to": [8.000001, 0.000001, 8.000001],
            "faces": {
                "north": {
                    "uv": [x1, y1, x2, y2],
                    "texture": format!("#{}", texture),
                    "tintindex": 0,
                }
            },
        }
    )
}