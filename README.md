# oscilloscope_graphics

A simple tool to control oscilliscope XY displays using audio signals.

## Building

In addition to a [Rust toolchain](https://rustup.rs), building on Linux
additionally requires the ALSA development files and `pkg-config`. See the
[CPAL repository](https://github.com/rustaudio/cpal) for more info.

## Setup and usage

In order to use this tool, you will need an oscilloscope with an XY mode (or
some other type of voltage-controlled XY display) and some way to connect the
scope to a computer's audio output. I used a short plug-plug 3.5mm audio cable
and some alligator clips. The left audio channel corresponds to the X axis, and
positive Y is considered down (you will probably need to invert the Y axis on
the scope). For a standard 3.5mm audio jack, the left (X) channel is on the tip,
right (Y) is in the middle, and ground is the furthest from the end. Use a high
output volume for a better signal to noise ratio (sharper image), 100% volume
should correspond to an about 4.5 V range. Using a volume higher than 100% will
likely cause clipping of the audio and image, which is fun to mess with but not
great for most cases. Adjusting the output volume is also good for fine image
size adjustment. The most important thing is to remember to *turn back down your
output volume* and not destroy your ears the next time you plug in your
headphones.

When run, this program repeatedly loads the `drawing.vgdl` file and displays the
output, along with any errors.

## VGDL

To describe the image to display on the scope, this project uses a tiny Vector
Graphics Description Language (VGDL). Whitespace is used exclusively as a
separator, the amount and type does not matter. A VGDL file consists of one of
the following commands, followed by its arguments:

- `draw`: Draw a series of lines. Expects a series of paths separated by `,` and
    terminated with `;`. Each path is a series of XY coordinates listed one
    after another.
- `sequence`: Run a sequence of commands (ending with a `.`) and
    combine their outputs.
- `row`: Similar to `sequence`, but offsets the output of each command in the
    sequence 2 units right of the previous one.
- `col`: Similar to `row`, but offsets 2 units down.
- `define`: Takes a name and a command to run. Binds the output of the run
    command to the given name so it can be reused.
- `load`: Takes a file path and runs the VGDL code in it. If given a path to a
    directory, loads and combines the output of its contents recursively.
- `move`: Takes an X offset, Y offset, and command. Applies the offsets to the
    command's output.
- `scale`: Takes an X scale, Y scale, and command. Applies the scale to the
    command's output.
- `text`: Takes a series of words terminated with `.` and generates their
    graphics. This acts similarly to the `row` command, except it takes each
    character in each word and runs it as a command.

## Included graphics

In addition to a starter `drawing.vgdl`, the `data` directory includes some
additional graphics. `data/calibration.vgdl` defines a `calibration` command
that draws marks in the corners of the screen and labels the positive X and Y
axis, as well as a simple `bouding_box` that outlines the screen.
`data/font.vgdl` contains a simple font, as well as a `font_demo` command that
displays all of its characters. These can be a good reference for what VGDL code
looks like.
