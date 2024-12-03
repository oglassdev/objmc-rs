# objmc-rs

> A *blazingly fast* 🚀 rewrite of objmc's CLI/generator.
> More info about the spec and shader can be found in the [original repository.](https://github.com/godlander/objmc)

---

## Current Features

objmc-rs is relatively new and isn't as fully featured as the original Python version. Below is a list of complete and incomplete features.

- [ ] Commands
  - [x] `convert` - takes an input model and texture and produces an output
  - [x] `join` - join multiple models together
  - [ ] `head` - creates a skull texture
- [ ] Generation
  - [x] Multiple models/textures
  - [x] Settings header
  - [x] Texture inversion
  - [x] Position/UV data generation
  - [x] Offset & scale
  - [ ] Compression

---

## Usage

Currently, no builds are provided, as it is in early development. As a temporary alternative, install Cargo and Rust, then build it yourself using `cargo build --release`

The final binary will end up in `./target/release`

---

### Convert Usage

Currently, this is the only implemented CLI command. This takes input OBJs and textures, then creates an output.

```bash
objmc convert [OPTIONS] --obj <OBJ> --texture <TEXTURE> <OUTPUT_MODEL> <OUTPUT_TEXTURE>
```

#### Arguments

| **Argument/Option**           | **Description**                                                           | **Default**                        |
|-------------------------------|---------------------------------------------------------------------------|------------------------------------|
| `--texture-resource <STRING>` | Resource reference in the model JSON. Defaults to the output texture.     | `None`                             |
| `-o, --obj <OBJ>`             | Input OBJ model file(s).                                                  | Required.                          |
| `-t, --texture <TEXTURE>`     | Input texture file(s).                                                    | Required.                          |
| `--offset <X Y Z>`            | Offset applied to the model.                                              | `None` (no offset).                |
| `--scale <X Y Z>`             | Scaling factors for the model in the x, y, and z directions.              | `None` (no scaling).               |
| `--duration <NUMBER>`         | Duration of the animation in ticks.                                       | `0`                                |
| `--fade-textures`             | Whether to interpolate between texture frames.                            | `true`                             |
| `--easing <EASING>`           | Easing function for animations (e.g., `linear`, `ease-in-out`, `bezier`). | `None`                             |
| `--colorbehavior <BEHAVIORS>` | Defines color overlay behaviors (e.g., `pitch`, `yaw`, `roll`).           | `["pitch", "yaw", "roll"]`         |
| `--autorotate-yaw`            | Attempt to estimate yaw rotation using normals.                           | `false`                            |
| `--autorotate-pitch`          | Attempt to estimate pitch rotation using normals.                         | `false`                            |
| `-c, --compress`              | Compress output.                                                          | `false`                            |
| `--no-shadow`                 | Disable shadows derived from face normals.                                | `false`                            |
| `--no-pow`                    | Disable enforcement of power-of-two textures.                             | `false`                            |
| `--flip-uv`                   | Invert the texture to adjust for flipped UV mapping.                      | `false`                            |
| `--autoplay`                  | Always interpolate animations, overriding `time`-based color behavior.    | `false`                            |
| `--visibility <VISIBILITY>`   | Determines model visibility (e.g., `gui`, `first-person`, `world`).       | `["gui", "first-person", "world"]` |

 For more info, run
```bash
 objmc convert -h
```

#### Examples

1. **Basic Conversion**
   ```bash
   objmc convert -o model.obj -t texture.png model.json texture_out.png
   ```

2. **Multiple Models/Textures**
   ```bash
   objmc convert -o model_0.obj -t texture_0.png -o model_1.obj -t model_1.png model.json texture_out.png
   ```