/// parse and consume a code block.
/// A code block is identified by the len(Indentation start) and ends
/// when indentation drops down.
#[macro_export]
macro_rules! block (
    ($i:expr, $len:expr) => (
        {
            use nom::InputIter;
            use nom::Slice;
            use nom::AsChar;
            let input = $i;

            let cnt = $len as usize;
            let mut res: nom::IResult<_,_> = nom::IResult::Incomplete(nom::Needed::Size(cnt));

            let mut start = false;
            let mut indent = 0;
            for (idx, item) in input.iter_indices() {
                res = nom::IResult::Done(input.slice(idx + 1..), input.slice(0..idx + 1));

                // consume everything until the indent level changes.
                if start == false && item.as_char() == '\n' {
                    start = true;
                } else if start == true && item.as_char() == ' ' {
                    indent += 1;
                } else if start == true && item.as_char() == '\n' {
                    indent = 0;
                } else if start == true && item.as_char() != ' ' {
                    start = false;
                    if indent <= cnt {
                        res = nom::IResult::Done(input.slice(idx - indent..), input.slice(0..idx - indent));
                        break;
                    } else {
                        indent = 0;
                    }
                }
            };

            res
        }
    );
);

/// many0_block is same as many0, except that when the indenation level falls
/// the parser is stopped.
#[macro_export]
macro_rules! many0_block(
  ($i:expr, $len:expr, $submac:ident!( $($args:tt)* )) => (
    {
      use nom::InputLength;
      use nom::InputIter;
      use nom::AsChar;

      let ret;
      let mut res   = ::std::vec::Vec::new();
      let mut input = $i;

      loop {
        if input.input_len() == 0 {
          ret = nom::IResult::Done(input, res);
          break;
        }

        let cnt = $len as usize;
        let mut indent = 0;

        for (idx, item) in input.iter_indices() {
            if item.as_char() == '\n' || item.as_char() == ' ' {
                if item.as_char() == ' ' {
                    indent += 1;
                }
            } else {
                break;
            }
        };

        if indent <= cnt {
            ret = nom::IResult::Done(input, res);
            break;
        }

        match $submac!(input, $($args)*) {
          nom::IResult::Error(_)                            => {
            ret = nom::IResult::Done(input, res);
            break;
          },
          nom::IResult::Incomplete(nom::Needed::Unknown) => {
            ret = nom::IResult::Incomplete(nom::Needed::Unknown);
            break;
          },
          nom::IResult::Incomplete(nom::Needed::Size(i)) => {
            let size = i + ($i).input_len() - input.input_len();
            ret = nom::IResult::Incomplete(nom::Needed::Size(size));
            break;
          },
          nom::IResult::Done(i, o)                          => {
            // loop trip must always consume (otherwise infinite loops)
            if i == input {
              ret = nom::IResult::Error(error_position!(nom::ErrorKind::Many0,input));
              break;
            }

            res.push(o);
            input = i;
          }
        }
      }

      ret
    }
  );
  ($i:expr, $len:expr , $f:expr) => (
    many0_block!($i, $len, call!($f));
  );
);

/// Take until a line containing a given tag.
/// starting from \n to the next \n see if the content has the tag.
/// If it has then consume until the previous line. Else consume the line.
#[macro_export]
macro_rules! take_until_line_containing_tag (
  ($i:expr, $substr:expr) => (
    {
      use nom::InputLength;
      use nom::FindSubstring;
      use nom::Slice;

      let res: nom::IResult<_,_> = if $substr.input_len() > $i.input_len() {
        nom::IResult::Incomplete(nom::Needed::Size($substr.input_len()))
      } else {
        match ($i).find_substring($substr) {
          None => {
            nom::IResult::Error(error_position!(nom::ErrorKind::TakeUntil,$i))
          },
          Some(index) => {
            let mut ind = index;
            while ind != 0 {
                if $i.slice(ind..ind + 1)[0] == 10 {
                    break;
                }
                ind -= 1;
            }
            nom::IResult::Done($i.slice(ind..), $i.slice(0..ind))
          },
        }
      };
      res
    }
  );
);

/// Returns false if there is no data remaining. Else returns true.
#[macro_export]
macro_rules! has_data (
  ($i:expr,) => (
    {
      use nom::InputLength;
      if ($i).input_len() == 0 {
        nom::IResult::Done($i, false)
      } else {
        nom::IResult::Done($i, true)
      }
    }
  );
);
