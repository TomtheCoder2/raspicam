use crate::image::settings::{CameraSettings, ImageSettings};
use std::io::Error;
use std::process::{Command, Output};

// Valid options are:
//   -h [ --help ] [=arg(=1)] (=0)         Print this help message
//   --version [=arg(=1)] (=0)             Displays the build version number
//   --list-cameras [=arg(=1)] (=0)        Lists the available cameras attached to the system.
//   --camera arg (=0)                     Chooses the camera to use. To list the available indexes, use the
//                                         --list-cameras option.
//   -v [ --verbose ] [=arg(=2)] (=1)      Set verbosity level. Level 0 is no output, 1 is default, 2 is verbose.
//   -c [ --config ] [=arg(=config.txt)]   Read the options from a file. If no filename is specified, default to
//                                         config.txt. In case of duplicate options, the ones provided on the command line
//                                         will be used. Note that the config file must only contain the long form
//                                         options.
//   --info-text arg (=#%frame (%fps fps) exp %exp ag %ag dg %dg)
//                                         Sets the information string on the titlebar. Available values:
//                                         %frame (frame number)
//                                         %fps (framerate)
//                                         %exp (shutter speed)
//                                         %ag (analogue gain)
//                                         %dg (digital gain)
//                                         %rg (red colour gain)
//                                         %bg (blue colour gain)
//                                         %focus (focus FoM value)
//                                         %aelock (AE locked status)
//                                         %lp (lens position, if known)
//                                         %afstate (AF state, if supported)
//   --width arg (=0)                      Set the output image width (0 = use default value)
//   --height arg (=0)                     Set the output image height (0 = use default value)
//   -t [ --timeout ] arg (=5sec)          Time for which program runs. If no units are provided default to ms.
//   -o [ --output ] arg                   Set the output file name
//   --post-process-file arg               Set the file name for configuring the post-processing
//   --post-process-libs arg               Set a custom location for the post-processing library .so files
//   -n [ --nopreview ] [=arg(=1)] (=0)    Do not show a preview window
//   -p [ --preview ] arg (=0,0,0,0)       Set the preview window dimensions, given as x,y,width,height e.g. 0,0,640,480
//   -f [ --fullscreen ] [=arg(=1)] (=0)   Use a fullscreen preview window
//   --qt-preview [=arg(=1)] (=0)          Use Qt-based preview window (WARNING: causes heavy CPU load, fullscreen not
//                                         supported)
//   --hflip [=arg(=1)] (=0)               Request a horizontal flip transform
//   --vflip [=arg(=1)] (=0)               Request a vertical flip transform
//   --rotation arg (=0)                   Request an image rotation, 0 or 180
//   --roi arg (=0,0,0,0)                  Set region of interest (digital zoom) e.g. 0.25,0.25,0.5,0.5
//   --shutter arg (=0)                    Set a fixed shutter speed. If no units are provided default to us
//   --analoggain arg (=0)                 Set a fixed gain value (synonym for 'gain' option)
//   --gain arg                            Set a fixed gain value
//   --metering arg (=centre)              Set the metering mode (centre, spot, average, custom)
//   --exposure arg (=normal)              Set the exposure mode (normal, sport)
//   --ev arg (=0)                         Set the EV exposure compensation, where 0 = no change
//   --awb arg (=auto)                     Set the AWB mode (auto, incandescent, tungsten, fluorescent, indoor, daylight,
//                                         cloudy, custom)
//   --awbgains arg (=0,0)                 Set explict red and blue gains (disable the automatic AWB algorithm)
//   --flush [=arg(=1)] (=0)               Flush output data as soon as possible
//   --wrap arg (=0)                       When writing multiple output files, reset the counter when it reaches this
//                                         number
//   --brightness arg (=0)                 Adjust the brightness of the output images, in the range -1.0 to 1.0
//   --contrast arg (=1)                   Adjust the contrast of the output image, where 1.0 = normal contrast
//   --saturation arg (=1)                 Adjust the colour saturation of the output, where 1.0 = normal and 0.0 =
//                                         greyscale
//   --sharpness arg (=1)                  Adjust the sharpness of the output image, where 1.0 = normal sharpening
//   --framerate arg (=-1)                 Set the fixed framerate for preview and video modes
//   --denoise arg (=auto)                 Sets the Denoise operating mode: auto, off, cdn_off, cdn_fast, cdn_hq
//   --viewfinder-width arg (=0)           Width of viewfinder frames from the camera (distinct from the preview window
//                                         size
//   --viewfinder-height arg (=0)          Height of viewfinder frames from the camera (distinct from the preview window
//                                         size)
//   --tuning-file arg (=-)                Name of camera tuning file to use, omit this option for libcamera default
//                                         behaviour
//   --lores-width arg (=0)                Width of low resolution frames (use 0 to omit low resolution stream)
//   --lores-height arg (=0)               Height of low resolution frames (use 0 to omit low resolution stream)
//   --lores-par [=arg(=1)] (=0)           Preserve the pixel aspect ratio of the low res image (where possible) by
//                                         applying a different crop on the stream.
//   --mode arg                            Camera mode as W:H:bit-depth:packing, where packing is P (packed) or U
//                                         (unpacked)
//   --viewfinder-mode arg                 Camera mode for preview as W:H:bit-depth:packing, where packing is P (packed)
//                                         or U (unpacked)
//   --buffer-count arg (=0)               Number of in-flight requests (and buffers) configured for video, raw, and
//                                         still.
//   --viewfinder-buffer-count arg (=0)    Number of in-flight requests (and buffers) configured for preview window.
//   --no-raw [=arg(=1)] (=0)              Disable requesting of a RAW stream. Will override any manual mode reqest the
//                                         mode choice when setting framerate.
//   --autofocus-mode arg (=default)       Control to set the mode of the AF (autofocus) algorithm.(manual, auto,
//                                         continuous)
//   --autofocus-range arg (=normal)       Set the range of focus distances that is scanned.(normal, macro, full)
//   --autofocus-speed arg (=normal)       Control that determines whether the AF algorithm is to move the lens as quickly
//                                         as possible or more steadily.(normal, fast)
//   --autofocus-window arg (=0,0,0,0)     Sets AfMetering to  AfMeteringWindows an set region used, e.g.
//                                         0.25,0.25,0.5,0.5
//   --lens-position arg                   Set the lens to a particular focus position, expressed as a reciprocal distance
//                                         (0 moves the lens to infinity), or "default" for the hyperfocal distance
//   --hdr [=arg(=auto)] (=off)            Enable High Dynamic Range, where supported. Available values are "off", "auto",
//                                         "sensor" for sensor HDR (e.g. for Camera Module 3), "single-exp" for PiSP based
//                                         single exposure multiframe HDR
//   --metadata arg                        Save captured image metadata to a file or "-" for stdout
//   --metadata-format arg (=json)         Format to save the metadata in, either txt or json (requires --metadata)
//   --flicker-period arg (=0s)            Manual flicker correction period
//                                         Set to 10000us to cancel 50Hz flicker.
//                                         Set to 8333us to cancel 60Hz flicker.
//
//   -q [ --quality ] arg (=93)            Set the JPEG quality parameter
//   -x [ --exif ] arg                     Add these extra EXIF tags to the output file
//   --timelapse arg (=0ms)                Time interval between timelapse captures. If no units are provided default to
//                                         ms.
//   --framestart arg (=0)                 Initial frame counter value for timelapse captures
//   --datetime [=arg(=1)] (=0)            Use date format for output file names
//   --timestamp [=arg(=1)] (=0)           Use system timestamps for output file names
//   --restart arg (=0)                    Set JPEG restart interval
//   -k [ --keypress ] [=arg(=1)] (=0)     Perform capture when ENTER pressed
//   -s [ --signal ] [=arg(=1)] (=0)       Perform capture when signal received
//   --thumb arg (=320:240:70)             Set thumbnail parameters as width:height:quality, or none
//   -e [ --encoding ] arg (=jpg)          Set the desired output encoding, either jpg, png, rgb/rgb24, rgb48, bmp or
//                                         yuv420
//   -r [ --raw ] [=arg(=1)] (=0)          Also save raw file in DNG format
//   --latest arg                          Create a symbolic link with this name to most recent saved file
//   --immediate [=arg(=1)] (=0)           Perform first capture immediately, with no preview phase
//   --autofocus-on-capture [=arg(=1)] (=0)
//                                         Switch to AfModeAuto and trigger a scan just before capturing a still
//   --zsl [=arg(=1)] (=0)                 Switch to AfModeAuto and trigger a scan just before capturing a still

static TRIGGER_CAMERA: &str = "rpicam-still";
static CAMERA_SHUTTER_SPEED: &str = "-t";
static IMAGE_WIDTH: &str = "--width";
static IMAGE_HEIGHT: &str = "--height";
static CLICKED_IMAGE_PATH: &str = "-o";
static IMAGE_CONTRAST: &str = "--contrast";
static IMAGE_SHARPNESS: &str = "--sharpness";
static IMAGE_BRIGHTNESS: &str = "--brightness";
static IMAGE_SATURATION: &str = "--saturation";
static IMAGE_QUALITY: &str = "--quality";
// static IMAGE_ISO: &str = "-ISO";
// static IMAGE_ROTATION: &str = "-rot";
// static IMAGE_HORIZONTAL_FLIP: &str = "-hf";
// static IMAGE_VERTICAL_FLIP: &str = "-vf";

/// Click image from RaspberryPi's camera and store that image in the user defined path.
///
/// # Arguments
///
/// * `camera_settings` - Structure of camera settings
///
/// * `image_settings` - Structure of image settings
///
/// # Return
///
/// This function retuns the response in Result enum of std::process::Output and std::io::Error.
///
/// # Example
///
/// ```
/// use raspicam::image::camera_operations::click_image;
/// use raspicam::image::settings::{CameraSettings, ImageSettings};
///
/// let camera_settings: CameraSettings = CameraSettings::default();
/// let image_settings: ImageSettings = ImageSettings::default();
/// let result = click_image(camera_settings, image_settings);
/// assert!(result.is_err());//because we don't have camera right now!
/// ```
///
pub fn click_image(
    camera_settings: CameraSettings,
    image_settings: ImageSettings,
) -> Result<(Command, Output), Error> {
    Command::new("rm")
        .arg(camera_settings.output.clone())
        .output()?;
    let mut command = Command::new(TRIGGER_CAMERA);
    command.args(&[
        IMAGE_CONTRAST,
        &*camera_settings.contrast,
        IMAGE_SHARPNESS,
        &*camera_settings.sharpness,
        IMAGE_BRIGHTNESS,
        &*camera_settings.brightness,
        IMAGE_SATURATION,
        &*camera_settings.saturation,
        IMAGE_QUALITY,
        &*camera_settings.quality,
        CAMERA_SHUTTER_SPEED,
        &*camera_settings.timeout,
        // IMAGE_ISO,
        // camera_settings.iso,
        CLICKED_IMAGE_PATH,
        &camera_settings.output.clone(),
        IMAGE_WIDTH,
        &*image_settings.width,
        IMAGE_HEIGHT,
        &*image_settings.height,
        // IMAGE_ROTATION,
        // image_settings.rotation,
        // IMAGE_HORIZONTAL_FLIP,
        // image_settings.horizontal_flip,
        // IMAGE_VERTICAL_FLIP,
        // image_settings.vertical_flip,
        "--autofocus-mode=manual",
        "--lens-position=1",
    ]);
    // print the command
    // println!("{:?}", command);
    // execute the command
    match command.output() {
        Ok(output) => {
            if output.status.success() {
                // println!("Image clicked successfully");
                Ok((command, output))
            } else {
                println!("Error clicking image: {:?}", output);
                Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Error clicking image",
                ))
            }
        }
        Err(e) => {
            // println!("Error executing command: {:?}", e);
            Err(e)
        }
    }
}
