use serde_json::{Map, Value};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn join_models(models: Vec<String>, output: &str) -> Result<(), Box<dyn Error>> {
    let files = models
        .iter()
        .map(|model| File::open(model))
        .collect::<Result<Vec<File>, _>>()?;

    let mut js: Value = serde_json::json!({
        "textures": {},
        "elements": [],
        "display": {
            "thirdperson_righthand": {"rotation": [85, 0, 0]},
            "thirdperson_lefthand": {"rotation": [85, 0, 0]},
        }
    });

    let mut texture_id = 0;
    let mut texture_map: HashMap<String, i32> = HashMap::new();

    for file in files {
        let reader = BufReader::new(file);
        let model: Value = serde_json::from_reader(reader)?;

        if let Value::Object(model_obj) = model {
            // Process textures
            if let Some(Value::Object(textures)) = model_obj.get("textures") {
                let js_textures = js["textures"]
                    .as_object_mut()
                    .expect("`textures` should be an object");

                for (key, value) in textures {
                    let tid_str = texture_id.to_string();
                    js_textures.insert(tid_str.clone(), value.clone());
                    texture_map.insert(key.clone(), texture_id);
                    texture_id += 1;
                }
            }

            // Process elements
            if let Some(Value::Array(elements)) = model_obj.get("elements") {
                let js_elements = js["elements"]
                    .as_array_mut()
                    .expect("`elements` should be an array");

                for mut element in elements.clone() {
                    if let Value::Object(faces) = element
                        .get_mut("faces")
                        .expect("Each element should have a `faces` object")
                    {
                        for face in faces.values_mut() {
                            if let Some(Value::String(texture)) = face.get_mut("texture") {
                                if let Some(tid) = texture_map.get(&texture[1..]) {
                                    *texture = format!("#{}", tid);
                                }
                            }
                        }
                    }
                    js_elements.push(element);
                }
            }
        }
    }

    let mut output_file = File::create(output)?;
    let js_string = serde_json::to_string(&js)?;
    output_file.write_all(js_string.as_bytes())?;

    Ok(())
}