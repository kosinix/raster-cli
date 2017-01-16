extern crate raster;
extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use std::process;

const VERSION: &'static str = "0.1.0";
const RASTER: &'static str = "0.2.0";
const USAGE: &'static str = "
Raster CLI Commands.

Usage:
    raster-cli blend <image1> <image2> <dest> [ <blendmode> <opacity> <pos> <offx> <offy> --debug]
    raster-cli crop <src> <dest> <width> <height> [ <pos> <offx> <offy> --debug]
    raster-cli equal <image1> <image2> [--debug]
    raster-cli fill <src> <dest> <fillcolor> [--debug]
    raster-cli gamma <src> <dest> <gamma> [--debug]
    raster-cli resize <src> <dest> <width> <height> [ <resizemode> --debug]
    raster-cli rotate <src> <dest> [--cc] <degrees> [ <bg> --debug]
    raster-cli similar <image1> <image2> [--debug]
    raster-cli (-h | --help)
    raster-cli (-v | --version)

Options:
    -h --help          Show this screen.
    -v --version       Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_blend: bool,
    cmd_crop: bool,
    cmd_equal: bool,
    cmd_fill: bool,
    cmd_gamma: bool,
    cmd_resize: bool,
    cmd_rotate: bool,
    cmd_similar: bool,
    arg_src: Option<String>,
    arg_dest: Option<String>,
    arg_image1: Option<String>,
    arg_image2: Option<String>,
    arg_width: Option<i32>,
    arg_height: Option<i32>,
    arg_blendmode: Option<String>,
    arg_resizemode: Option<String>,
    arg_pos: Option<String>,
    arg_offx: Option<i32>,
    arg_offy: Option<i32>,
    arg_opacity: Option<f32>,
    arg_degrees: Option<i32>,
    arg_bg: Option<String>,
    arg_gamma: Option<f32>,
    arg_fillcolor: Option<String>,
    flag_help: bool,
    flag_version: bool, // version info
    flag_cc: bool, // rotate counter clockwise
    flag_debug: bool, // Print args if true
}

// TODO: Too much redundant code. needs abstraction
fn main() {
    
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    if args.flag_debug {
        println!("{:?}", args);
    }

    if args.flag_version {
        println!("raster-cli v{}", VERSION);
        println!("raster v{}", RASTER);

    } else if args.cmd_blend { // blend

        let image1 = image_open_unwrap_or_else("<image1>", args.arg_image1);
        let image2 = image_open_unwrap_or_else("<image2>", args.arg_image2);
        let dest = opt_unwrap_or_else_string("<dest>", args.arg_dest);
        let blendmode = get_blend_mode(args.arg_blendmode);
        let opacity = args.arg_opacity.unwrap_or(1.0);
        let pos = get_position(args.arg_pos);
        let offx = args.arg_offx.unwrap_or(0);
        let offy = args.arg_offy.unwrap_or(0);

        println!("Blending...");
        match raster::editor::blend(&image1, &image2, blendmode, opacity, pos, offx, offy) {
            Err(e) => {
                println!("Error blend {:?}", e);
                process::exit(1);
            },
            Ok(o) => {
                save(&o, dest.as_str());
            }
        }
    } else if args.cmd_crop { // crop

        let mut image = image_open_unwrap_or_else("<src>", args.arg_src);
        let dest = opt_unwrap_or_else_string("<dest>", args.arg_dest);
        let width = opt_unwrap_or_else_i32("<width>", args.arg_width);
        let height = opt_unwrap_or_else_i32("<height>", args.arg_height);
        let pos = get_position(args.arg_pos);
        let offx = args.arg_offx.unwrap_or(0);
        let offy = args.arg_offy.unwrap_or(0);

        println!("Cropping...");
        match raster::editor::crop(&mut image, width as i32, height as i32, pos, offx, offy) {
            Err(e) => {
                println!("Error crop {:?}", e);
                process::exit(1);
            },
            Ok(_) => {
                save(&image, dest.as_str());
            }
        }
    } else if args.cmd_equal { // equal
        let image1 = image_open_unwrap_or_else("<image1>", args.arg_image1);
        let image2 = image_open_unwrap_or_else("<image2>", args.arg_image2);
        if let Ok(r) = raster::compare::equal(&image1, &image2) {
            if r {
                println!("true");
            } else {
                println!("false");
            }
        } else {
            println!("Something went wrong.");
        }
    } else if args.cmd_fill { // fill
        let mut image = image_open_unwrap_or_else("<src>", args.arg_src);
        let dest = opt_unwrap_or_else_string("<dest>", args.arg_dest);
        
        let fillcolor = args.arg_fillcolor.unwrap_or("#000000".to_string());
        let fillcolor = raster::Color::hex(fillcolor.as_str()).unwrap_or_else(|_|{ 
            println!("Error parsing <fillcolor>");
            process::exit(1);
        });
        
        println!("Filling...");
        match raster::editor::fill(&mut image, fillcolor) {
            Err(e) => {
                println!("Error filling {:?}", e);
                process::exit(1);
            },
            Ok(_) => {
                save(&image, dest.as_str());
            }
        }

    } else if args.cmd_gamma { // gamma

        let mut src = image_open_unwrap_or_else("<src>", args.arg_src);
        let dest = opt_unwrap_or_else_string("<dest>", args.arg_dest);
        let gamma = opt_unwrap_or_else_f32("<dest>", args.arg_gamma);

        println!("Gamma...");
        match raster::filter::gamma(&mut src, gamma) {
            Err(e) => {
                println!("Error gamma {:?}", e);
                process::exit(1);
            },
            Ok(_) => {
                save(&src, dest.as_str());
            }
        }
    
    
    
    } else if args.cmd_resize { // resize

        let mut image = image_open_unwrap_or_else("<src>", args.arg_src);
        let dest = opt_unwrap_or_else_string("<dest>", args.arg_dest);
        let width = opt_unwrap_or_else_i32("<width>", args.arg_width);
        let height = opt_unwrap_or_else_i32("<height>", args.arg_height);
        let resizemode = get_resize_mode(args.arg_resizemode);

        println!("Resizing...");
        match raster::editor::resize(&mut image, width as i32, height as i32, resizemode) {
            Err(e) => {
                println!("Error resize {:?}", e);
                process::exit(1);
            },
            Ok(_) => {
                save(&image, dest.as_str());
            }
        }

    } else if args.cmd_rotate { // rotate

        let mut image = image_open_unwrap_or_else("<src>", args.arg_src);
        let dest = opt_unwrap_or_else_string("<dest>", args.arg_dest);
        let degrees = {
            let mut degrees = opt_unwrap_or_else_i32("<degrees>", args.arg_degrees);
            // counter-clockwise
            if args.flag_cc {
                degrees *= -1;
            }
            degrees
        };
        
        let bg = args.arg_bg.unwrap_or("#000000".to_string());
        let bg = raster::Color::hex(bg.as_str()).unwrap_or_else(|_|{ 
            println!("Error parsing <bg>");
            process::exit(1);
        });
        
        println!("Rotating...");
        match raster::transform::rotate(&mut image, degrees, bg) {
            Err(e) => {
                println!("Error rotate {:?}", e);
                process::exit(1);
            },
            Ok(_) => {
                save(&image, dest.as_str());
            }
        }
        
    } else if args.cmd_similar { // similar
        let image1 = image_open_unwrap_or_else("<image1>", args.arg_image1);
        let image2 = image_open_unwrap_or_else("<image2>", args.arg_image2);
        if let Ok(hamming_distance) = raster::compare::similar(&image1, &image2) {
            println!("Hamming distance: {}", hamming_distance);
        } else {
            println!("Something went wrong.");
        }
    }
}

fn opt_unwrap_or_else_string(param: &str, option: Option<String>) -> String {
    option.unwrap_or_else(||{ 
        println!("Error {:?}", param);
        process::exit(1);
    })
}

// fn opt_unwrap_or_else_u32(param: &str, option: Option<u32>) -> u32 {
//     option.unwrap_or_else(||{ 
//         println!("Error {:?}", param);
//         process::exit(1);
//     })
// }

fn opt_unwrap_or_else_i32(param: &str, option: Option<i32>) -> i32 {
    option.unwrap_or_else(||{ 
        println!("Error {:?}", param);
        process::exit(1);
    })
}

fn opt_unwrap_or_else_f32(param: &str, option: Option<f32>) -> f32 {
    option.unwrap_or_else(||{ 
        println!("Error {:?}", param);
        process::exit(1);
    })
}


fn image_open_unwrap_or_else(param: &str, option: Option<String>) -> raster::Image {
    let src: String = opt_unwrap_or_else_string(param, option);
    raster::open(src.as_str())
        .unwrap_or_else(|_|{ 
            println!("Error opening {:?}", param);
            process::exit(1);
        })
}

fn get_blend_mode(mode: Option<String>) -> raster::BlendMode {
    mode.and_then(|p| { // If is_some(), check if its allowed
            match &p[..] {
                "normal" => Some(raster::BlendMode::Normal),
                "difference" => Some(raster::BlendMode::Difference),
                "multiply" => Some(raster::BlendMode::Multiply),
                "overlay" => Some(raster::BlendMode::Overlay),
                "screen" => Some(raster::BlendMode::Screen),
                _ => {
                    println!("Error <blendmode>");
                    std::process::exit(1);
                }, 
            }
        })
        .unwrap_or_else(||{raster::BlendMode::Normal}) // Default
}

fn get_position(pos: Option<String>) -> raster::PositionMode {
    pos.and_then(|p| { // If is_some(), check if its allowed
            match &p[..] {
                "top_left" => Some(raster::PositionMode::TopLeft),
                "top_center" => Some(raster::PositionMode::TopCenter),
                "top_right" => Some(raster::PositionMode::TopRight),
                "center_left" => Some(raster::PositionMode::CenterLeft),
                "center" => Some(raster::PositionMode::Center),
                "center_right" => Some(raster::PositionMode::CenterRight),
                "bottom_left" => Some(raster::PositionMode::BottomLeft),
                "bottom_center" => Some(raster::PositionMode::BottomCenter),
                "bottom_right" => Some(raster::PositionMode::BottomRight),
                _ => {
                    println!("Error <pos>. Allowed are top_left, top_center, top_right, center_left, center_right, center, bottom_left, bottom_center, and bottom_right.");
                    std::process::exit(1);
                }, 
            }
        })
        .unwrap_or_else(||{raster::PositionMode::Center}) // Default
}

fn get_resize_mode(mode: Option<String>) -> raster::ResizeMode {
    mode.and_then(|p| { // If is_some(), check if its allowed
            match &p[..] {
                "fit" => Some(raster::ResizeMode::Fit),
                "fill" => Some(raster::ResizeMode::Fill),
                "exact" => Some(raster::ResizeMode::Exact),
                "exact_width" => Some(raster::ResizeMode::ExactWidth),
                "exact_height" => Some(raster::ResizeMode::ExactHeight),
                _ => {
                    println!("Error <resizemode>. Allowed are fit, fill, exact, exact_width, exact_height.");
                    std::process::exit(1);
                }, 
            }
        })
        .unwrap_or_else(||{raster::ResizeMode::Fit}) // Default
}

fn save(image: &raster::Image, dest: &str) {
    if let Err(e) = raster::save(&image, dest) {
        println!("Error saving {:?}", e);
    } else {
        println!("Done.");
    }
}