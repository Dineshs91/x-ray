use std::ops::{Range, RangeFrom, RangeTo};

use nom::{Slice, Needed, InputLength, InputIter, ErrorKind, AsChar};
use nom::IResult;
use nom::IResult::{Done, Incomplete, Error};

/// 0x5F (underscore _)
/// Checks for valid identifier [A-Za-z_]
fn is_ident(chr: char) -> bool
{
    let chr: u8 = chr as u8;
    (chr >= 0x41 && chr <= 0x5A) || (chr >= 0x61 && chr <= 0x7A) || (chr == 0x5F)
}

pub fn ident<T>(input: T) -> IResult<T, T> where 
    T: Slice<Range<usize>>+Slice<RangeFrom<usize>>+Slice<RangeTo<usize>>,
    T: InputIter+InputLength
{
    let input_length = input.input_len();
    if input_length == 0 {
        return Incomplete(Needed::Unknown);
    }

    for (idx, item) in input.iter_indices() {
        if !is_ident(item.as_char()) {
            if idx == 0 {
                return Error(error_position!(ErrorKind::Alpha, input))
            } else {
                return Done(input.slice(idx..), input.slice(0..idx))
            }
        }
    }
    Done(input.slice(input_length..), input)
}
