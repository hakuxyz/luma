use crate::backend::DisplayBackend;
use x11rb::connection::Connection;
use x11rb::protocol::randr::{self, ConnectionExt as _};

pub struct X11Backend {
    conn: x11rb::rust_connection::RustConnection,
    root: u32,
}

impl DisplayBackend for X11Backend {
    fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        let (conn, screen_num) = x11rb::connect(None)?;
        let root = conn.setup().roots[screen_num].root;
        conn.randr_query_version(1, 4)?;
        Ok(X11Backend { conn, root })
    }

    fn set_gamma(&mut self, _temp: f32, r: f32, g: f32, b: f32) -> Result<(), Box<dyn std::error::Error>> {
        let resources = self.conn.randr_get_screen_resources_current(self.root)?.reply()?;
        for crtc in resources.crtcs {
            let size_reply = self.conn.randr_get_crtc_gamma_size(crtc)?.reply()?;
            let size = size_reply.size as usize;
            if size == 0 { continue; }
            let mut red_ramp = Vec::with_capacity(size);
            let mut green_ramp = Vec::with_capacity(size);
            let mut blue_ramp = Vec::with_capacity(size);
            for i in 0..size {
                let v = i as f32 / (size - 1) as f32;
                red_ramp.push(((v * r) * 65535.0) as u16);
                green_ramp.push(((v * g) * 65535.0) as u16);
                blue_ramp.push(((v * b) * 65535.0) as u16);
            }
            self.conn.randr_set_crtc_gamma(crtc, &red_ramp, &green_ramp, &blue_ramp)?;
        }
        self.conn.flush()?;
        Ok(())
    }
}
