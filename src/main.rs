
use raylib::{prelude::*, texture::RaylibTexture2D};
use raylib::core::texture::{Image, RenderTexture2D};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

fn rand_float() -> f32 {
    let mut rng = ChaCha20Rng::from_entropy();
    rng.gen_range(0.0..1.0)
}

fn main() {
    let factor = 100;
    let screen_width = 16* factor;
    let screen_height = 9 * factor;
    let scalar = 0.3;
    let texture_width = (screen_width as f32 * scalar) as i32;
    let texture_height = (screen_height as f32 * scalar) as i32;
    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Game of Life")
        .build();

    rl.set_target_fps(60);

    let mut image = Image::gen_image_color(texture_width, texture_height, Color::BLACK);
    let mut pixels: Vec<u8> = Vec::new();

    let w = texture_width / 3;
    let h = texture_height/ 3;
    for dy in 0..h {
        for dx in 0..w {
            let v = (rand_float()*255.0) as u8;
            let x = dx  + texture_width / 2 - w / 2;
            let y = dy + texture_height / 2 - h / 2;
            image.draw_pixel(x, y, Color::new(v, v, v, 255));
        }
    }

    let imagedata = image.get_image_data().to_vec();
    imagedata.iter().for_each(|x| {
        pixels.push(x.r);
        pixels.push(x.g);
        pixels.push(x.b);
        pixels.push(x.a);
    });
    let mut state: [RenderTexture2D;2] = [rl.load_render_texture(&thread, texture_width as u32, texture_height as u32).unwrap(), rl.load_render_texture(&thread, texture_width as u32, texture_height as u32).unwrap()];
    state[0].set_texture_wrap(&thread, TextureWrap::TEXTURE_WRAP_REPEAT);
    state[0].set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
    state[0].update_texture(pixels.as_slice());

    state[1].set_texture_wrap(&thread, TextureWrap::TEXTURE_WRAP_REPEAT);
    state[1].set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);

    let mut shader = rl.load_shader(&thread, None, Some("smoothlife.fs")).unwrap();
    let resolution: raylib::math::Vector2 = raylib::math::Vector2::new(texture_width as f32, texture_height as f32);
    let resolution_location = shader.get_shader_location("resolution");
    shader.set_shader_value(resolution_location, resolution);

    let mut i = 0;
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let tx = state[i].texture().clone();
        let mut t = d.begin_texture_mode(&thread, &mut state[1- i]);
        t.clear_background(Color::WHITE);
        let mut s = t.begin_shader_mode(&shader);
        s.draw_texture(tx, 0, 0, Color::WHITE);
        drop(s);
        drop(t);

        i = 1 - i;
        d.clear_background(Color::BLACK);
        d.draw_texture_ex(state[i].texture(), raylib::math::Vector2::new(0.0, 0.0), 0.0, 1.0/scalar, Color::WHITE);
        drop(d);
    }
}
