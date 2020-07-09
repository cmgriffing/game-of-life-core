extern crate image;
extern crate slug;

use log::*;
use slug::slugify;
use std::path::Path;

use crate::core::seeds::seeds::{get_seeds, Seed};
use image::ImageBuffer;

pub fn render_seeds() {
  let seeds = get_seeds();

  // println!("{:?}", seeds);

  seeds.iter().for_each(|seed| render_seed(seed.clone()));
}

fn render_seed(seed: Seed) {
  let live_pixel = image::Rgb([0, 0, 0]);
  let dead_pixel = image::Rgb([255, 255, 255]);

  let img = ImageBuffer::from_fn(500, 400, |x, y| {
    // let index = ((x / 10) * (y / 10)) as usize;
    let x = x / 10;
    let y = y / 10;
    let base = y * 50;
    let index = (base + x) as usize;

    let mut is_alive = false;
    if index < 2000 {
      is_alive = seed.cellules[index].alive();
    } else {
      println!("Index out of bounds {:?}", index);
    }

    if is_alive {
      if seed.label == "Line" {
        // println!("Printing pixel: {:?} at x: {:?} and y:{:?}", index, x, y);
      }
      live_pixel
    } else {
      dead_pixel
    }
  });

  let image_buffer = img.into_raw();

  let output_path = format!("./seed-images/{}.png", slugify(seed.label));
  image::save_buffer_with_format(
    Path::new(&output_path),
    &image_buffer,
    500,
    400,
    image::ColorType::Rgb8,
    image::ImageFormat::Png,
  )
  .unwrap();

  println!("{:?}", output_path);
}
