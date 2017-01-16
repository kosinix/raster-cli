# Raster CLI

Command line program for [Raster](https://github.com/kosinix/raster)

## Commands
### Blend
raster-cli blend <image1> <image2> [ <blendmode> <opacity> <pos> <offx> <offy> ]

* image1 - The base image
* image2 - The top image
* blendmode - normal, difference, multiply, overlay, screen
* opacity - 0.0 to 1.0
* pos - top_left, top_center, top_right, center_left, center, center_right, bottom_left, bottom_center, bottom_right
* offx - Negative or positive integer. Eg -100
* offy - Negative or positive integer. Eg. 20

### Crop
raster-cli crop <src> <dest> <width> <height> [ <pos> <offx> <offy> ]

### Resize
raster-cli resize <src> <dest> <width> <height> [ <resizemode> ]

### Rotate
raster-cli rotate <src> <dest> [--cc] <degrees> [ <bg> ]

### Help
raster-cli (-h | --help)

### Version
raster-cli (-v | --version)

## Installation

Download source from github and use cargo build

On the binary's directory run: raster-cli -h
