use crate::buffer::Buffer;
use crate::parser::Expression;
use std::fmt::Debug;

pub mod parser;
pub mod stdio;
mod buffer;

#[derive(Debug)]
pub enum Error<IOErr> {
    Parser(parser::Error),
    IO(IOErr),
}

impl<IOErr> From<parser::Error> for Error<IOErr> {
    fn from(err: parser::Error) -> Self {
        Self::Parser(err)
    }
}

pub trait IO {
    type Err;

    fn input(&mut self) -> Result<i32, Self::Err>;
    fn output(&mut self, val: i32) -> Result<(), Self::Err>;
}

pub fn run<InOut: IO>(brainfuck: impl Iterator<Item=char>, inout: &mut InOut) -> Result<(), Error<InOut::Err>> {
    let expressions = parser::parse(brainfuck)?;
    eval(&expressions, &mut Buffer::new(256), inout)
}

fn eval<InOut: IO>(expressions: &[Expression], buffer: &mut Buffer<i32>, inout: &mut InOut) -> Result<(), Error<InOut::Err>> {
    for exp in expressions {
        match exp {
            Expression::PtrOffset(offset) => buffer.offset_ptr(*offset),
            Expression::ValOffset(offset) => buffer.offset_val(*offset),
            Expression::Input => buffer.write(inout.input().map_err(Error::IO)?),
            Expression::Output => inout.output(*buffer.read()).map_err(Error::IO)?,
            Expression::Loop(exps) => while *buffer.read() > 0 {
                eval(exps, buffer, inout)?
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{IO, run};

    struct TestIO(String);

    impl IO for TestIO {
        type Err = ();

        fn input(&mut self) -> Result<i32, Self::Err> {
            todo!()
        }

        fn output(&mut self, val: i32) -> Result<(), Self::Err> {
            self.0.push(val as u8 as char);
            Ok(())
        }
    }

    #[test]
    fn it_works() {
        let mut io = TestIO(String::new());
        let res = run("++++++++++[>+++++++>++++++++++>+++>+<<<<-]\
            >++.>+.+++++++..+++.>++.<<+++++++++++++++.>.\
            +++.------.--------.>+.>.".chars(), &mut io);
        println!("{:?}", res);
        assert_eq!(io.0, "Hello World!\n")
    }
}
