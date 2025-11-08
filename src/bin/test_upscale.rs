use std::path::PathBuf;
use std::time::Duration;
use aw_upscale::Upscaler;

fn main() {
    // Create an upscaler with default settings (using embedded Python script)
    let mut upscaler = Upscaler::new(None);
    
    // Set some test parameters
    upscaler
        .set_scale(2) // 2x upscaling
        .set_denoise(Some(1))
        .set_timeout(Some(Duration::from_secs(60)));

    // Test with an input image
    let source = PathBuf::from("input.png");
    let destination = PathBuf::from("output7.png");

    match upscaler.run(&source, &destination) {
        Ok((width, height)) => {
            println!("Successfully upscaled image to {}x{}", width, height);
        }
        Err(e) => {
            eprintln!("Failed to upscale image: {}", e);
            std::process::exit(1);
        }
    }
}