# Raster CLI

Command line program for [Raster](https://github.com/kosinix/raster)

## Commands
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

## Installation

Download source from github and use cargo build

On the binary's directory run: raster-cli -h
