use std::{error::Error, fmt};

#[derive(Clone, Debug)]
pub enum RelativeError {
    PreambleMismatch,
    OpcodeMismatch,
    UnexpectedEof,
}

impl Error for RelativeError {}

impl fmt::Display for RelativeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::OpcodeMismatch => "opcode mismatch",
            Self::PreambleMismatch => "preamble mismatch",
            Self::UnexpectedEof => "expected more bytes",
        };

        fmt.write_str(message)
    }
}

pub fn resolve_relative<const PREAMBLE: usize, const OPCODE: usize>(
    preamble: [u8; PREAMBLE],
    opcode: [u8; OPCODE],
    code: &[u8],
) -> Result<*const u8, RelativeError> {
    let addr = code.as_ptr();
    let offset = const { PREAMBLE + OPCODE + size_of::<i32>() };

    let Some((chunk, code)) = code.split_first_chunk() else {
        return Err(RelativeError::UnexpectedEof);
    };

    if *chunk != preamble {
        return Err(RelativeError::PreambleMismatch);
    }

    let Some((chunk, code)) = code.split_first_chunk() else {
        return Err(RelativeError::UnexpectedEof);
    };

    if *chunk != opcode {
        return Err(RelativeError::OpcodeMismatch);
    };

    let Ok(relative) = code.try_into().map(i32::from_le_bytes) else {
        return Err(RelativeError::UnexpectedEof);
    };

    Ok(unsafe { addr.add(offset).offset(relative as isize) })
}
