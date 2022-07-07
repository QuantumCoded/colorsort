use crate::error::Error;
use palette::{rgb::Srgb, FromColor, Hsv};

mod cli;
mod error;

fn main() -> main_error::MainResult {
    let path = cli::main()?;

    let mut images = path
        .read_dir()
        .map_err(|error| -> Error { error.into() })?
        .map(|entry| -> Result<_, Error> {
            let path = entry?.path();
            let image = image::open(&path)
                .map_err(|error| -> Error { Error::ImageError(path.clone(), error) })?
                .into_rgb8();

            Ok((
                image
                    .pixels()
                    .map(|pixel| {
                        Hsv::from_color(Srgb::new(
                            pixel.0[0] as f64 / u8::MAX as f64,
                            pixel.0[1] as f64 / u8::MAX as f64,
                            pixel.0[2] as f64 / u8::MAX as f64,
                        ))
                        .hue
                        .to_positive_degrees()
                            / 360f64
                    })
                    .sum::<f64>()
                    / image.pixels().len() as f64,
                path,
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;

    if images.len() == 0 {
        return Ok(());
    }

    images.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("unexpected NaN value"));

    images
        .iter()
        .map(|(_, path)| path)
        .enumerate()
        .map(|(idx, path)| -> Result<_, Error> {
            let mut name = std::ffi::OsString::from(format!(
                "{:0width$}",
                idx,
                width = (images.len() as f64).log10().floor() as usize + 1
            ));

            name.push(".");
            name.push(path.extension().expect("image should have extension"));

            std::fs::rename(path, name)?;

            Ok(())
        })
        .collect::<Result<(), _>>()?;

    Ok(())
}
