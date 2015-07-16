extern crate gfx;
extern crate gfx_device_gl;

#[macro_use]
extern crate backend;

backend!(
    Resources [gfx::Resources],
    Factory [gfx::Factory<Self::Resources>]
);

pub enum GlBackend {}

backend_impl!(
    GlBackend {
        Resources = gfx_device_gl::Resources,
        Factory = gfx_device_gl::Factory
    }
);

fn main() {

}
