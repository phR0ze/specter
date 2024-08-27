/// Describes the image
/// * **Format**: ASCII string
pub(crate) const IMAGE_DESCRIPTION: u16 = 0x010E;

/// Shows manufacturer of digicam
/// * Format: ASCII string
pub(crate) const MAKE: u16 = 0x010F;

/// Shows model number of digicam
/// * **Format**: ASCII string
pub(crate) const MODEL: u16 = 0x0110;

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

/// Shows Exif IFD offset. The value of this tag is the byte offset from the start of the TIFF header to the Exif IFD.
/// * **Format**: Unsigned long
/// * **Components**: 1
pub(crate) const EXIF_IFD_OFFSET: u16 = 0x8769;

/// Converts a tag to human understandable descriptive string
pub(crate) fn to_string(tag: u16) -> &'static str {
    match tag {
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
        PRIMARY_CHROMATICITIES => "Primary Chromaticities",
        Y_CB_CR_COEFFICIENTS => "Y Cb Cr Coefficients",
        Y_CB_CR_POSITIONING => "Y Cb Cr Positioning",
        REFERENCE_BLACK_WHITE => "Reference Black White",
        COPYRIGHT => "Copyright",
        EXIF_IFD_OFFSET => "Exif Offset",
        _ => "Unknown",
    }
}
