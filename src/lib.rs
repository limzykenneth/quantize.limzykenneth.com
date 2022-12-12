use worker::*;
use palette_extract::{
    get_palette_with_options,
    PixelEncoding,
    Quality,
    MaxColors,
    PixelFilter
};
use image::{load_from_memory, ColorType, DynamicImage};

mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/", |_, _| {
            let pixels: [u8; 12] = [255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0];
            let colors = get_palette_with_options(&pixels,
                PixelEncoding::Rgb,
                Quality::new(1),
                MaxColors::new(4),
                PixelFilter::White
            );

            Response::from_json(&colors)
        })
        .post_async("/", |mut req, _ctx| async move {
            let data = req.bytes().await?;
            let image = match load_from_memory(&data) {
                Ok(result) => match result.color() {
                    ColorType::Rgb8 => result,
                    _ => {
                        let rgb_image = result.to_rgb8();
                        DynamicImage::ImageRgb8(rgb_image)
                    }
                },
                Err(_) => return Response::error("Bad Request", 400)
            };

            let colors = get_palette_with_options(&image.as_bytes(),
                PixelEncoding::Rgb,
                Quality::new(10),
                MaxColors::new(10),
                PixelFilter::None
            );

            Response::from_json(&colors)
        })
        .run(req, env)
        .await
}
