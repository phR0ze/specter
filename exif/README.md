# Exif
Exif according to the JEIDA/JEITA/CIPA specifications is a standard that specifies formats for 
images, sound, and ancillary tags used by digital cameras, smart phones, scanners and other systems. 
The specification adds metadata tags to existing media formats e.g. JPEG, TIFF, WAV, PCM, IMA-ADPCM. 
It doesn't support JPEG 2000 or GIF. There is both an Exif image file specification as well as an 
Exif audio file specification. The Exif metadata tags cover camera settings, image metrics, data and 
time, location, thumbnails, descriptions, copyright information etc...

### Quick links
* [Overview](#overview)
  * [Timeline](#timeline)
  * [References](#references)
  * [Terms](#terms)
* [JPEG](#jpeg)
  * [JFIF](#jfif)
  * [JPEG EXIF](#jpeg-exif)
* [TIFF](#tiff)

## Overview
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
| 2.31          | Jul 2016      |	
| 2.32 	        | May 2019      |	
| 3.0  	        | May 2023 	    | UTF-8 data type 

### References:
* [Wikipedia Exif](https://en.wikipedia.org/wiki/Exif)
* [Exif 3.0 spec](https://archive.org/details/exif-specs-3.0-dc-008-translation-2023-e/)
* [MIT quick reference for 2.1](https://www.media.mit.edu/pia/Research/deepview/exif.html)
* [File Signatures](https://web.archive.org/web/20221112073316/https://www.garykessler.net/library/file_sigs.html)
 
### Terms
* ***DSC*** - Digital Still Camera
* ***DVC*** - Digital Video Camera
* ***DTV*** - Digital Television
* ***Exif*** - Exchangeable image file format
* ***JEIDA*** - Japan Electronic Industries Development Association
* ***JEITA*** - Japan Electronic and Information Technologies Industries Association
* ***Primary Image*** - The main image data
* ***Thumbnail Image*** - A reduced-size image used to index the primary image

## Tags

### Examples
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

### Time Tags

## JPEG
JPEG's are constructed using `Markers`. Markers are a binary formatted value used to mark a segment 
of the file for a specific purpose e.g. start of the image data, end of the image data, etc...

**References**
* [JFIF Wikipedia](https://en.wikipedia.org/wiki/JPEG_File_Interchange_Format)

| Marker   | Name | Data    | Description
| -------- | ---- | ------- | -------------
| `0xFFD8` | SOI  |         | Start of the file
| `0xFFD9` | EOI  |         | End of image data
| `0xFFE0` | APP0 | "JFIF"  | JFIF marker segment
| `0xFFE1` | APP1 | "Exif"  | Exif marker segment
| `0xFFE2` | APP2 | "Exif"  | Exif marker segment

Marker format `0xFF` + Marker Number (1 byte) + Data size (2 bytes) + Data (n bytes).

### Marker size component
The size component consists of 2 bytes taken together to represent a big endian 16-bit integer 
specifying the length of the data following e.g. `0xFF 0xE0 0x01 0x02` has a size of 
`0x01 + 0x02 - 2`


is 2 bytes in big endian format. . Markers `0xFFE0` through `0xFFEF` are called 
application markers and are not necessary for decoding the image. They are used by cameras and 
applications to store information.


### JFIF
[JPEG File Interchange Format](https://www.loc.gov/preservation/digital/formats/fdd/fdd000018.shtml) 
builds on JPEG to store metadata in the file. It uses the APP0 `0xFFE0` marker to insert camera 
configuration data and a thumbnail of the image. JFIF files start with `0xFFD8 0xFFE0`.

**References**
* [JFIF 1.02 specification](https://web.archive.org/web/20120301195630/http:/www.jpeg.org/public/jfif.pdf)


### JPEG EXIF
When Exif is embedded in a JPEG, the exif data is stored in one of JPEG's defined Application 
Segments. The `APP1` segment with marker `0xFFE1` can in effect hold an entire TIFF within it.

Exif metadata is restricted in size to 64 kB in JPEG images as the specification defines that all 
exif data should reside in the APP1 segment although the FlashPix extensions allow information to 
span APP1 and APP2 segments however this is not commonly used. This has led some camera manufacturers 
to develop non standard ways to store large preview images in the image.

### Example headers
```
0x000000: 0xff 0xd8 0xff 0xe0 0x00 0x10 0x4a 0x46 0x49 0x46 0x00 0x01 0x02 0x01 0x00 0x48 0x00 0x48 0x00 0x00
```
* The first two bytes `0xff 0xd8` indicate this is a JPEG image
* The next two bytes `0xff 0xe0` indicate that there is JFIF metadata stored in it
* 


### TIFF
When Exif is embedded in a TIFF, the exif data is stored in a TIFF sub-image file directory (sub-IFD) 
using the tag `0x8769` or the global sub-IFD defined by the tag `0x8825` or the Interoperability IFD 
defined with the tag `0xA005`.
