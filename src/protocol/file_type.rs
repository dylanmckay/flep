use {Argument, Error};
use std::io::prelude::*;

use byteorder::ReadBytesExt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FileType
{
    AsciiText(TextFormat),
    EbcdicText(TextFormat),
    Binary,
    LocalFormat { bits_per_byte: u8 },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextFormat
{
    NonPrint,
    TelnetFormatControl,
    ASACarriageControl,
}

impl FileType
{
    pub fn ascii() -> Self { FileType::AsciiText(TextFormat::NonPrint) }
}

impl Argument for FileType
{
    fn read(read: &mut BufRead) -> Result<Self, Error> {
        let type_byte = read.read_u8()?;

        match type_byte as char {
            'A' => {
                let format = TextFormat::read(read)?;
                Ok(FileType::AsciiText(format))
            },
            'E' => {
                let format = TextFormat::read(read)?;
                Ok(FileType::EbcdicText(format))
            },
            'I' => Ok(FileType::Binary),
            'L' => {
                assert_eq!(read.read_u8()? as char, ' ');
                let bits_per_byte_char = read.read_u8()? as char;
                let bits_per_byte = bits_per_byte_char.to_string().parse().unwrap();
                Ok(FileType::LocalFormat { bits_per_byte: bits_per_byte })
            },
            c => panic!("invalid file type char: '{}'", c),
        }
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write!(write, " ")?;

        match *self {
            FileType::AsciiText(format) => {
                write!(write, "A")?;
                format.write(write)?;
            },
            FileType::EbcdicText(format) => {
                write!(write, "E")?;
                format.write(write)?;
            },
            FileType::Binary => write!(write, "I")?,
            FileType::LocalFormat { bits_per_byte } => {
                assert!(bits_per_byte < 10,
                        "bits per byte must be single-digit");
                write!(write, "L {}", bits_per_byte)?;
            },
        }
        Ok(())
    }
}

impl Argument for TextFormat
{
    fn read(read: &mut BufRead) -> Result<Self, Error> {
        let mut buf: [u8; 1] = [0; 1];

        // Check if we received a character.
        if read.read(&mut buf)? == 1 {
            assert_eq!(buf[0] as char, ' ');

            match read.read_u8()? as char {
                'N' => Ok(TextFormat::NonPrint),
                'T' => Ok(TextFormat::TelnetFormatControl),
                'C' => Ok(TextFormat::ASACarriageControl),
                _ => panic!("invalid text format character"),
            }
        } else {
            // The default is non-print.
            Ok(TextFormat::NonPrint)
        }
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write!(write, " ")?;

        match *self {
            TextFormat::NonPrint => write!(write, "N")?,
            TextFormat::TelnetFormatControl => write!(write, "T")?,
            TextFormat::ASACarriageControl => write!(write, "C")?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod test
{
    use Argument;
    use super::*;

    #[test]
    fn correctly_writes_ascii_nonprint() {
        assert_eq!(FileType::AsciiText(TextFormat::NonPrint).to_string(),
                   " A N");
    }

    #[test]
    fn correctly_writes_ebcdic_telnet() {
        assert_eq!(FileType::EbcdicText(TextFormat::TelnetFormatControl).to_string(),
                   " E T");
    }

    #[test]
    fn correctly_writes_binary() {
        assert_eq!(FileType::Binary.to_string(), " I");
    }

    #[test]
    fn correctly_writes_local_5bit() {
        assert_eq!(FileType::LocalFormat { bits_per_byte: 5 }.to_string(), " L 5");
    }

    #[test]
    fn correctly_reads_ascii_nonprint() {
        assert_eq!(FileType::parse_text(" A N"),
                   FileType::AsciiText(TextFormat::NonPrint));
    }

    #[test]
    fn correctly_reads_ebcdic_default_format() {
        assert_eq!(FileType::parse_text(" E"),
                   FileType::EbcdicText(TextFormat::NonPrint));
    }

    #[test]
    fn correctly_reads_binary() {
        assert_eq!(FileType::parse_text(" I"), FileType::Binary);
    }

    #[test]
    fn correctly_reads_local_2bit() {
        assert_eq!(FileType::parse_text(" L 2"),
                   FileType::LocalFormat { bits_per_byte: 2 });
    }
}
