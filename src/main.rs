/**
 * !Note: there is a bug in this code. Right now it only works if there is a thumbnails/images directory in the root directory
 * save_path variable in the make_thumbnail function needs to be fixed for that
*/
use glob::{glob_with, MatchOptions};
use image::imageops::FilterType;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::path::Path;

fn main() {
    // let options = MatchOptions::new();
    // let paths = glob_with("images/*.png", options).unwrap();
    // let files: Vec<_> = paths.map(|x| x.ok()).collect();

    // Method 2
    // let paths: Vec<Result<PathBuf, glob::GlobError>> = glob_with(
    //     "images/*.png", // the pattern to match
    //     glob::MatchOptions {
    //         case_sensitive: true,             // match case-sensitively
    //         require_literal_separator: false, // allow using wildcards to match path separators
    //         ..glob::MatchOptions::default()   // use the default values for the other options
    //     },
    // )
    // .unwrap()
    // .collect();

    // Method 3
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("images/*.png", options)
        .unwrap()
        .filter_map(|x| x.ok())
        .collect();

    // check if there were files found
    if files.is_empty() {
        println!("No files found");
        return;
    }

    // create a new thumbnail directory
    let thumbnail_dir = "thumbnails";
    // create_dir(thumbnail_dir);

    // println!("{:?}", file);

    // without threading
    // for file in files {
    //     make_thumbnail(file, thumbnail_dir, 300);
    // }

    // with threading

    let failed_images: Vec<_> = files
        .par_iter()
        .map(|image| make_thumbnail(image, thumbnail_dir, 300).map_err(|e| e))
        .filter_map(|x| x.err())
        .collect();

    if failed_images.is_empty() {
        println!("All thumbnails generated successfully");
    } else {
        failed_images
            .iter()
            .for_each(|failed_image| println!("{}", failed_image.to_string()));
    }

    println!(
        "{} thumbnails saved successfully",
        files.len() - failed_images.len()
    );

    fn make_thumbnail<PA: AsRef<Path>, PB: AsRef<Path>>(
        original: PA,
        thumbnail: PB,
        longest_edge: u32,
    ) -> Result<(), std::io::Error> {
        // where PA: AsRef<Path>,
        // PB: AsRef<Path>,
        {
            // open image
            let original_img = image::open(original.as_ref()).unwrap();
            // make save file path
            let save_path = thumbnail.as_ref().join(original.as_ref());
            // resize and save the image
            Ok(original_img
                .resize(longest_edge, longest_edge, FilterType::Nearest)
                .save(save_path)
                .unwrap())
        }
    }
}
