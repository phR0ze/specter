// Exiftool Tag definitions are an invaluable source
// https://exiftool.org/TagNames/EXIF.html

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tag {
    /// Image width
    /// * **Format**: u32
    /// * **Components**: 1
    ImageWidth,

    /// Image height
    /// * **Format**: u32
    /// * **Components**: 1
    ImageHeight,

    /// Number of bits per sample
    /// * **Format**: u16
    /// * **Components**: 1
    BitsPerSample,

    /// Compression scheme used on the image data.
    /// * 1 = Uncompressed, 6 = JPEG compression (thumbnails or preview images)
    /// * **Format**: u16
    /// * **Components**: 1
    Compression,

    /// Photometric interpretation
    /// * 2 = RGB
    /// * **Format**: u16
    /// * **Components**: 1
    PhotometricInterpretation,

    /// Describes the image
    /// * **Format**: ASCII
    /// * **Components**: n
    ImageDescription,

    /// Shows manufacturer of digicam
    /// * **Format**: ASCII
    /// * **Components**: n
    Make,

    /// Shows model number of digicam
    /// * **Format**: ASCII
    /// * **Components**: n
    Model,

    /// Strip offsets
    /// * **Format**: u32
    /// * **Components**: 1
    StripOffsets,

    /// Shows orientation of the camera
    /// * **1** => Horizontal,
    /// * **2** => MirrorHorizontal,
    /// * **3** => Rotate180,
    /// * **4** => MirrorVertical,
    /// * **5** => MirrorHorizontalAndRotate270CW,
    /// * **6** => Rotate90CW,
    /// * **7** => MirrorHorizontalAndRotate90CW,
    /// * **8** => Rotate270CW,
    /// * **Format**: u16
    /// * **Components**: 1
    Orientation,

    /// The number of separate planes in this image.
    /// * **Format**: u16
    /// * **Components**: 1
    SamplesPerPixel,

    /// Shows resolution of the X axis, often 72/1 i.e. 72 pixels per inch, but this really has
    /// no meaning as computers don't use this value for display.
    /// * **Format**: Unsigned rational
    /// * **Components**: 1
    XResolution,

    /// Shows resolution of the Y axis, often 72/1 i.e. 72 pixels per inch, but this really has
    /// no meaning as computers don't use this value for display.
    /// * **Format**: Unsigned rational
    /// * **Components**: 1
    YResolution,

    /// Shows resolution unit
    /// * 1 = no-unit, 2 = inch, 3 = centimeter
    /// * **Format**: u16
    /// * **Components**: 1
    ResolutionUnit,

    /// Shows software version
    /// * **Format**: ASCII
    /// * **Components**: n e.g. 12
    Software,

    /// Date and time of image was last modified.
    /// * Data format is "YYYY:MM:DD HH:MM:SS"+0x00, total 20bytes.
    /// * Usually has the same value of DateTimeOriginal(0x9003)
    /// * **Format**: ASCII
    /// * **Components**: 20
    DateTime,

    /// Defines chromaticity of white point of the image.
    /// * If the image uses CIE Standard Illumination D65(known as international standard of 'daylight'), the values are '3127/10000,3290/10000'.
    /// * **Format**: Unsigned rational
    /// * **Components**: 2
    WhitePoint,

    /// Defines chromaticity of the primaries of the image.
    /// * If the image uses CCIR Recommendation 709 primearies, values are '640/1000,330/1000,300/1000,600/1000,150/1000,0/1000'.
    /// * **Format**: Unsigned rational
    /// * **Components**: 6
    PrimaryChromaticities,

    /// Thumbnail offset
    /// * Data format is ordinary JPEG starting from 0xFFD8 and ending by 0xFFD9
    /// * Typically the recommended thumbnail size is 160x120 for Exif 2.1 or later
    /// * **Format**: u32
    /// * **Components**: 1
    ThumbnailOffset,

    /// Thumbnail length in bytes
    /// * Data format is ordinary JPEG starting from 0xFFD8 and ending by 0xFFD9
    /// * Typically the recommended thumbnail size is 160x120 for Exif 2.1 or later
    /// * **Format**: u32
    /// * **Components**: 1
    ThumbnailLength,

    /// When image format is YCbCr, this value shows a constant to translate it to RGB format. In usual, values are '0.299/0.587/0.114'.
    /// * **Format**: Unsigned rational
    /// * **Components**: 3
    YCbCrCoefficients,

    /// When image format is YCbCr and uses 'Subsampling'(cropping of chroma data, all the digicam do that), defines the chroma sample point of subsampling pixel array. '1' means the center of pixel array, '2' means the datum point.
    /// * **1** => Centered
    /// * **2** => Co-sited
    /// * **Format**: u16
    /// * **Components**: 1
    YCbCrPositioning,

    /// Shows reference value of black point/white point. In case of YCbCr format, first 2 show black/white of Y, next 2 are Cb, last 2 are Cr. In case of RGB format, first 2 show black/white of R, next 2 are G, last 2 are B.
    /// * **Format**: Unsigned rational
    /// * **Components**: 6
    ReferenceBlackWhite,

    /// Show copyright information
    /// * **Format**: ASCII
    /// * **Components**: ?
    Copyright,

    /// Exposure time (reciprocol of shutter speed).
    /// * Unit is second.
    /// * **Format**: Unsigned rational
    /// * **Components**: 1
    ExposureTime,

    /// The actual F-number(F-stop) of lens when the image was taken.
    /// * **Format**: Unsigned rational
    /// * **Components**: 1
    FNumber,

    /// Shows Exif IFD offset. The value of this tag is the byte offset from the start of the TIFF header to the Exif IFD.
    /// * **Format**: u32
    /// * **Components**: 1
    ExifSubIfdOffset,

    /// Exposure program that the camera used when image was taken.
    /// * 0 = not defined, 1 = manual, 2 = normal program, 3 = aperture priority, 4 = shutter priority,
    /// * 5 = creative program, 6 = action program, 7 = portrait mode, 8 = landscape mode
    /// * **Format**: u16
    /// * **Components**: 1
    ExposureProgram,

    /// Shows offset to GPS Info IFD
    /// * **Format**: u32
    /// * **Components**: 1
    GpsSubIfdOffset,

    /// CCD sensitivity equivalent to Ag-Hr film speedrate.
    /// * **Format**: u16
    /// * **Components**: 2
    IsoSpeedRatings,

    /// Exif version number.
    /// * Stored as 4bytes of ASCII character (e.g. "0210")
    /// * **Format**: Undefined but turns out to be ASCII
    /// * **Components**: 4
    ExifVersion,

    /// Date/Time of original image taken.
    /// * This value should not be modified by user program.
    /// * **Format**: ASCII
    /// * **Components**: 20
    DateTimeOriginal,

    /// Date/Time of image digitized.
    /// * Usually, it contains the same value of DateTimeOriginal(0x9003).
    /// * **Format**: ASCII
    /// * **Components**: 20
    DateTimeDigitized,

    /// Unknown value
    /// * Seems to always be 0x00,0x01,0x02,0x03
    /// * **Format**: u32
    /// * **Components**: 1
    ComponentConfiguration,

    /// The average compression ratio of JPEG.
    /// * **Format**: Unsigned rational
    /// * **Components**: 1
    CompressedBitsPerPixel,

    /// Shutter speed.
    /// * To convert this value to ordinary 'Shutter Speed'); calculate this value's power of 2, then reciprocal.
    /// * For example, if value is '4', shutter speed is 1/(2^4)=1/16 second.
    /// * **Format**: Signed rational
    /// * **Components**: 1
    ShutterSpeedValue,

    /// The actual aperture value of lens when the image was taken.
    /// * To convert this value to ordinary F-number(F-stop), calculate this value's power of root 2 (=1.4142).
    /// * For example, if value is '5', F-number is 1.4142^5 = F5.6.
    /// * **Format**: Unsigned rational
    /// * **Components**: 1
    ApexApertureValue,

    /// Brightness of taken subject, unit is EV.
    /// * **Format**: Signed rational
    /// * **Components**: 1
    BrightnessValue,

    /// Exposure bias value of taking picture.
    /// * Unit is EV.
    /// * **Format**: Signed rational
    /// * **Components**: 1
    ExposureBiasValue,

    /// Maximum aperture value of lens.
    /// * You can convert to F-number by calculating power of root 2 (same process of ApertureValue(0x9202).
    /// * **Format**: Unsigned rational
    /// * **Components**: 1
    MaxApertureValue,

    /// Distance to focus point, unit is meter.
    /// * **Format**: Signed rational
    /// * **Components**: 1
    SubjectDistance,

    /// Exposure metering method.
    /// * 1 = average, 2 = center weighted average, 3 = spot, 4 = multi-spot, 5 = multi-segment
    /// * **Format**: u16
    /// * **Components**: 1
    MeteringMode,

    /// Light source, actually this means white balance setting.
    /// * 0 = auto, 1 = daylight, 2 = fluorescent, 3 = tungsten, 10 = flash
    /// * **Format**: u16
    /// * **Components**: 1
    LightSource,

    /// Flash status.
    /// * 0 = no flash, 1 = flash used
    /// * **Format**: u16
    /// * **Components**: 1
    Flash,

    /// Focal length of lens used to take image. Unit is millimeter.
    /// * **Format**: Unsigned rational
    /// * **Components**: 1
    FocalLength,

    /// Maker dependent internal data.
    /// * Some of maker such as Olympus/Nikon/Sanyo etc. uses IFD format for this area.
    /// * **Format**: Undefined
    MakerNote,

    /// Stores user comments
    /// * **Format**: ASCII
    /// * **Components**: ?
    UserComment,

    /// Stores the XP comment
    /// * **Format**: ASCII
    /// * **Components**: n
    XPComment,

    /// Stores the XP author
    /// * **Format**: ASCII
    /// * **Components**: n
    XPAuthor,

    /// Stores the XP keywords
    /// * **Format**: ASCII
    /// * **Components**: n
    XPKeywords,

    /// Stores the XP subject
    /// * **Format**: ASCII
    /// * **Components**: n
    XPSubject,

    /// Stores the FlashPix version
    /// * **Format**: ASCII
    /// * **Components**: 4
    FlashPixVersion,

    /// Color space information
    /// * **Format**: u16
    /// * **Components**: 1
    ColorSpace,

    /// Width of main image.
    /// * **Format**: u16
    /// * **Components**: 1
    ExifImageWidth,

    /// Height of main image.
    /// * **Format**: u16
    /// * **Components**: 1
    ExifImageHeight,

    /// If this digicam can record audio data with image, shows name of audio data.
    /// * **Format**: ASCII
    /// * **Components**: ?
    RelatedSoundFile,

    /// Extension of "ExifR98", detail is unknown.
    /// * This value is offset to IFD format data. Currently there are 2 directory entries, first one is Tag0x0001, value is "R98", next is Tag0x0002, value is "0100".
    /// * **Format**: u32
    /// * **Components**: 1
    ExifInteroperabilityOffset,

    /// CCD's pixel density.
    /// * **Format**: Unsigned rational
    /// * **Components**: 1
    FocalPlaneXResolution,

    /// CCD's pixel density.
    /// * **Format**: Unsigned rational
    /// * **Components**: 1
    FocalPlaneYResolution,

    /// Unit of FocalPlaneXResoluton/FocalPlaneYResolution.
    /// * '1' means no-unit, '2' inch, '3' centimeter.
    /// * **Format**: u16
    /// * **Components**: 1
    FocalPlaneResolutionUnit,

    /// Show type of image sensor unit.
    /// * '2' means 1 chip color area sensor, most of all digicam use this type.
    /// * **Format**: u16
    /// * **Components**: 1
    SensingMethod,

    /// ?
    /// * **Format**: undefined
    /// * **Components**: 1
    FileSource,

    /// ?
    /// * **Format**: undefined
    /// * **Components**: 1
    SceneType,

    /// Exposure Mode
    /// * 0 = Auto, 1 = Manual, 2 = Auto bracket
    /// * **Format**: u16
    /// * **Components**: 1
    ExposureMode,

    /// White balance
    /// * 0 = Auto, 1 = Manual
    /// * **Format**: u16
    /// * **Components**: 1
    WhiteBalance,

    /// Digital zoom ratio
    /// * **Format**: Unsigned rational
    /// * **Components**: 1
    DigitalZoomRatio,

    /// Focal length in 35mm format
    /// * **Format**: u16
    /// * **Components**: 1
    FocalLengthIn35mmFormat,

    /// Scene capture type
    /// * 0 = Standard, 1 = Landscape, 2 = Portrait, 3 = Night, 4 = Other
    /// * **Format**: u16
    /// * **Components**: 1
    SceneCaptureType,

    /// Gain control
    /// * 0 = None, 1 = Low gain up, 2 = High gain up, 3 = Low gain down, 4 = High gain down
    /// * **Format**: u16
    /// * **Components**: 1
    GainControl,

    /// Contrast
    /// * 0 = Normal, 1 = Low, 2 = High
    /// * **Format**: u16
    /// * **Components**: 1
    Contrast,

    /// Saturation
    /// * 0 = Normal, 1 = Low, 2 = High
    /// * **Format**: u16
    /// * **Components**: 1
    Saturation,

    /// Sharpness
    /// * 0 = Normal, 1 = Soft, 2 = Hard
    /// * **Format**: u16
    /// * **Components**: 1
    Sharpness,

    /// Device setting description
    /// * **Format**: undefined
    /// * **Components**: n
    DeviceSettingDescription,

    /// Subject distance range
    /// * 0 = Unknown, 1 = Macro, 2 = Close, 3 = Distant
    /// * **Format**: u16
    /// * **Components**: 1
    SubjectDistanceRange,

    /// Unique image ID
    /// * **Format**: ASCII
    /// * **Components**: n
    ImageUniqueID,

    /// Camera owner name
    /// * **Format**: ASCII
    /// * **Components**: n
    OwnerName,

    /// Serial number
    /// * **Format**: ASCII
    /// * **Components**: n
    SerialNumber,

    /// Lens specification
    /// * **Format**: Unsigned rational
    /// * **Components**: 4
    LensSpecification,

    /// Lens make
    /// * **Format**: ASCII
    /// * **Components**: n
    LensMake,

    /// Lens model
    /// * **Format**: ASCII
    /// * **Components**: n
    LensModel,

    /// Lens serial number
    /// * **Format**: ASCII
    /// * **Components**: n
    LensSerialNumber,

    /// Title
    /// * **Format**: ASCII
    /// * **Components**: n
    Title,

    /// Raw tag value for unknown tags
    Raw(u16),
}

impl From<i32> for Tag {
    fn from(val: i32) -> Self {
        Tag::from(val as u16)
    }
}

impl From<u16> for Tag {
    fn from(val: u16) -> Self {
        match val {
            0x0100 => Tag::ImageWidth,
            0x0101 => Tag::ImageHeight,
            0x0102 => Tag::BitsPerSample,
            0x0103 => Tag::Compression,
            0x0106 => Tag::PhotometricInterpretation,
            0x010E => Tag::ImageDescription,
            0x010F => Tag::Make,
            0x0110 => Tag::Model,
            0x0111 => Tag::StripOffsets,
            0x0112 => Tag::Orientation,
            0x0115 => Tag::SamplesPerPixel,
            0x011A => Tag::XResolution,
            0x011B => Tag::YResolution,
            0x0128 => Tag::ResolutionUnit,
            0x0131 => Tag::Software,
            0x0132 => Tag::DateTime,
            0x013E => Tag::WhitePoint,
            0x013F => Tag::PrimaryChromaticities,
            0x0201 => Tag::ThumbnailOffset,
            0x0202 => Tag::ThumbnailLength,
            0x0211 => Tag::YCbCrCoefficients,
            0x0213 => Tag::YCbCrPositioning,
            0x0214 => Tag::ReferenceBlackWhite,
            0x8298 => Tag::Copyright,
            0x829A => Tag::ExposureTime,
            0x829D => Tag::FNumber,
            0x8769 => Tag::ExifSubIfdOffset,
            0x8822 => Tag::ExposureProgram,
            0x8825 => Tag::GpsSubIfdOffset,
            0x8827 => Tag::IsoSpeedRatings,
            0x9000 => Tag::ExifVersion,
            0x9003 => Tag::DateTimeOriginal,
            0x9004 => Tag::DateTimeDigitized,
            0x9101 => Tag::ComponentConfiguration,
            0x9102 => Tag::CompressedBitsPerPixel,
            0x9201 => Tag::ShutterSpeedValue,
            0x9202 => Tag::ApexApertureValue,
            0x9203 => Tag::BrightnessValue,
            0x9204 => Tag::ExposureBiasValue,
            0x9205 => Tag::MaxApertureValue,
            0x9206 => Tag::SubjectDistance,
            0x9207 => Tag::MeteringMode,
            0x9208 => Tag::LightSource,
            0x9209 => Tag::Flash,
            0x920A => Tag::FocalLength,
            0x927C => Tag::MakerNote,
            0x9286 => Tag::UserComment,
            0x9288 => Tag::XPComment,
            0x9291 => Tag::XPAuthor,
            0x9292 => Tag::XPKeywords,
            0x9293 => Tag::XPSubject,
            0xA000 => Tag::FlashPixVersion,
            0xA001 => Tag::ColorSpace,
            0xA002 => Tag::ExifImageWidth,
            0xA003 => Tag::ExifImageHeight,
            0xA004 => Tag::RelatedSoundFile,
            0xA005 => Tag::ExifInteroperabilityOffset,
            0xA20E => Tag::FocalPlaneXResolution,
            0xA20F => Tag::FocalPlaneYResolution,
            0xA210 => Tag::FocalPlaneResolutionUnit,
            0xA217 => Tag::SensingMethod,
            0xA300 => Tag::FileSource,
            0xA301 => Tag::SceneType,
            0xA402 => Tag::ExposureMode,
            0xA403 => Tag::WhiteBalance,
            0xA404 => Tag::DigitalZoomRatio,
            0xA405 => Tag::FocalLengthIn35mmFormat,
            0xA406 => Tag::SceneCaptureType,
            0xA407 => Tag::GainControl,
            0xA408 => Tag::Contrast,
            0xA409 => Tag::Saturation,
            0xA40A => Tag::Sharpness,
            0xA40B => Tag::DeviceSettingDescription,
            0xA40C => Tag::SubjectDistanceRange,
            0xA420 => Tag::ImageUniqueID,
            0xA430 => Tag::OwnerName,
            0xA431 => Tag::SerialNumber,
            0xA432 => Tag::LensSpecification,
            0xA433 => Tag::LensMake,
            0xA434 => Tag::LensModel,
            0xA435 => Tag::LensSerialNumber,
            0xA436 => Tag::Title,
            _ => Tag::Raw(val),
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::ImageWidth => write!(f, "Image Width"),
            Tag::ImageHeight => write!(f, "Image Height"),
            Tag::BitsPerSample => write!(f, "Bits Per Sample"),
            Tag::Compression => write!(f, "Compression"),
            Tag::PhotometricInterpretation => write!(f, "Photometric Interpretation"),
            Tag::ImageDescription => write!(f, "Image Description"),
            Tag::Make => write!(f, "Make"),
            Tag::Model => write!(f, "Model"),
            Tag::StripOffsets => write!(f, "Strip Offsets"),
            Tag::Orientation => write!(f, "Orientation"),
            Tag::SamplesPerPixel => write!(f, "Samples Per Pixel"),
            Tag::XResolution => write!(f, "X Resolution"),
            Tag::YResolution => write!(f, "Y Resolution"),
            Tag::ResolutionUnit => write!(f, "Resolution Unit"),
            Tag::Software => write!(f, "Software"),
            Tag::DateTime => write!(f, "Date Time"),
            Tag::WhitePoint => write!(f, "White Point"),
            Tag::PrimaryChromaticities => write!(f, "Primary Chromaticities"),
            Tag::ThumbnailOffset => write!(f, "Thumbnail Offset"),
            Tag::ThumbnailLength => write!(f, "Thumbnail Length"),
            Tag::YCbCrCoefficients => write!(f, "Y Cb Cr Coefficients"),
            Tag::YCbCrPositioning => write!(f, "Y Cb Cr Positioning"),
            Tag::ReferenceBlackWhite => write!(f, "Reference Black White"),
            Tag::Copyright => write!(f, "Copyright"),
            Tag::ExposureTime => write!(f, "Exposure Time"),
            Tag::FNumber => write!(f, "F Number"),
            Tag::ExifSubIfdOffset => write!(f, "Exif Offset"),
            Tag::ExposureProgram => write!(f, "Exposure Program"),
            Tag::GpsSubIfdOffset => write!(f, "GPS Offset"),
            Tag::IsoSpeedRatings => write!(f, "ISO Speed Ratings"),
            Tag::ExifVersion => write!(f, "Exif Version"),
            Tag::DateTimeOriginal => write!(f, "Date Time Original"),
            Tag::DateTimeDigitized => write!(f, "Date Time Digitized"),
            Tag::ComponentConfiguration => write!(f, "Component Configuration"),
            Tag::CompressedBitsPerPixel => write!(f, "Compressed Bits Per Pixel"),
            Tag::ShutterSpeedValue => write!(f, "Shutter Speed Value"),
            Tag::ApexApertureValue => write!(f, "Apex Aperture Value"),
            Tag::BrightnessValue => write!(f, "Brightness Value"),
            Tag::ExposureBiasValue => write!(f, "Exposure Bias Value"),
            Tag::MaxApertureValue => write!(f, "Max Aperture Value"),
            Tag::SubjectDistance => write!(f, "Subject Distance"),
            Tag::MeteringMode => write!(f, "Metering Mode"),
            Tag::LightSource => write!(f, "Light Source"),
            Tag::Flash => write!(f, "Flash"),
            Tag::FocalLength => write!(f, "Focal Length"),
            Tag::MakerNote => write!(f, "Maker Note"),
            Tag::UserComment => write!(f, "User Comment"),
            Tag::XPComment => write!(f, "XP Comment"),
            Tag::XPAuthor => write!(f, "XP Author"),
            Tag::XPKeywords => write!(f, "XP Keywords"),
            Tag::XPSubject => write!(f, "XP Subject"),
            Tag::FlashPixVersion => write!(f, "FlashPix Version"),
            Tag::ColorSpace => write!(f, "Color Space"),
            Tag::ExifImageWidth => write!(f, "Exif Image Width"),
            Tag::ExifImageHeight => write!(f, "Exif Image Height"),
            Tag::RelatedSoundFile => write!(f, "Related Sound File"),
            Tag::ExifInteroperabilityOffset => write!(f, "Exif Interoperability Offset"),
            Tag::FocalPlaneXResolution => write!(f, "Focal Plane X Resolution"),
            Tag::FocalPlaneYResolution => write!(f, "Focal Plane Y Resolution"),
            Tag::FocalPlaneResolutionUnit => write!(f, "Focal Plane Resolution Unit"),
            Tag::SensingMethod => write!(f, "Sensing Method"),
            Tag::FileSource => write!(f, "File Source"),
            Tag::SceneType => write!(f, "Scene Type"),
            Tag::ExposureMode => write!(f, "Exposure Mode"),
            Tag::WhiteBalance => write!(f, "White Balance"),
            Tag::DigitalZoomRatio => write!(f, "Digital Zoom Ratio"),
            Tag::FocalLengthIn35mmFormat => write!(f, "Focal Length In 35mm Format"),
            Tag::SceneCaptureType => write!(f, "Scene Capture Type"),
            Tag::GainControl => write!(f, "Gain Control"),
            Tag::Contrast => write!(f, "Contrast"),
            Tag::Saturation => write!(f, "Saturation"),
            Tag::Sharpness => write!(f, "Sharpness"),
            Tag::DeviceSettingDescription => write!(f, "Device Setting Description"),
            Tag::SubjectDistanceRange => write!(f, "Subject Distance Range"),
            Tag::ImageUniqueID => write!(f, "Image Unique ID"),
            Tag::OwnerName => write!(f, "Owner Name"),
            Tag::SerialNumber => write!(f, "Serial Number"),
            Tag::LensSpecification => write!(f, "Lens Specification"),
            Tag::LensMake => write!(f, "Lens Make"),
            Tag::LensModel => write!(f, "Lens Model"),
            Tag::LensSerialNumber => write!(f, "Lens Serial Number"),
            Tag::Title => write!(f, "Title"),
            Tag::Raw(val) => write!(f, "Unknown({:02x?})", val),
        }
    }
}