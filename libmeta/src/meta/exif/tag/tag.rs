// Exiftool Tag definitions are an invaluable source
// https://exiftool.org/TagNames/EXIF.html

use std::fmt::Display;

/// Tag uses the New Type pattern to provide some helper functions for Tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag(u16);

impl Tag {
    pub(crate) fn from(tag: u16) -> Self {
        Self(tag)
    }

    /// Return true if the tag isn't useful for display
    pub(crate) fn no_display(&self) -> bool {
        // match self {
        //     &EXIF_SUB_IFD_OFFSET => true,
        //     _ => false,
        // }
        false
    }
}

impl From<i32> for Tag {
    fn from(val: i32) -> Self {
        Self(val as u16)
    }
}

impl From<u16> for Tag {
    fn from(val: u16) -> Self {
        Self(val)
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                &IMAGE_WIDTH => "Image Width".to_owned(),
                &IMAGE_HEIGHT => "Image Height".to_owned(),
                &BITS_PER_SAMPLE => "Bits Per Sample".to_owned(),
                &COMPRESSION => "Compression".to_owned(),
                &PHOTOMETRIC_INTERPRETATION => "Photometric Interpretation".to_owned(),
                &IMAGE_DESCRIPTION => "Image Description".to_owned(),
                &MAKE => "Make".to_owned(),
                &MODEL => "Model".to_owned(),
                &ORIENTATION => "Orientation".to_owned(),
                &SAMPLES_PER_PIXEL => "Samples Per Pixel".to_owned(),
                &X_RESOLUTION => "X Resolution".to_owned(),
                &Y_RESOLUTION => "Y Resolution".to_owned(),
                &RESOLUTION_UNIT => "Resolution Unit".to_owned(),
                &SOFTWARE => "Software".to_owned(),
                &DATE_TIME => "Date Time".to_owned(),
                &WHITE_POINT => "White Point".to_owned(),
                &THUMBNAIL_OFFSET => "Thumbnail Offset".to_owned(),
                &THUMBNAIL_LENGTH => "Thumbnail Length".to_owned(),
                &PRIMARY_CHROMATICITIES => "Primary Chromaticities".to_owned(),
                &Y_CB_CR_COEFFICIENTS => "Y Cb Cr Coefficients".to_owned(),
                &Y_CB_CR_POSITIONING => "Y Cb Cr Positioning".to_owned(),
                &REFERENCE_BLACK_WHITE => "Reference Black White".to_owned(),
                &COPYRIGHT => "Copyright".to_owned(),
                &EXPOSURE_TIME => "Exposure Time".to_owned(),
                &F_NUMBER => "F Number".to_owned(),
                &EXIF_SUB_IFD_OFFSET => "Exif Offset".to_owned(),
                &EXPOSURE_PROGRAM => "Exposure Program".to_owned(),
                &GPS_SUB_IFD_OFFSET => "GPS Offset".to_owned(),
                &ISO_SPEED_RATINGS => "ISO Speed Ratings".to_owned(),
                &EXIF_VERSION => "Exif Version".to_owned(),
                &DATE_TIME_ORIGINAL => "Date Time Original".to_owned(),
                &DATE_TIME_DIGITIZED => "Date Time Digitized".to_owned(),
                &COMPONENT_CONFIGURATION => "Component Configuration".to_owned(),
                &COMPRESSED_BITS_PER_PIXEL => "Compressed Bits Per Pixel".to_owned(),
                &SHUTTER_SPEED_VALUE => "Shutter Speed Value".to_owned(),
                &APEX_APERTURE_VALUE => "Apex Aperture Value".to_owned(),
                &BRIGHTNESS_VALUE => "Brightness Value".to_owned(),
                &EXPOSURE_BIAS_VALUE => "Exposure Bias Value".to_owned(),
                &MAX_APERTURE_VALUE => "Max Aperture Value".to_owned(),
                &SUBJECT_DISTANCE => "Subject Distance".to_owned(),
                &METERING_MODE => "Metering Mode".to_owned(),
                &LIGHT_SOURCE => "Light Source".to_owned(),
                &FLASH => "Flash".to_owned(),
                &FOCAL_LENGTH => "Focal Length".to_owned(),
                &MAKER_NOTE => "Maker Note".to_owned(),
                &USER_COMMENT => "User Comment".to_owned(),
                &XP_COMMENT => "XP Comment".to_owned(),
                &XP_AUTHOR => "XP Author".to_owned(),
                &XP_KEYWORDS => "XP Keywords".to_owned(),
                &XP_SUBJECT => "XP Subject".to_owned(),
                &FLASHPIX_VERSION => "FlashPix Version".to_owned(),
                &COLOR_SPACE => "Color Space".to_owned(),
                &EXIF_IMAGE_WIDTH => "Exif Image Width".to_owned(),
                &EXIF_IMAGE_HEIGHT => "Exif Image Height".to_owned(),
                &RELATED_SOUND_FILE => "Related Sound File".to_owned(),
                &EXIF_INTEROPERABILITY_OFFSET => "Exif Interoperability Offset".to_owned(),
                &FOCAL_PLANE_X_RESOLUTION => "Focal Plane X Resolution".to_owned(),
                &FOCAL_PLANE_Y_RESOLUTION => "Focal Plane Y Resolution".to_owned(),
                &FOCAL_PLANE_RESOLUTION_UNIT => "Focal Plane Resolution Unit".to_owned(),
                &SENSING_METHOD => "Sensing Method".to_owned(),
                &FILE_SOURCE => "File Source".to_owned(),
                &SCENE_TYPE => "Scene Type".to_owned(),
                &EXPOSURE_MODE => "Exposure Mode".to_owned(),
                &WHITE_BALANCE => "White Balance".to_owned(),
                &DIGITAL_ZOOM_RATIO => "Digital Zoom Ratio".to_owned(),
                &FOCAL_LENGTH_IN_35MM_FORMAT => "Focal Length In 35mm Format".to_owned(),
                &SCENE_CAPTURE_TYPE => "Scene Capture Type".to_owned(),
                &GAIN_CONTROL => "Gain Control".to_owned(),
                &CONTRAST => "Contrast".to_owned(),
                &SATURATION => "Saturation".to_owned(),
                &SHARPNESS => "Sharpness".to_owned(),
                &DEVICE_SETTING_DESCRIPTION => "Device Setting Description".to_owned(),
                &SUBJECT_DISTANCE_RANGE => "Subject Distance Range".to_owned(),
                &IMAGE_UNIQUE_ID => "Image Unique ID".to_owned(),
                &OWNER_NAME => "Owner Name".to_owned(),
                &SERIAL_NUMBER => "Serial Number".to_owned(),
                &LENS_INFO => "Lens Info".to_owned(),
                &LENS_MAKE => "Lens Make".to_owned(),
                &LENS_MODEL => "Lens Model".to_owned(),
                &LENS_SERIAL_NUMBER => "Lens Serial Number".to_owned(),
                &TITLE => "Title".to_owned(),
                _ => format!("Unknown({:02x?})", self.0),
            }
        )
    }
}

/// Image width
pub(crate) const IMAGE_WIDTH: Tag = Tag(0x0100);

/// Image height
pub(crate) const IMAGE_HEIGHT: Tag = Tag(0x0101);

/// Number of bits per sample
pub(crate) const BITS_PER_SAMPLE: Tag = Tag(0x0102);

/// Compression scheme used on the image data.
/// * 1 = Uncompressed, 6 = JPEG compression (thumbnails or preview images)
pub(crate) const COMPRESSION: Tag = Tag(0x0103);

/// * 2 = RGB
pub(crate) const PHOTOMETRIC_INTERPRETATION: Tag = Tag(0x0106);

/// Describes the image
/// * **Format**: ASCII string
pub(crate) const IMAGE_DESCRIPTION: Tag = Tag(0x010E);

/// Shows manufacturer of digicam
/// * Format: ASCII string
pub(crate) const MAKE: Tag = Tag(0x010F);

/// Shows model number of digicam
/// * **Format**: ASCII string
pub(crate) const MODEL: Tag = Tag(0x0110);

///
pub(crate) const STRIP_OFFSETS: Tag = Tag(0x0111);

/// Shows orientation of the camera
/// * **1** => Horizontal,
/// * **2** => MirrorHorizontal,
/// * **3** => Rotate180,
/// * **4** => MirrorVertical,
/// * **5** => MirrorHorizontalAndRotate270CW,
/// * **6** => Rotate90CW,
/// * **7** => MirrorHorizontalAndRotate90CW,
/// * **8** => Rotate270CW,
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const ORIENTATION: Tag = Tag(0x0112);

/// The number of separate planes in this image.
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const SAMPLES_PER_PIXEL: Tag = Tag(0x0115);

/// Shows resolution of the X axis, often 72/1 i.e. 72 pixels per inch, but this really has
/// no meaning as computers don't use this value for display.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const X_RESOLUTION: Tag = Tag(0x011A);

/// Shows resolution of the Y axis, often 72/1 i.e. 72 pixels per inch, but this really has
/// no meaning as computers don't use this value for display.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const Y_RESOLUTION: Tag = Tag(0x011B);

/// Shows resolution unit
/// * 1 = no-unit, 2 = inch, 3 = centimeter
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const RESOLUTION_UNIT: Tag = Tag(0x0128);

/// Shows software version
/// * **Format**: ASCII string
/// * **Components**: variable e.g. 12
pub(crate) const SOFTWARE: Tag = Tag(0x0131);

/// Date and time of image was last modified.
/// * Data format is "YYYY:MM:DD HH:MM:SS"+0x00, total 20bytes.
/// * Usually has the same value of DateTimeOriginal(0x9003)
/// * **Format**: ASCII string
/// * **Components**: 20
pub(crate) const DATE_TIME: Tag = Tag(0x0132);

/// Defines chromaticity of white point of the image.
/// * If the image uses CIE Standard Illumination D65(known as international standard of 'daylight'), the values are '3127/10000,3290/10000'.
/// * **Format**: Unsigned rational
/// * **Components**: 2
pub(crate) const WHITE_POINT: Tag = Tag(0x013E);

/// Defines chromaticity of the primaries of the image.
/// * If the image uses CCIR Recommendation 709 primearies, values are '640/1000,330/1000,300/1000,600/1000,150/1000,0/1000'.
/// * **Format**: Unsigned rational
/// * **Components**: 6
pub(crate) const PRIMARY_CHROMATICITIES: Tag = Tag(0x013F);

/// Thumbnail offset
/// * Data format is ordinary JPEG starting from 0xFFD8 and ending by 0xFFD9
/// * Typically the recommended thumbnail size is 160x120 for Exif 2.1 or later
/// * **Format**: Unsigned long
/// * **Components**: 1
pub(crate) const THUMBNAIL_OFFSET: Tag = Tag(0x0201);

/// Thumbnail length in bytes
/// * Data format is ordinary JPEG starting from 0xFFD8 and ending by 0xFFD9
/// * Typically the recommended thumbnail size is 160x120 for Exif 2.1 or later
/// * **Format**: Unsigned long
/// * **Components**: 1
pub(crate) const THUMBNAIL_LENGTH: Tag = Tag(0x0202);

/// When image format is YCbCr, this value shows a constant to translate it to RGB format. In usual, values are '0.299/0.587/0.114'.
/// * **Format**: Unsigned rational
/// * **Components**: 3
pub(crate) const Y_CB_CR_COEFFICIENTS: Tag = Tag(0x0211);

/// When image format is YCbCr and uses 'Subsampling'(cropping of chroma data, all the digicam do that), defines the chroma sample point of subsampling pixel array. '1' means the center of pixel array, '2' means the datum point.
/// * **1** => Centered
/// * **2** => Co-sited
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const Y_CB_CR_POSITIONING: Tag = Tag(0x0213);

/// Shows reference value of black point/white point. In case of YCbCr format, first 2 show black/white of Y, next 2 are Cb, last 2 are Cr. In case of RGB format, first 2 show black/white of R, next 2 are G, last 2 are B.
/// * **Format**: Unsigned rational
/// * **Components**: 6
pub(crate) const REFERENCE_BLACK_WHITE: Tag = Tag(0x0214);

/// Show copyright information
/// * **Format**: ASCII string
/// * **Components**: ?
pub(crate) const COPYRIGHT: Tag = Tag(0x8298);

/// Exposure time (reciprocol of shutter speed).
/// * Unit is second.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const EXPOSURE_TIME: Tag = Tag(0x829A);

/// The actual F-number(F-stop) of lens when the image was taken.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const F_NUMBER: Tag = Tag(0x829D);

/// Shows Exif IFD offset. The value of this tag is the byte offset from the start of the TIFF header to the Exif IFD.
/// * **Format**: Unsigned long
/// * **Components**: 1
pub(crate) const EXIF_SUB_IFD_OFFSET: Tag = Tag(0x8769);

/// Exposure program that the camera used when image was taken.
/// * 0 = not defined, 1 = manual, 2 = normal program, 3 = aperture priority, 4 = shutter priority,
/// * 5 = creative program, 6 = action program, 7 = portrait mode, 8 = landscape mode
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const EXPOSURE_PROGRAM: Tag = Tag(0x8822);

/// Shows offset to GPS Info IFD
/// * **Format**: Unsigned long
/// * **Components**: 1
pub(crate) const GPS_SUB_IFD_OFFSET: Tag = Tag(0x8825);

/// CCD sensitivity equivalent to Ag-Hr film speedrate.
/// * **Format**: Unsigned short
/// * **Components**: 2
pub(crate) const ISO_SPEED_RATINGS: Tag = Tag(0x8827);

/// Exif version number.
/// * Stored as 4bytes of ASCII character (e.g. "0210")
/// * **Format**: Undefined but turns out to be ASCII string
/// * **Components**: 4
pub(crate) const EXIF_VERSION: Tag = Tag(0x9000);

/// Date/Time of original image taken.
/// * This value should not be modified by user program.
/// * **Format**: ASCII string
/// * **Components**: 20
pub(crate) const DATE_TIME_ORIGINAL: Tag = Tag(0x9003);

/// Date/Time of image digitized.
/// * Usually, it contains the same value of DateTimeOriginal(0x9003).
/// * **Format**: ASCII string
/// * **Components**: 20
pub(crate) const DATE_TIME_DIGITIZED: Tag = Tag(0x9004);

/// Unknown value
/// * Seems to always be 0x00,0x01,0x02,0x03
/// * **Format**: Unsigned long
/// * **Components**: 1
pub(crate) const COMPONENT_CONFIGURATION: Tag = Tag(0x9101);

/// The average compression ratio of JPEG.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const COMPRESSED_BITS_PER_PIXEL: Tag = Tag(0x9102);

/// Shutter speed.
/// * To convert this value to ordinary 'Shutter Speed'); calculate this value's power of 2, then reciprocal.
/// * For example, if value is '4', shutter speed is 1/(2^4)=1/16 second.
/// * **Format**: Signed rational
/// * **Components**: 1
pub(crate) const SHUTTER_SPEED_VALUE: Tag = Tag(0x9201);

/// The actual aperture value of lens when the image was taken.
/// * To convert this value to ordinary F-number(F-stop), calculate this value's power of root 2 (=1.4142).
/// * For example, if value is '5', F-number is 1.4142^5 = F5.6.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const APEX_APERTURE_VALUE: Tag = Tag(0x9202);

/// Brightness of taken subject, unit is EV.
/// * **Format**: Signed rational
/// * **Components**: 1
pub(crate) const BRIGHTNESS_VALUE: Tag = Tag(0x9203);

/// Exposure bias value of taking picture.
/// * Unit is EV.
/// * **Format**: Signed rational
/// * **Components**: 1
pub(crate) const EXPOSURE_BIAS_VALUE: Tag = Tag(0x9204);

/// Maximum aperture value of lens.
/// * You can convert to F-number by calculating power of root 2 (same process of ApertureValue(0x9202).
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const MAX_APERTURE_VALUE: Tag = Tag(0x9205);

/// Distance to focus point, unit is meter.
/// * **Format**: Signed rational
/// * **Components**: 1
pub(crate) const SUBJECT_DISTANCE: Tag = Tag(0x9206);

/// Exposure metering method.
/// * 1 = average, 2 = center weighted average, 3 = spot, 4 = multi-spot, 5 = multi-segment
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const METERING_MODE: Tag = Tag(0x9207);

/// Light source, actually this means white balance setting.
/// * 0 = auto, 1 = daylight, 2 = fluorescent, 3 = tungsten, 10 = flash
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const LIGHT_SOURCE: Tag = Tag(0x9208);

/// Flash status.
/// * 0 = no flash, 1 = flash used
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const FLASH: Tag = Tag(0x9209);

/// Focal length of lens used to take image. Unit is millimeter.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const FOCAL_LENGTH: Tag = Tag(0x920A);

/// Maker dependent internal data.
/// * Some of maker such as Olympus/Nikon/Sanyo etc. uses IFD format for this area.
/// * **Format**: Undefined
pub(crate) const MAKER_NOTE: Tag = Tag(0x927C);

/// Stores user comments
/// * **Format**: ASCII string
/// * **Components**: ?
pub(crate) const USER_COMMENT: Tag = Tag(0x9286);

/// Stores the XP comment
/// * **Format**: ASCII string
/// * **Components**: variable
pub(crate) const XP_COMMENT: Tag = Tag(0x9C9C);

/// Stores the XP author
/// * **Format**: ASCII string
/// * **Components**: variable
pub(crate) const XP_AUTHOR: Tag = Tag(0x9C9D);

/// Stores the XP keywords
/// * **Format**: ASCII string
/// * **Components**: variable
pub(crate) const XP_KEYWORDS: Tag = Tag(0x9C9E);

/// Stores the XP subject
/// * **Format**: ASCII string
/// * **Components**: variable
pub(crate) const XP_SUBJECT: Tag = Tag(0x9C9F);

/// Stores the FlashPix version
/// * **Format**: ASCII string
/// * **Components**: 4
pub(crate) const FLASHPIX_VERSION: Tag = Tag(0xA000);

/// Color space information
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const COLOR_SPACE: Tag = Tag(0xA001);

/// Width of main image.
/// * **Format**: Unsigned short/long
/// * **Components**: 1
pub(crate) const EXIF_IMAGE_WIDTH: Tag = Tag(0xA002);

/// Height of main image.
/// * **Format**: Unsigned short/long
/// * **Components**: 1
pub(crate) const EXIF_IMAGE_HEIGHT: Tag = Tag(0xA003);

/// If this digicam can record audio data with image, shows name of audio data.
/// * **Format**: ASCII string
/// * **Components**: ?
pub(crate) const RELATED_SOUND_FILE: Tag = Tag(0xA004);

/// Extension of "ExifR98", detail is unknown.
/// * This value is offset to IFD format data. Currently there are 2 directory entries, first one is Tag0x0001, value is "R98", next is Tag0x0002, value is "0100".
/// * **Format**: Unsigned long
/// * **Components**: 1
pub(crate) const EXIF_INTEROPERABILITY_OFFSET: Tag = Tag(0xA005);

/// CCD's pixel density.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const FOCAL_PLANE_X_RESOLUTION: Tag = Tag(0xA20E);

/// CCD's pixel density.
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const FOCAL_PLANE_Y_RESOLUTION: Tag = Tag(0xA20F);

/// Unit of FocalPlaneXResoluton/FocalPlaneYResolution.
/// * '1' means no-unit, '2' inch, '3' centimeter.
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const FOCAL_PLANE_RESOLUTION_UNIT: Tag = Tag(0xA210);

/// Show type of image sensor unit.
/// * '2' means 1 chip color area sensor, most of all digicam use this type.
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const SENSING_METHOD: Tag = Tag(0xA217);

/// ?
/// * **Format**: undefined
/// * **Components**: 1
pub(crate) const FILE_SOURCE: Tag = Tag(0xA300);

/// ?
/// * **Format**: undefined
/// * **Components**: 1
pub(crate) const SCENE_TYPE: Tag = Tag(0xA301);

/// Exposure Mode
/// * 0 = Auto, 1 = Manual, 2 = Auto bracket
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const EXPOSURE_MODE: Tag = Tag(0xA402);

/// White balance
/// * 0 = Auto, 1 = Manual
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const WHITE_BALANCE: Tag = Tag(0xA403);

/// Digital zoom ratio
/// * **Format**: Unsigned rational
/// * **Components**: 1
pub(crate) const DIGITAL_ZOOM_RATIO: Tag = Tag(0xA404);

/// Focal length in 35mm format
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const FOCAL_LENGTH_IN_35MM_FORMAT: Tag = Tag(0xA405);

/// Scene capture type
/// * 0 = Standard, 1 = Landscape, 2 = Portrait, 3 = Night, 4 = Other
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const SCENE_CAPTURE_TYPE: Tag = Tag(0xA406);

/// Gain control
/// * 0 = None, 1 = Low gain up, 2 = High gain up, 3 = Low gain down, 4 = High gain down
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const GAIN_CONTROL: Tag = Tag(0xA407);

/// Contrast
/// * 0 = Normal, 1 = Low, 2 = High
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const CONTRAST: Tag = Tag(0xA408);

/// Saturation
/// * 0 = Normal, 1 = Low, 2 = High
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const SATURATION: Tag = Tag(0xA409);

/// Sharpness
/// * 0 = Normal, 1 = Soft, 2 = Hard
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const SHARPNESS: Tag = Tag(0xA40A);

/// Device setting description
/// * **Format**: undefined
/// * **Components**: ?
pub(crate) const DEVICE_SETTING_DESCRIPTION: Tag = Tag(0xA40B);

/// Subject distance range
/// * 0 = Unknown, 1 = Macro, 2 = Close, 3 = Distant
/// * **Format**: Unsigned short
/// * **Components**: 1
pub(crate) const SUBJECT_DISTANCE_RANGE: Tag = Tag(0xA40C);

/// Unique image ID
/// * **Format**: ASCII string
/// * **Components**: variable
pub(crate) const IMAGE_UNIQUE_ID: Tag = Tag(0xA420);

/// Camera owner name
/// * **Format**: ASCII string
/// * **Components**: variable
pub(crate) const OWNER_NAME: Tag = Tag(0xA430);

/// Serial number
/// * **Format**: ASCII string
/// * **Components**: variable
pub(crate) const SERIAL_NUMBER: Tag = Tag(0xA431);

/// Lens information
/// * **Format**: Unsigned rational
/// * **Components**: 4
pub(crate) const LENS_INFO: Tag = Tag(0xA432);

/// Lens make
/// * **Format**: ASCII string
/// * **Components**: variable
pub(crate) const LENS_MAKE: Tag = Tag(0xA433);

/// Lens model
/// * **Format**: ASCII string
/// * **Components**: variable
pub(crate) const LENS_MODEL: Tag = Tag(0xA434);

/// Lens serial number
/// * **Format**: ASCII string
/// * **Components**: variable
pub(crate) const LENS_SERIAL_NUMBER: Tag = Tag(0xA435);

/// Title
/// * **Format**: ASCII string
/// * **Components**: variable
pub(crate) const TITLE: Tag = Tag(0xA436);
