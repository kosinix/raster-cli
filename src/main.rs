extern crate raster;
extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

const USAGE: &'static str = "
Raster CLI Commands.

Usage:
  raster-cli resize <src> <dest> [--rmode=<m>]
  raster-cli crop <src> <dest> <width> <height> [ --pos=<p> <offx> <offy> ]
  raster-cli rotate <src> <dest> <degrees>
  raster-cli (-h | --help)
  raster-cli (-v | --version)

Options:
    --rmode=<m>         Resize mode (fit|fill|exact|exact_width|exact_height) [default: fit].
    --pos=<p>           Position (top-left|top-center|top-right|center-left|center|center-right|bottom-left|bottom-center|bottom-right) [default: center].
    -h --help          Show this screen.
    -v --version       Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_resize: bool,
    arg_src: Option<String>,
    arg_dest: Option<String>,
    arg_width: Option<u32>,
    arg_height: Option<u32>,
    arg_offx: Option<i32>,
    arg_offy: Option<i32>,
    flag_pos: String,
    flag_rmode: String,
    flag_help: bool,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    

    if args.cmd_resize {
        println!("resizing... ");
        let mut image = raster::open(args.arg_src.unwrap().as_str()).unwrap();
        match raster::editor::resize(&mut image, 150, 150, args.flag_rmode.as_str()) {
            Ok(_) => {
                raster::save(&image, args.arg_dest.unwrap().as_str());
                println!("success");
            },
            Err(s) => {
                println!("error: {}", s);
            }
        }
        
    } else if args.flag_version {
        println!("raster-cli v0.1.0");
        println!("raster v0.0.7");
    }
}
