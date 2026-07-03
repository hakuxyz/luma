use crate::backend::DisplayBackend;
use std::process::Command;
use std::fs::File;
use std::io::Write as _;

pub struct WaylandBackend;

impl DisplayBackend for WaylandBackend {
    fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(WaylandBackend)
    }

    fn set_gamma(&mut self, temp: f32, r: f32, g: f32, b: f32) -> Result<(), Box<dyn std::error::Error>> {
        // 1. KDE Plasma Fallback
        let _ = Command::new("qdbus")
            .args(&["org.kde.KWin", "/org/kde/KWin/NightLight", "org.kde.KWin.NightLight", "setTemperature", &format!("{:.0}", temp)])
            .output();

        // 2. Modernes Hyprland GLSL 300 ES Shader Script
        let shader_path = "/tmp/luma_shader.glsl";
        let shader_code = format!(
            "#version 300 es\n\
             precision mediump float;\n\
             in vec2 v_texcoord;\n\
             layout(location = 0) out vec4 fragColor;\n\
             uniform sampler2D tex;\n\
             void main() {{\n\
                 vec4 pix = texture(tex, v_texcoord);\n\
                 pix.r *= {:.3};\n\
                 pix.g *= {:.3};\n\
                 pix.b *= {:.3};\n\
                 fragColor = pix;\n\
             }}", r, g, b
        );
        
        if let Ok(mut file) = File::create(shader_path) {
            let _ = file.write_all(shader_code.as_bytes());
        }

        // Shader live in Hyprland reinklinken
        let _ = Command::new("hyprctl")
            .env("HYPRLAND_INSTANCE_SIGNATURE", std::env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap_or_default())
            .args(&["keyword", "decoration:screen_shader", shader_path])
            .output();

        Ok(())
    }
}
