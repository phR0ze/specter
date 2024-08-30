use std::f32::consts::E;

/// Image width
pub(crate) const IMAGE_WIDTH: u16 = 0x0100;

/// Image height
pub(crate) const IMAGE_HEIGHT: u16 = 0x0101;

/// Number of bits per sample
pub(crate) const BITS_PER_SAMPLE: u16 = 0x0102;

/// Compression scheme used on the image data.
/// * 1 = Uncompressed, 6 = JPEG compression (thumbnails or preview images)
pub(crate) const COMPRESSION: u16 = 0x0103;

/// * 2 = RGB
pub(crate) const PHOTOMETRIC_INTERPRETATION: u16 = 0x0106;

/// Describes the image
/// * **Format**: ASCII string
pub(crate) const IMAGE_DESCRIPTION: u16 = 0x010E;

/// Shows manufacturer of digicam
/// * Format: ASCII string
pub(crate) const MAKE: u16 = 0x010F;

/// Shows model number of digicam
/// * **Format**: ASCII string
pub(crate) const MODEL: u16 = 0x0110;

pub(crate) const STRIP_OFFSETS: u16 = 0x0111;

/// Shows orientation of the camera
/// 1 = upper left, 3 = lower right, 6 = upper right, 8 = lower left, 9 = undefined
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const ORIENTATION: u16 = 0x0112;

/// Shows resolution of the X axis, often 1/72, but this really has no meaning as computers don't
/// use this value for display.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const X_RESOLUTION: u16 = 0x011A;

/// Shows resolution of the Y axis, often 1/72, but this really has no meaning as computers don't
/// use this value for display.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const Y_RESOLUTION: u16 = 0x011B;

/// Shows resolution unit
/// * 1 = no-unit, 2 = inch, 3 = centimeter
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const RESOLUTION_UNIT: u16 = 0x0128;

/// Shows software version
/// * **Format**: ASCII string
/// * **Components**: ?
pub(crate) const SOFTWARE: u16 = 0x0131;

/// Date and time of image was last modified.
/// * Data format is "YYYY:MM:DD HH:MM:SS"+0x00, total 20bytes.
/// * Usually has the same value of DateTimeOriginal(0x9003)
/// * **Format**: ASCII string
/// * **Components**: 20
pub(crate) const DATE_TIME: u16 = 0x0132;

/// Defines chromaticity of white point of the image.
/// * If the image uses CIE Standard Illumination D65(known as international standard of 'daylight'), the values are '3127/10000,3290/10000'.
/// * **Format**: Unsigned rational
/// * **Components**: 2
pub(crate) const WHITE_POINT: u16 = 0x013E;

/// Defines chromaticity of the primaries of the image.
/// * If the image uses CCIR Recommendation 709 primearies, values are '640/1000,330/1000,300/1000,600/1000,150/1000,0/1000'.
/// * **Format**: Unsigned rational
/// * **Components**: 6
pub(crate) const PRIMARY_CHROMATICITIES: u16 = 0x013F;

/// JPEG Thumbnail offset
/// * Data format is ordinary JPEG starting from 0xFFD8 and ending by 0xFFD9
/// * Typically the recommended thumbnail size is 160x120 for Exif 2.1 or later
/// * **Format**: Unsigned long
/// * **Components**: 1
pub(crate) const JPEG_THUMBNAIL_OFFSET: u16 = 0x0201;

/// JPEG Thumbnail length in bytes
/// * Data format is ordinary JPEG starting from 0xFFD8 and ending by 0xFFD9
/// * Typically the recommended thumbnail size is 160x120 for Exif 2.1 or later
/// * **Format**: Unsigned long
/// * **Components**: 1
pub(crate) const JPEG_THUMBNAIL_LENGTH: u16 = 0x0202;

/// When image format is YCbCr, this value shows a constant to translate it to RGB format. In usual, values are '0.299/0.587/0.114'.
/// * **Format**: Unsigned rational
/// * **Components**: 3
pub(crate) const Y_CB_CR_COEFFICIENTS: u16 = 0x0211;

/// When image format is YCbCr and uses 'Subsampling'(cropping of chroma data, all the digicam do that), defines the chroma sample point of subsampling pixel array. '1' means the center of pixel array, '2' means the datum point.
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const Y_CB_CR_POSITIONING: u16 = 0x0213;

/// Shows reference value of black point/white point. In case of YCbCr format, first 2 show black/white of Y, next 2 are Cb, last 2 are Cr. In case of RGB format, first 2 show black/white of R, next 2 are G, last 2 are B.
/// * **Format**: Unsigned rational
/// * **Components**: 6
pub(crate) const REFERENCE_BLACK_WHITE: u16 = 0x0214;

/// Show copyright information
/// * **Format**: ASCII string
/// * **Components**: ?
pub(crate) const COPYRIGHT: u16 = 0x8298;

/// Exposure time (reciprocol of shutter speed).
/// * Unit is second.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const EXPOSURE_TIME: u16 = 0x829A;

/// The actual F-number(F-stop) of lens when the image was taken.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const F_NUMBER: u16 = 0x829D;

/// Shows Exif IFD offset. The value of this tag is the byte offset from the start of the TIFF header to the Exif IFD.
/// * **Format**: Unsigned long
/// * **Components**: 1
pub(crate) const EXIF_SUB_IFD_OFFSET: u16 = 0x8769;

/// Exposure program that the camera used when image was taken.
/// * 0 = not defined, 1 = manual, 2 = normal program, 3 = aperture priority, 4 = shutter priority,
/// * 5 = creative program, 6 = action program, 7 = portrait mode, 8 = landscape mode
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const EXPOSURE_PROGRAM: u16 = 0x8822;

/// CCD sensitivity equivalent to Ag-Hr film speedrate.
/// * **Format**: Unsigned short
/// * **Components**: 2
pub(crate) const ISO_SPEED_RATINGS: u16 = 0x8827;

/// Exif version number.
/// * Stored as 4bytes of ASCII character (like "0210")
/// * **Format**: ASCII string
/// * **Components**: 4
pub(crate) const EXIF_VERSION: u16 = 0x9000;

/// Date/Time of original image taken.
/// * This value should not be modified by user program.
/// * **Format**: ASCII string
/// * **Components**: 20
pub(crate) const DATE_TIME_ORIGINAL: u16 = 0x9003;

/// Date/Time of image digitized.
/// * Usually, it contains the same value of DateTimeOriginal(0x9003).
/// * **Format**: ASCII string
/// * **Components**: 20
pub(crate) const DATE_TIME_DIGITIZED: u16 = 0x9004;

/// Unknown value
/// * Seems to always be 0x00,0x01,0x02,0x03
/// * **Format**: Unsigned long
/// * **Components**: 1
pub(crate) const COMPONENT_CONFIGURATION: u16 = 0x9101;

/// The average compression ratio of JPEG.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const COMPRESSED_BITS_PER_PIXEL: u16 = 0x9102;

/// Shutter speed.
/// * To convert this value to ordinary 'Shutter Speed'; calculate this value's power of 2, then reciprocal.
/// * For example, if value is '4', shutter speed is 1/(2^4)=1/16 second.
/// * **Format**: Signed rational
/// * **Components**: 1
pub(crate) const SHUTTER_SPEED_VALUE: u16 = 0x9201;

/// The actual aperture value of lens when the image was taken.
/// * To convert this value to ordinary F-number(F-stop), calculate this value's power of root 2 (=1.4142).
/// * For example, if value is '5', F-number is 1.4142^5 = F5.6.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const APEX_APERTURE_VALUE: u16 = 0x9202;

/// Brightness of taken subject, unit is EV.
/// * **Format**: Signed rational
/// * **Components**: 1
pub(crate) const BRIGHTNESS_VALUE: u16 = 0x9203;

/// Exposure bias value of taking picture.
/// * Unit is EV.
/// * **Format**: Signed rational
/// * **Components**: 1
pub(crate) const EXPOSURE_BIAS_VALUE: u16 = 0x9204;

/// Maximum aperture value of lens.
/// * You can convert to F-number by calculating power of root 2 (same process of ApertureValue(0x9202).
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const MAX_APERTURE_VALUE: u16 = 0x9205;

/// Distance to focus point, unit is meter.
/// * **Format**: Signed rational
/// * **Components**: 1
pub(crate) const SUBJECT_DISTANCE: u16 = 0x9206;

/// Exposure metering method.
/// * 1 = average, 2 = center weighted average, 3 = spot, 4 = multi-spot, 5 = multi-segment
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const METERING_MODE: u16 = 0x9207;

/// Light source, actually this means white balance setting.
/// * 0 = auto, 1 = daylight, 2 = fluorescent, 3 = tungsten, 10 = flash
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const LIGHT_SOURCE: u16 = 0x9208;

/// Flash status.
/// * 0 = no flash, 1 = flash used
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const FLASH: u16 = 0x9209;

/// Focal length of lens used to take image. Unit is millimeter.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const FOCAL_LENGTH: u16 = 0x920A;

/// Maker dependent internal data.
/// * Some of maker such as Olympus/Nikon/Sanyo etc. uses IFD format for this area.
/// * **Format**: Undefined
pub(crate) const MAKER_NOTE: u16 = 0x927C;

/// Stores user comments
/// * **Format**: ASCII string
/// * **Components**: ?
pub(crate) const USER_COMMENT: u16 = 0x9286;

/// Stores the FlashPix version
/// * **Format**: ASCII string
/// * **Components**: 4
pub(crate) const FLASHPIX_VERSION: u16 = 0xA000;

/// Color space information
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const COLOR_SPACE: u16 = 0xA001;

/// Width of main image.
/// * **Format**: Unsigned short/long
/// * **Components**: 1
pub(crate) const EXIF_IMAGE_WIDTH: u16 = 0xA002;

/// Height of main image.
/// * **Format**: Unsigned short/long
/// * **Components**: 1
pub(crate) const EXIF_IMAGE_HEIGHT: u16 = 0xA003;

/// If this digicam can record audio data with image, shows name of audio data.
/// * **Format**: ASCII string
/// * **Components**: ?
pub(crate) const RELATED_SOUND_FILE: u16 = 0xA004;

/// Extension of "ExifR98", detail is unknown.
/// * This value is offset to IFD format data. Currently there are 2 directory entries, first one is Tag0x0001, value is "R98", next is Tag0x0002, value is "0100".
/// * **Format**: Unsigned long
/// * **Components**: 1
pub(crate) const EXIF_INTEROPERABILITY_OFFSET: u16 = 0xA005;

/// CCD's pixel density.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const FOCAL_PLANE_X_RESOLUTION: u16 = 0xA20E;

/// CCD's pixel density.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const FOCAL_PLANE_Y_RESOLUTION: u16 = 0xA20F;

/// Unit of FocalPlaneXResoluton/FocalPlaneYResolution.
/// * '1' means no-unit, '2' inch, '3' centimeter.
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const FOCAL_PLANE_RESOLUTION_UNIT: u16 = 0xA210;

/// Show type of image sensor unit.
/// * '2' means 1 chip color area sensor, most of all digicam use this type.
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const SENSING_METHOD: u16 = 0xA217;

/// ?
/// * **Format**: undefined
/// * **Components**: 1
pub(crate) const FILE_SOURCE: u16 = 0xA300;

/// ?
/// * **Format**: undefined
/// * **Components**: 1
pub(crate) const SCENE_TYPE: u16 = 0xA301;

/// Converts a tag to human understandable descriptive string
pub(crate) fn to_string(tag: u16) -> &'static str {
    match tag {
        IMAGE_WIDTH => "Image Width",
        IMAGE_HEIGHT => "Image Height",
        BITS_PER_SAMPLE => "Bits Per Sample",
        COMPRESSION => "Compression",
        PHOTOMETRIC_INTERPRETATION => "Photometric Interpretation",
        IMAGE_DESCRIPTION => "Image Description",
        MAKE => "Make",
        MODEL => "Model",
        ORIENTATION => "Orientation",
        X_RESOLUTION => "X Resolution",
        Y_RESOLUTION => "Y Resolution",
        RESOLUTION_UNIT => "Resolution Unit",
        SOFTWARE => "Software",
        DATE_TIME => "Date Time",
        WHITE_POINT => "White Point",
        JPEG_THUMBNAIL_OFFSET => "JPEG Thumbnail Offset",
        JPEG_THUMBNAIL_LENGTH => "JPEG Thumbnail Length",
        PRIMARY_CHROMATICITIES => "Primary Chromaticities",
        Y_CB_CR_COEFFICIENTS => "Y Cb Cr Coefficients",
        Y_CB_CR_POSITIONING => "Y Cb Cr Positioning",
        REFERENCE_BLACK_WHITE => "Reference Black White",
        COPYRIGHT => "Copyright",
        EXPOSURE_TIME => "Exposure Time",
        F_NUMBER => "F Number",
        EXIF_SUB_IFD_OFFSET => "Exif Offset",
        EXPOSURE_PROGRAM => "Exposure Program",
        ISO_SPEED_RATINGS => "ISO Speed Ratings",
        EXIF_VERSION => "Exif Version",
        DATE_TIME_ORIGINAL => "Date Time Original",
        DATE_TIME_DIGITIZED => "Date Time Digitized",
        COMPONENT_CONFIGURATION => "Component Configuration",
        COMPRESSED_BITS_PER_PIXEL => "Compressed Bits Per Pixel",
        SHUTTER_SPEED_VALUE => "Shutter Speed Value",
        APEX_APERTURE_VALUE => "Apex Aperture Value",
        BRIGHTNESS_VALUE => "Brightness Value",
        EXPOSURE_BIAS_VALUE => "Exposure Bias Value",
        MAX_APERTURE_VALUE => "Max Aperture Value",
        SUBJECT_DISTANCE => "Subject Distance",
        METERING_MODE => "Metering Mode",
        LIGHT_SOURCE => "Light Source",
        FLASH => "Flash",
        FOCAL_LENGTH => "Focal Length",
        MAKER_NOTE => "Maker Note",
        USER_COMMENT => "User Comment",
        FLASHPIX_VERSION => "FlashPix Version",
        COLOR_SPACE => "Color Space",
        EXIF_IMAGE_WIDTH => "Exif Image Width",
        EXIF_IMAGE_HEIGHT => "Exif Image Height",
        RELATED_SOUND_FILE => "Related Sound File",
        EXIF_INTEROPERABILITY_OFFSET => "Exif Interoperability Offset",
        FOCAL_PLANE_X_RESOLUTION => "Focal Plane X Resolution",
        FOCAL_PLANE_Y_RESOLUTION => "Focal Plane Y Resolution",
        FOCAL_PLANE_RESOLUTION_UNIT => "Focal Plane Resolution Unit",
        SENSING_METHOD => "Sensing Method",
        FILE_SOURCE => "File Source",
        SCENE_TYPE => "Scene Type",
        _ => "Unknown",
    }
}
