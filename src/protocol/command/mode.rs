use Argument;

use std::io::prelude::*;
use std::io;

use byteorder::ReadBytesExt;

define_command!(MODE {
    mode: Mode,
});

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mode
{
    /// Mode character 'S'.
    Stream,
    /// Mode character 'B'.
    Block,
    /// Mode character 'C'.
    Compressed,
}

impl Argument for Mode
{
    fn read(read: &mut BufRead) -> Result<Self, io::Error> {
        let c = read.read_u8()? as char;

        match c {
            'S' => Ok(Mode::Stream),
            'B' => Ok(Mode::Block),
            'C' => Ok(Mode::Compressed),
            _ => panic!("unknown argument code: {}", c),
        }
    }

    fn write(&self, write: &mut Write) -> Result<(), io::Error> {
        let mode_character = match *self {
            Mode::Stream => 'S',
            Mode::Block => 'B',
            Mode::Compressed => 'C',
        };

        write.write(&[mode_character as u8])?;
        Ok(())
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use {Command, CommandKind};
    use std::io;

    fn read(text: &str) -> MODE {
        let command_kind = CommandKind::read(&mut io::Cursor::new(text)).unwrap();

        if let CommandKind::MODE(mode) = command_kind {
            mode
        } else {
            panic!();
        }
    }

    #[test]
    fn correctly_writes_stream_modeset() {
        let command = MODE { mode: Mode::Stream };
        assert_eq!(command.to_string(), "MODE S");
    }

    #[test]
    fn correctly_writes_block_modeset() {
        let command = MODE { mode: Mode::Block };
        assert_eq!(command.to_string(), "MODE B");
    }

    #[test]
    fn correctly_writes_compressed_modeset() {
        let command = MODE { mode: Mode::Compressed };
        assert_eq!(command.to_string(), "MODE C");
    }

    #[test]
    fn correctly_reads_stream_modeset() {
        assert_eq!(read("MODE S"), MODE { mode: Mode::Stream });
    }

    #[test]
    fn correctly_reads_block_modeset() {
        assert_eq!(read("MODE B"), MODE { mode: Mode::Block });
    }

    #[test]
    fn correctly_reads_compressed_modeset() {
        assert_eq!(read("MODE C"), MODE { mode: Mode::Compressed });
    }
}

