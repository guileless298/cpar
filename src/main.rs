use std::fs;
use std::path::PathBuf;
use clap::Parser;
use image::{GenericImageView, ImageReader, Pixel};
use image::imageops::FilterType;

#[derive(Parser)]
/// Crop Preserving Aspect Ratio
///
/// Crops artwork and restores it to the original aspect ratio
struct CPAR {
    /// Source file(s) to process
    #[clap(num_args = 1.., required = true)]
    source: Vec<PathBuf>,
    /// Output folder to place processed images within
    output: PathBuf,

    /// Threshold value to check whitespace in both axes.
    /// When a row/column drops below this threshold, identify it as part of the image edge
    #[clap(short, long, default_value_t = 250)]
    threshold: u8,
    /// Threshold value to check whitespace in the x-axis
    #[clap(long, alias = "xt", conflicts_with = "threshold")]
    x_threshold: Option<u8>,
    /// Threshold value to check whitespace in the y-axis
    #[clap(long, alias = "yt", conflicts_with = "threshold")]
    y_threshold: Option<u8>,

    /// Percentile to accept border in both axes.
    /// When X% of the rows/columns have crossed this threshold, crop image to this point
    #[clap(short, long, default_value_t = 95, value_parser = clap::value_parser!(u8).range(0..=100))]
    percentile: u8,
    /// Percentile to accept border in x-axis
    #[clap(long, alias = "xp", conflicts_with = "percentile", value_parser = clap::value_parser!(u8).range(0..=100))]
    x_percentile: Option<u8>,
    /// Percentile to accept border in x-axis
    #[clap(long, alias = "yp", conflicts_with = "percentile", value_parser = clap::value_parser!(u8).range(0..=100))]
    y_percentile: Option<u8>,

    /// Extra margin to crop beyond threshold in both axes
    #[clap(short, long, default_value_t = 0)]
    extra: u32,
    /// Extra margin to crop beyond threshold in x-axis
    #[clap(long, alias = "ex", conflicts_with = "extra")]
    x_extra: Option<u32>,
    /// Extra margin to crop beyond threshold in y-axis
    #[clap(long, alias = "ey", conflicts_with = "extra")]
    y_extra: Option<u32>,

    /// Blur image by sigma
    #[clap(short, long)]
    blur: Option<f32>,

    /// Downscale image by factor
    #[clap(short, long, default_value_t = 1.0)]
    downscale: f32
}

fn main() -> std::io::Result<()> {
    let args = CPAR::parse();

    // Set axis thresholds
    let x_threshold = args.x_threshold.unwrap_or(args.threshold);
    let y_threshold = args.y_threshold.unwrap_or(args.threshold);
    let x_percentile = 1.0 - args.x_percentile.unwrap_or(args.percentile) as f32 / 100.0;
    let y_percentile = 1.0 - args.y_percentile.unwrap_or(args.percentile) as f32 / 100.0;
    let x_extra = args.x_extra.unwrap_or(args.extra);
    let y_extra = args.y_extra.unwrap_or(args.extra);

    // Ensure destination folder exists
    fs::create_dir_all(&args.output)?;

    // Process images
    for path in args.source {
        let img = ImageReader::open(&path)?.decode().expect("failed to decode image");
        println!("Processing {}", path.file_name().unwrap().to_str().unwrap());

        let mut x_thresholds = Vec::new();
        let mut y_thresholds = Vec::new();

        // Check right edge of image
        for y in 0..img.height() {
            for x in (0..img.width()).rev() {
                if img.get_pixel(x, y).to_luma().0[0] < x_threshold {
                    x_thresholds.push(x);
                    break;
                }
            }
        }

        // Check bottom edge of image
        for x in 0..img.width() {
            for y in (0..img.height()).rev() {
                if img.get_pixel(x, y).to_luma().0[0] < y_threshold {
                    y_thresholds.push(y);
                    break;
                }
            }
        }

        // Safety!
        if x_thresholds.is_empty() || y_thresholds.is_empty() {
            panic!("Failed to detect sides of image");
        }

        // Determine percentile-based depth into image from sides to declare image edge
        x_thresholds.sort_unstable();
        y_thresholds.sort_unstable();
        let x_percentile = (x_percentile * (x_thresholds.len() - 1) as f32).floor() as usize;
        let y_percentile = (y_percentile * (y_thresholds.len() - 1) as f32).floor() as usize;
        let x_edge = (*x_thresholds.get(x_percentile).unwrap() - x_extra).max(0);
        let y_edge = (*y_thresholds.get(y_percentile).unwrap() - y_extra).max(0);

        // Determine new dimensions for image, such that it is downscaled, restoring aspect ratio
        let f_width = img.width() as f32;
        let f_height = img.height() as f32;
        let x_rel_size = x_edge as f32 / f_width;
        let y_rel_size = y_edge as f32 / f_height;
        let [new_x, new_y] = if x_rel_size < y_rel_size {
            [x_edge as f32, x_rel_size * f_height.floor()]
        } else {
            [y_rel_size * f_width, y_edge as f32]
        };

        // Perform image processing
        let cropped = img.crop_imm(0, 0, x_edge, y_edge);
        let blurred = if let Some(sigma) = args.blur {
            cropped.blur(sigma)
        } else {
            cropped
        };
        let scaled = blurred.resize_exact(
            (new_x / args.downscale).floor() as u32,
            (new_y / args.downscale).floor() as u32,
            FilterType::Gaussian
        );

        // Save image
        let filename = path.file_name().unwrap().to_str().unwrap();
        let dest = args.output.join(filename);
        scaled.save(&dest).expect("Failed to save output");
    }
    Ok(())
}
