# Meta
Specter meta supports a collection of metadata formats from various media files.

## Goals
* Provide a simple API to read and write media file metadata
  * Ability to target specific tags by name or category
* High performance tag location without unncessary reads
* Support removing PII or all metadata from files
* Support bulk operations

### Quick links
* [Overview](#overview)
  * [Terms](#terms)
  * [File signatures](#file-signatures)
* [Exif](#exif)
  * [Timeline](#timeline)
  * [References](#references)
  * [Tags](#tags)
    * [Example tags](#example-tags)
    * [Time tags](#time-tags)
  * [Exif structure](#exif-structure)
* [XMP](#xmp)
* [GIF](#gif)
* [JPEG](#jpeg)
  * [Marker size component](#marker-size-component)
  * [JFIF](#jfif)
* [TIFF](#tiff)
 
## Overview

### Terms
* ***DSC*** - Digital Still Camera
* ***DVC*** - Digital Video Camera
* ***DTV*** - Digital Television
* ***Exif*** - Exchangeable image file format
* ***JEIDA*** - Japan Electronic Industries Development Association
* ***JEITA*** - Japan Electronic and Information Technologies Industries Association
* ***Primary Image*** - The main image data
* ***Thumbnail Image*** - A reduced-size image used to index the primary image

### File signatures
Per [Gary Kessler's signatures page](https://www.garykessler.net/library/file_sigs.html) I'm looking 
to target mediat files:

| File Extension    | Hex Signature                         | ASCI Signature  | Description
| ----------------- | ------------------------------------- | --------------- | ----------------------------
| JPE,JPEG,JPG,JFIF | `FF D8` prefix, `FF D9` suffix        | `ÿØ`            | Generic JPEG Image file
|                   | `FF D8 FF E1 xx xx 45 78 69 66 00`    | `ÿØÿà..JFIF.`   | JPEG JFIF application segment
|                   | `FF D8 FF E1 xx xx 45 78 69 66 00`    | `ÿØÿà..Exif.`   | JPEG Exif application segment
|                   | `FF D8 FF E8 xx xx 53 50 49 46 46 00` | `ÿØÿà..SPIFF.`  | JPEG SPIFF application segment

## Exif
Exif according to the JEIDA/JEITA/CIPA specifications is a standard that specifies formats for 
images, sound, and ancillary tags used by digital cameras, smart phones, scanners and other systems. 
The specification adds metadata tags to existing media formats e.g. JPEG, TIFF, WAV, PCM, IMA-ADPCM. 
It doesn't support JPEG 2000 or GIF. There is both an Exif image file specification as well as an 
Exif audio file specification. The Exif metadata tags cover camera settings, image metrics, data and 
time, location, thumbnails, descriptions, copyright information etc...

The Exif tag structure is borrowed from the TIFF format. 

### Timeline
| Version       | Release Date  | Changes
| ------------- | ------------- | ---------------------------------------
| 1.0 	        | Oct 1995      | Removed dependencies to io package
| 1.1  	        | May 1997      |	
| 2.0  	        | Nov 1997 	    | License change to MIT license
| 2.1  	        | Dec 1998      |	
| 2.2  	        | Apr 2002 	    | Added HEIC support
| 2.21 	        | Sep 2003 	    | Addition of "Exif Print"
| 2.21 unified  |	Sep 2009      |	
| 2.3  	        | Apr 2010 	    |
| 2.3 revised   |	Dec 2012      |	
| 2.31          | Jul 2016      |	Added OffsetTime, OffsetTimeOriginal, OffsetTimeDigitized
| 2.32 	        | May 2019      |	
| 3.0  	        | May 2023 	    | UTF-8 data type 

### References:
* [Wikipedia Exif](https://en.wikipedia.org/wiki/Exif)
* [Exif 3.0 spec](https://archive.org/details/exif-specs-3.0-dc-008-translation-2023-e/)
* [MIT quick reference for 2.1](https://www.media.mit.edu/pia/Research/deepview/exif.html)
* [File Signatures](https://web.archive.org/web/20221112073316/https://www.garykessler.net/library/file_sigs.html)
* [Exiftool tag names](https://exiftool.org/TagNames/EXIF.html)
* [Exif 2.32 spec](https://web.archive.org/web/20190624045241if_/http://www.cipa.jp:80/std/documents/e/DC-008-Translation-2019-E.pdf)
* [Exiftool industry standard](https://exiftool.org/)
* [Sample jpegs to validate against](https://github.com/cdcseacave/TinyEXIF/tree/master/Samples)

### Tags

#### Example tags
| Tag                       | Value
| ------------------------- | ----------------------------------
| Manufacturer              | CASIO
| Model                     | QV-4000
| Orientation (rotation)    | top-left [8 possible values[29]]
| Software                  | Ver1.01
| Date and time             | 2003:08:11 16:45:32
| YCbCr positioning         | centered
| Compression               | JPEG compression
| X resolution              | 72.00
| Y resolution              | 72.00
| Resolution unit           | Inch
| Exposure time             | 1/659 s
| F-number                  | f/4.0
| Exposure program          | Normal program
| Exif version              | Exif version 2.1
| Date and time (original)  | 2003:08:11 16:45:32
| Date and time (digitized) | 2003:08:11 16:45:32
| Components configuration  | Y Cb Cr –
| Compressed bits per pixel | 4.01
| Exposure bias             | 0.0
| Max. aperture value       | 2.00
| Metering mode             | Pattern
| Flash                     | Flash did not fire
| Focal length              | 20.1 mm
| MakerNote                 | 432 bytes unknown data
| FlashPix version          | FlashPix version 1.0
| Color space               | sRGB
| Pixel X dimensio          | 2240
| Pixel Y dimension         | 1680
| File source               | DSC
| Interoperability index    | R98
| Interoperability version  | (null) 

#### Time Tags

### Exif Structure
| Field                   | Size | Description
| ----------------------- | ---- | ------------------------------------------
| `4578 6966 0000`        | 6    | Exif header i.e. `Exif` and 2 bytes of `0x00`
| `4949 2A00 0800 0000`   | 8    | Tiff header, 2 bytes of align `0x4949` is Little-Endian, `0x4D4D` is Big-Endian

#### IFD structure
The Image File Directory (IFD) structure contains image information data.

| Field         | Size | Description
| ------------- | ---- | ------------------------------------------
| `NNNN`        | 2    | Number of file entries in the IFD
| `XX...XX`     | 12   | Entry 0 Header: each entry gets a 12 byte header
| `XX...XX`     | 12   | Entry 1 Header
| ....          | ...  | ...
| `XX...XX`     | 12   | Entry N Header

#### IFD entry structure 


## XMP
Extensible Metadata Platform (XMP)

**References**
* [XMP Wikipedia](https://en.wikipedia.org/wiki/Extensible_Metadata_Platform)

## GIF
**References**
* [GIF Wikipedia](https://en.wikipedia.org/wiki/GIF#Metadata)

## JPEG
JPEG's are constructed using `Markers`. Markers are a binary formatted value used to mark a segment 
of the file for a specific purpose e.g. start of the image data, end of the image data, etc...

JPEG data is all in Big-Endian format except for potentially Exif which can be in Little-Endian but 
usually not. Start of Scan is immediately followed by the actual image data without any size until it 
reaches the end of the file marker.

**References**
* [JFIF Wikipedia](https://en.wikipedia.org/wiki/JPEG_File_Interchange_Format)
* [Exif MIT](https://www.media.mit.edu/pia/Research/deepview/exif.html)
* [ExifLibrary for DotNet](https://www.codeproject.com/Articles/43665/ExifLibrary-for-NET)
* [Markers enumerated](https://techstumbler.blogspot.com/2008/09/jpeg-marker-codes.html)
* [Decode JPEG in Python](https://practicalpython.yasoob.me/chapter10.html)

| Marker   | Name | Data    | Description
| -------- | ---- | ------- | -------------
| `0xFFD8` | SOI  |         | Start of image file i.e. JPEG header
| `0xFFC0` | SOF0 |         | Start of frame (Baseline DCT)
| `0xFFC2` | SOF2 |         | Start of frame (Progressive DCT)
| `0xFFC3` | SOF3 |         | Start of frame (Lossless sequential)
| `0xFFC4` | DHT  |         | Define Huffman Table, there are usually 4
| `0xFFC5` | SOF5 |         | Start of frame (Differential squeential DCT)
| `0xFFC6` | SOF6 |         | Start of frame (Differential progressive DCT)
| `0xFFC7` | SOF7 |         | Start of frame (Differential lossless DCT)
| `0xFFD0` | RST  |         | Restart `0xFFD0 - 0xFFD7`
| `0xFFDA` | SOS  |         | Start of scan i.e. start of image data
| `0xFFDB` | DQT  |         | Define Quantization Table
| `0xFFDC` | DNL  |         | Define Number of Lines
| `0xFFDD` | DRI  |         | Define Restart Interval
| `0xFFDE` | DHP  |         | Define Hierarchical Progression
| `0xFFDF` | EXP  |         | Expand References Components
| `0xFFD9` | EOI  |         | End of image file i.e. JPEG footer
| `0xFFE0` | APP0 | "JFIF"  | JFIF marker segment
| `0xFFE1` | APP1 | "Exif"  | Exif marker segment
| `0xFFE2` | APP2 | "CIFF"  | Canon Camera Image File Format
| `?`      | ?    | "CIPA"  | Mutli Picture Object specification
| `0xFFE3` | APP3 |         | ?
| `0xFFE4` | APP4 |         | ?
| `0xFFE5` | APP5 |         | ?
| `0xFFE6` | APP6 |         | ?
| `0xFFE7` | APP7 |         | ?
| `0xFFE8` | APP8 | "SPIFF" | Still Picture Interchange File Format
| `0xFFE9` | APP9 |         | ?
| `0xFFEA` | APPA |         | ?
| `0xFFEB` | APPB |         | ?
| `0xFFEC` | APPC |         | ?
| `0xFFED` | APPD |         | ?
| `0xFFEE` | APPE |         | ?
| `0xFFEF` | APPF |         | ?
| `0xFFF0` | EXT0 |         | Extensions
| `0xFFF1` | EXT1 |         | Extensions
| `0xFFF2` | EXT2 |         | Extensions
| `0xFFF3` | EXT3 |         | Extensions
| `0xFFF4` | EXT4 |         | Extensions
| `0xFFF5` | EXT5 |         | Extensions
| `0xFFF6` | EXT6 |         | Extensions
| `0xFFF7` | EXT7 |         | Extensions
| `0xFFF8` | EXT8 |         | Extensions
| `0xFFF9` | EXT9 |         | Extensions
| `0xFFFA` | EXTA |         | Extensions
| `0xFFFB` | EXTB |         | Extensions
| `0xFFFC` | EXTC |         | Extensions
| `0xFFFD` | EXTD |         | Extensions
| `0xFFFE` | COM  |         | Comment


Marker format `0xFF` (1 byte) + Marker Number (1 byte) + Data size (2 bytes) + Data (n bytes).
* (2 bytes) of marker
* (2 bytes) of data size
* (n bytes) of data

### Marker size component
The size component consists of 2 bytes taken together to represent a big endian 16-bit integer 
specifying the length of the data following including the 2 bytes for the data size itself e.g. `0xFF 
0xE0 0x00 0x10` has a size of `16-2=14`

Markers `0xFFE0` through `0xFFEF` are called application markers and are not necessary for decoding 
the image. They are used by cameras and applications to store metadata about the image.

### Start of Frame
The start of frame contains image data properties

| Field               | Size | Description 
| ------------------- | ---- | -------------------------
| Marker Identifier   | 2    | 0xff, 0xc0 to identify SOF0 marker
| Length              | 2    | This value equals to 8 + components`*`3 value
| Data precision      | 1    | This is in bits/sample, usually 8 (12 and 16 not supported by most software).
| Image height        | 2    | This must be > 0
| Image Width         | 2    | This must be > 0
| Number of components | 1   | Usually 1 = grey scaled, 3 = color YcbCr or YIQ
| Each component      | 3    | Read each component data of 3 bytes. It contains, (component Id(1byte)(1 = Y, 2 = Cb, 3 = Cr, 4 = I, 5 = Q), sampling factors (1byte) (bit 0-3 vertical., 4-7 horizontal.), quantization table number (1 byte)).

### Restart Interval


### Quantinization Table
The Define Quantinization Table contains the following data

| Field               | Size | Description 
| ------------------- | ---- | -------------------------
| Marker Identifier   | 2    | 0xff, 0xdb identifies DQT
| Length              | 2    | This gives the length of QT.
| QT information      | 1    | bit 0..3: number of QT (0..3, otherwise error) bit 4..7: the precision of QT, 0 = 8 bit, otherwise 16 bit
| Bytes               | n    | This gives QT values, `n = 64*(precision+1)`

### Huffman Tables
Huffman encoding is a method for lossless compression of information. The `DCT (Discrete Cosine Tranform)`
is stored in up to 4 Huffman tables in a JPEG. These "Define Huffman Table" sections.

**References**
* [Tom Scott Huffman Coding](https://www.youtube.com/watch?v=JsTptu56GM8)
* [Python Huffman explanation](https://practicalpython.yasoob.me/chapter10.html#huffman-encoding)

| Field               | Size | Description 
| ------------------- | ---- | -------------------------
| Marker Identifier   | 2    | 0xff, 0xc4 to identify DHT marker
| Length              | 2    | This specifies the length of Huffman table
| HT information      | 1    | 
| Number of Symbols   | 16   | sum(n) of these bytes is the total number of codes, which must be <= 256
| Symbols             | n    | Table containing the symbols in order of increasing code length ( n = total number of codes ).

**HT information**
* bit 0..3: number of HT (0..3, otherwise error) bit 4: type of HT, 0 = DC table, 1 = AC table bit 5..7: not used, must be 0

### JFIF
[JPEG File Interchange Format](https://www.loc.gov/preservation/digital/formats/fdd/fdd000018.shtml) 
builds on JPEG to store metadata in the file. It uses the APP0 `0xFFE0` marker to insert metadata 
about the image including an optional thumbnail. JFIF files start with `0xFFD8 0xFFE0`. JFIF may have 
an optional extension to the APP0 `0xFFE0` marker using the same marker value immediately following 
the original JFIF APP0 marker. Th extension was added post 1.02 to allow for embedding thumbnails in 
three different formats.

**References**
* [JFIF 1.02 specification](https://web.archive.org/web/20120301195630/http:/www.jpeg.org/public/jfif.pdf)
* [JFIF wikipedia](https://en.wikipedia.org/wiki/JPEG_File_Interchange_Format)

### JFIF file example
```
0x000000: 0xff 0xd8 0xff 0xe0 0x00 0x10 0x4a 0x46 0x49 0x46 0x00 0x01 0x02 0x01 0x00 0x48 0x00 0x48 0x00 0x00
0x000014: 0xff 0xe1 0x1c 0x45 0x45 0x78 0x69 0x66 0x00 0x00 0x49 0x49 0x2a 0x00 0x08 0x00 0x00 0x00 0x0b 0x00
```
| Field             | Byte# | Description
| ----------------- | ----- | ------------------------------------------
| JPEG marker       | 2     | `0xff 0xd8` indicates this file is a JPEG image
| APP0 marker       | 2     | `0xff 0xe0` indicates this file contains JFIF metadata
| APP0 data length  | 2     | `0x00 0x10` size i.e. `0x0010` or `16-2` or `14 bytes` of data.
| Identifier        | 5     | `0x4a 0x46 0x49 0x46 0x00` = `JFIF` in ASCII terminated by a null byte
| JFIF version      | 2     | `0x01 0x02` is the major and minor JFIF version i.e. `1.02`
| Density Units     | 1     | `0x00` = None, `0x01` = pixels per inch, `0x02` = pixels per centimeter
| Xdensity          | 2     | `0x00 0x48` = `72` Horizontal pixel density, Must not be zero
| Ydensity          | 2     | `0x00 0x48` = `72` Vertical pixel density, Must not be zero
| Xthumbnail        | 1     | `0x00` Horizontal pixels of the embedded RGB thumbnail, May be zero
| Ythumbnail        | 1     | `0x00` Vertical pixels of the embedded RGB thumbnail, May be zero
| Thumbnail data    | 3 x n | Uncompressed 24 bit RGB (8 bits per color channel) raster thumbnail

### JPEG EXIF
When Exif is embedded in a JPEG, the exif data is stored in one of JPEG's defined Application 
Segments. The `APP1` segment with marker `0xFFE1` can in effect hold an entire TIFF within it.

Exif metadata is restricted in size to 64 kB in JPEG images as the specification defines that all 
exif data should reside in the APP1 segment although the `FlashPix` extensions allow information to 
span `APP1` and `APP2` segments however this is not commonly used. This has led some camera 
manufacturers to develop non-standard ways to store large preview images in the image.

| Field         | Size | Description
| ------------- | ---- | ------------------------------------------
| `FFD8`        | 2    | JPEG marker indicates this file is a JPEG image
| `FFE1`        | 2    | APP1 marker indicates this file contains EXIF metadata
| `XXXX`        | 2    | APP1 data size, in Big Endian, including the 2 size bytes so subtract 2

see [Exif structure](#exif-structure) for breakdown

### TIFF
When Exif is embedded in a TIFF, the exif data is stored in a TIFF sub-image file directory (sub-IFD) 
using the tag `0x8769` or the global sub-IFD defined by the tag `0x8825` or the Interoperability IFD 
defined with the tag `0xA005`.

**References**
* [Tiff - Paul Bourke](https://paulbourke.net/dataformats/tiff/)
* [Tiff Spec Summary](https://paulbourke.net/dataformats/tiff/tiff_summary.pdf)

#### TIFF Structure
TIFF files are organized into three sections: the Image File Header (IFH), the Image File Directory 
(IFD) and the bitmap data. Only the IFH and IFD are required. TIFF files can contain multiple images. 
There is no limit on the number if IFDs in a TIFF. TIFF IFD and bitmap data can be arranged how ever 
the implementer sees fit as long as the correct offsets are in place to locate the data.

Note: TIFF IFDs are not part of the TIFF header.

#### TIFF header
First 8 bytes make up the header

| Field         | Size | Description
| ------------- | ---- | ------------------------------------------
| `4D4D`        | 2    | Alignment of `4949` i.e. `II` or Intel is Little-Endian, `4D4D` i.e. `MM` or Motorola is Big-Endian
| `002A`        | 2    | TIFF version, but always `0024` or `2400` depending on endian
| `00000008`    | 4    | Offset from the start of the TIFF to the first IFD

#### TIFF IFD structure
An IFD consists of two bytes indicating the number of entries followed by the entries themselves. The 
IFD is terminated with 4 bytes of offset to the next IFD or 0 if there are none. All TIFF files must 
contain at least one IFD.

| Field         | Size | Description
| ------------- | ---- | ------------------------------------------
| `LLLL`        | 2    | Number of entries in the IFD
| `XX...XX`     | 12n  | 12 bytes per entry for all entries
| `00000008`    | 4    | Last 4 bytes of every IFD is the offset to the next IFD or zero if no more `00000000`
