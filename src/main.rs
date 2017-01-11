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
  raster-cli crop <src> <dest> <width> <height> [ <pos> <offx> <offy> ]
  raster-cli resize <src> <dest> <width> <height> [ <resizemode> ]
  raster-cli rotate <src> <dest> [--cc] <degrees> [ <bg> ] [-j]
  raster-cli (-h | --help)
  raster-cli (-v | --version)

Options:
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
    arg_width: Option<u32>,
    arg_height: Option<u32>,
    arg_resizemode: Option<String>,
    arg_pos: Option<String>,
    arg_offx: Option<i32>,
    arg_offy: Option<i32>,
    arg_degrees: Option<i32>,
    arg_bg: Option<String>,
    flag_help: bool,
    flag_version: bool, // version info
    flag_cc: bool, // rotate counter clockwise
}

// TODO: Too much redundant code. needs abstraction
fn main() {
    
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    // println!("{:?}", args);
                
    if args.flag_version {
        println!("raster-cli v{}", VERSION);
        println!("raster v{}", RASTER);
    } else {
        // common among these apis
        let src = args.arg_src
            .unwrap_or_else(||{ 
                println!("Error <src>");
                process::exit(1);
            });

        let dest = args.arg_dest
            .unwrap_or_else(||{ 
                println!("Error <dest>");
                process::exit(1);
            });

        let mut image = raster::open(src.as_str())
            .unwrap_or_else(|_|{ 
                println!("Error opening <src>");
                process::exit(1);
            });

        // crop
        if args.cmd_crop {

            let width = args.arg_width
                .unwrap_or_else(||{ 
                    println!("Error <width>");
                    process::exit(1);
                });

            let height = args.arg_height
                .unwrap_or_else(||{ 
                    println!("Error <height>");
                    process::exit(1);
                });

            let pos = args.arg_pos
                .and_then(|p| { // If is_some(), check if its allowed
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
                .unwrap_or_else(||{raster::PositionMode::Center}); // None, defaults to center

            let offx = args.arg_offx.unwrap_or(0);

            let offy = args.arg_offy.unwrap_or(0);

            
            println!("Cropping...");
            if let Err(e)  = raster::editor::crop(&mut image, width as i32, height as i32, pos, offx, offy) {
                println!("Error crop {:?}", e);
                process::exit(1);
            } else {
                // TODO: redundundant!
                if let Err(e) = raster::save(&image, dest.as_str()) {
                    println!("Error saving {:?}", e);
                } else {
                    println!("Done.");
                }
            }
        // resize
        } else if args.cmd_resize {

            let width = args.arg_width
                .unwrap_or_else(||{ 
                    println!("Error <width>");
                    process::exit(1);
                });

            let height = args.arg_height
                .unwrap_or_else(||{ 
                    println!("Error <height>");
                    process::exit(1);
                });

            let resizemode = args.arg_resizemode
                .and_then(|p| { // If is_some(), check if its allowed
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
                .unwrap_or_else(||{raster::ResizeMode::Fit}); // None, default

            println!("Resizing...");
            if let Err(e)  = raster::editor::resize(&mut image, width as i32, height as i32, resizemode) {
                println!("Error resize {:?}", e);
                process::exit(1);
            } else {
                // TODO: redundundant!
                if let Err(e) = raster::save(&image, dest.as_str()) {
                    println!("Error saving {:?}", e);
                } else {
                    println!("Done.");
                }
            }
        // rotate
        } else if args.cmd_rotate {

            

            let mut degrees = args.arg_degrees
                .unwrap_or_else(||{ 
                    println!("Error <degrees>");
                    process::exit(1);
                });

            // counter-cc
            if args.flag_cc {
                degrees *= -1;
            }

            let bg = args.arg_bg.unwrap_or("#000000".to_string());
            let bg = raster::Color::hex(bg.as_str()).unwrap_or_else(|_|{ 
                println!("Error parsing <bg>");
                process::exit(1);
            });
            
            println!("Rotating...");
            if let Err(e)  = raster::transform::rotate(&mut image, degrees, bg) {
                println!("Error rotate {:?}", e);
                process::exit(1);
            } else {
                // TODO: redundundant!
                if let Err(e) = raster::save(&image, dest.as_str()) {
                    println!("Error saving {:?}", e);
                } else {
                    println!("Done.");
                }
            }
        }
    }

    

        //         raster::editor::crop(&mut image, crop_w, crop_h, pos, args.arg_offx.unwrap_or(0), args.arg_offy.unwrap_or(0))
        //     } else if args.cmd_rotate {
        //         println!("Rotating image ...");
        //         let mut degrees = args.arg_degrees.unwrap();
        //         let bg_color = args.arg_bg.unwrap_or("#000000".to_string());
        //         let bg_color = raster::Color::hex(bg_color.as_str()).unwrap();
        //         let mut image = raster::open(src.as_str()).unwrap();

        //         if args.flag_cc {
        //             degrees*=-1;
        //         }
        //         
        //     } else if args.flag_version {
        //         println!("raster-cli v{}", VERSION);
        //         println!("raster v{}", RASTER);

        //         Ok(())
        //     } else {
        //         Err(raster::error::RasterError::Unexpected)
        //     }
        // };

        // match result {
        //     Ok(_) => {
        //         if let Err(e) = raster::save(&image, dest.as_str()) {
        //             println!("Save failed: {:?}", e);
        //         } else {
        //             println!("Done!");
        //         }
        //     },
        //     Err(s) => {
        //         println!("Error: {:?}", s);
        //     }
        // }
}
