/// TODO: Add doc here.
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
