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

/// 0x5F (underscore _)
/// Checks for valid identifier [A-Za-z_]
/// Allow equal to (=) 0x3D
/// Allow {} 0x7B 0x7D
/// Allow [] 0x5B 0x5D
fn is_func_param(chr: char) -> bool
{
    let chr: u8 = chr as u8;
    (chr >= 0x41 && chr <= 0x5A) || (chr >= 0x61 && chr <= 0x7A) || (chr == 0x5F) || (chr == 0x3D)
    || (chr == 0x7B) || (chr == 0x7D) || (chr == 0x5B) || (chr == 0x5D)
}

pub fn func_param<T>(input: T) -> IResult<T, T> where
    T: Slice<Range<usize>>+Slice<RangeFrom<usize>>+Slice<RangeTo<usize>>,
    T: InputIter+InputLength
{
    let input_length = input.input_len();
    if input_length == 0 {
        return Incomplete(Needed::Unknown);
    }

    for (idx, item) in input.iter_indices() {
        if !is_func_param(item.as_char()) {
            if idx == 0 {
                return Error(error_position!(ErrorKind::Alpha, input))
            } else {
                return Done(input.slice(idx..), input.slice(0..idx))
            }
        }
    }
    Done(input.slice(input_length..), input)
}

pub fn emptyline<T>(input: T) -> IResult<T, T> where
    T: Slice<Range<usize>>+Slice<RangeFrom<usize>>+Slice<RangeTo<usize>>,
    T: InputIter+InputLength
{
    let input_length = input.input_len();
    if input_length == 0 {
        return Incomplete(Needed::Unknown);
    }

    let mut start: bool = false;
    let mut newline_index: usize = 0;

    for(idx, item) in input.iter_indices() {
        let item_char = item.as_char();
        if item_char == '\n' {
            start = true;
            newline_index = idx;
        }

        if start == true && item_char == ' ' {
            continue;
        }

        if start == true && !(item_char == '\n' || item_char == ' ') {
            return Done(input.slice(newline_index..), input.slice(0..newline_index));
        } else if start == false && (item_char != '\n') {
            return Done(input.slice(input_length..), input);
        }
    }
    Done(input.slice(input_length..), input)
}
