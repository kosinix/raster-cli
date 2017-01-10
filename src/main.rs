extern crate raster;
extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

const VERSION: &'static str = "0.1.0";
const RASTER: &'static str = "0.2.0";
const USAGE: &'static str = "
Raster CLI Commands.

Usage:
  raster-cli resize <src> <dest> <width> <height> [ <mode> ]
  raster-cli crop <src> <dest> <width> <height> [ <pos> <offx> <offy> ]
  raster-cli rotate <src> <dest> [--cc] <degrees> [ <bg> ]
  raster-cli (-h | --help)
  raster-cli (-v | --version)

Options:
    -j --json          Return result as JSON.
    -h --help          Show this screen.
    -v --version       Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_resize: bool,
    cmd_crop: bool,
    cmd_rotate: bool,
    arg_src: Option<String>,
    arg_dest: Option<String>,
    arg_width: Option<i32>,
    arg_height: Option<i32>,
    arg_mode: Option<String>,
    arg_pos: Option<String>,
    arg_offx: Option<i32>,
    arg_offy: Option<i32>,
    arg_degrees: Option<i32>,
    arg_bg: Option<String>,
    flag_json: bool, // return json
    flag_help: bool,
    flag_version: bool, // version info
    flag_cc: bool, // rotate counter clockwise
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    // println!("{:?}", args);
    

    if args.cmd_resize {
        // TODO: sanity checks
        println!("Resizing image ... ");
        println!(" ");
        let src = args.arg_src.unwrap();
        let dest = args.arg_dest.unwrap();
        let resize_w = args.arg_width.unwrap();
        let resize_h = args.arg_height.unwrap();
        let mode = {
            match args.arg_mode.unwrap().as_str() {
                "fill" => raster::ResizeMode::Fill,
                "exact" => raster::ResizeMode::Exact,
                "exact_width" => raster::ResizeMode::ExactWidth,
                "exact_height" => raster::ResizeMode::ExactHeight,
                _ => raster::ResizeMode::Fit, 
            }
        };
        let mut image = raster::open(src.as_str()).unwrap();
        match raster::editor::resize(&mut image, resize_w, resize_h, mode) {
            Ok(_) => {
                raster::save(&image, dest.as_str()).unwrap();
                println!("Done!");
            },
            Err(s) => {
                println!("Error: {:?}", s);
            }
        }
    } else if args.cmd_crop {
        // TODO: sanity checks
        println!("Cropping image ... ");
        println!(" ");
        let src = args.arg_src.unwrap();
        let dest = args.arg_dest.unwrap();
        let crop_w = args.arg_width.unwrap();
        let crop_h = args.arg_height.unwrap();
        // optionals
        let pos = {
            match args.arg_pos.unwrap().as_str() {
                "top_left" => raster::PositionMode::TopLeft,
                "top_center" => raster::PositionMode::TopCenter,
                "top_right" => raster::PositionMode::TopRight,
                "center_left" => raster::PositionMode::CenterLeft,
                "center_right" => raster::PositionMode::CenterRight,
                "bottom_left" => raster::PositionMode::BottomLeft,
                "bottom_center" => raster::PositionMode::BottomCenter,
                "bottom_right" => raster::PositionMode::BottomRight,
                _ => raster::PositionMode::Center,
            }
        };
        let offx = args.arg_offx.unwrap_or(0);
        let offy = args.arg_offy.unwrap_or(0);

        let mut image = raster::open(src.as_str()).unwrap();
        match raster::editor::crop(&mut image, crop_w, crop_h, pos, offx, offy) {
            Ok(_) => {
                raster::save(&image, dest.as_str()).unwrap();
                println!("Done!");
            },
            Err(s) => {
                println!("Error: {:?}", s);
            }
        }
    } else if args.cmd_rotate {
        println!("Rotating image ... ");
        println!(" ");
        let src = args.arg_src.unwrap();
        let dest = args.arg_dest.unwrap();
        let mut degrees = args.arg_degrees.unwrap();
        let bg_color = args.arg_bg.unwrap_or("#000000".to_string());
        let bg_color = raster::Color::hex(bg_color.as_str()).unwrap();
        let mut image = raster::open(src.as_str()).unwrap();

        if args.flag_cc {
            degrees*=-1;
        }
        match raster::transform::rotate(&mut image, degrees, bg_color) {
            Ok(_) => {
                raster::save(&image, dest.as_str()).unwrap();
                println!("Done!");
            },
            Err(s) => {
                println!("Error: {:?}", s);
            }
        }
    } else if args.flag_version {
        println!("raster-cli v{}", VERSION);
        println!("raster v{}", RASTER);
    }
}
