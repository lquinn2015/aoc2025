/*
    Usage
    let in = io::input::Input::stdin();
    let out = io::output::Output::stdout();


    in.read_u32();
    out.print_line((1.0, 2u32));

*/

pub mod input {
    use std::fs::File;
    use std::io::{Read, Stdin};
    use std::mem::MaybeUninit;
    enum InputSource {
        Stdin(Stdin),
        File(File),
        Slice,
    }

    pub struct Input {
        input: InputSource,
        buf: Vec<u8>,
        at: usize,
        buf_read: usize,
        eol: bool,
    }

    macro_rules! read_impl {
        ($t:ty, $read_name:ident, $read_vec_name:ident) => {
            pub fn $read_name(&mut self) -> $t {
                self.read()
            }
            pub fn $read_vec_name(&mut self, len: usize) -> Vec<$t> {
                self.read_vec(len)
            }
        };
        ($t:ty, $read_name:ident, $read_vec_name:ident, $read_pair_vec_name:ident) => {
            read_impl!($t, $read_name, $read_vec_name);
            pub fn $read_pair_vec_name(&mut self, len: usize) -> Vec<($t, $t)> {
                self.read_vec(len)
            }
        };
    }
    impl Input {
        const DEFAULT_BUF_SIZE: usize = 4096;
        pub fn slice(input: &[u8]) -> Self {
            Self {
                input: InputSource::Slice,
                buf: input.to_vec(),
                at: 0,
                buf_read: input.len(),
                eol: true,
            }
        }
        pub fn stdin() -> Self {
            Self {
                input: InputSource::Stdin(std::io::stdin()),
                buf: vec![0; Self::DEFAULT_BUF_SIZE],
                at: 0,
                buf_read: 0,
                eol: true,
            }
        }
        pub fn file(file: File) -> Self {
            Self {
                input: InputSource::File(file),
                buf: vec![0; Self::DEFAULT_BUF_SIZE],
                at: 0,
                buf_read: 0,
                eol: true,
            }
        }
        pub fn get(&mut self) -> Option<u8> {
            if self.refill_buffer() {
                let res = self.buf[self.at];
                self.at += 1;
                if res == b'\r' {
                    self.eol = true;
                    if self.refill_buffer() && self.buf[self.at] == b'\n' {
                        self.at += 1;
                    }
                    return Some(b'\n');
                }
                self.eol = res == b'\n';
                Some(res)
            } else {
                None
            }
        }
        pub fn peek(&mut self) -> Option<u8> {
            if self.refill_buffer() {
                let res = self.buf[self.at];
                Some(if res == b'\r' { b'\n' } else { res })
            } else {
                None
            }
        }
        pub fn skip_whitespace(&mut self) {
            while let Some(b) = self.peek() {
                if !b.is_ascii_whitespace() {
                    return;
                }
                self.get();
            }
        }
        pub fn next_token(&mut self) -> Option<Vec<u8>> {
            self.skip_whitespace();
            let mut res = Vec::new();
            while let Some(c) = self.get() {
                if c.is_ascii_whitespace() {
                    break;
                }
                res.push(c);
            }
            if res.is_empty() {
                None
            } else {
                Some(res)
            }
        }
        pub fn is_exhausted(&mut self) -> bool {
            self.peek().is_none()
        }
        pub fn is_empty(&mut self) -> bool {
            self.skip_whitespace();
            self.is_exhausted()
        }
        pub fn check_empty(&mut self) -> bool {
            match self.input {
                InputSource::Slice => self.is_empty(),
                _ => true,
            }
        }
        pub fn read<T: Readable>(&mut self) -> T {
            T::read(self)
        }
        pub fn read_vec<T: Readable>(&mut self, size: usize) -> Vec<T> {
            let mut res = Vec::with_capacity(size);
            for _ in 0..size {
                res.push(self.read());
            }
            res
        }
        pub fn read_char(&mut self) -> u8 {
            self.skip_whitespace();
            self.get().unwrap()
        }
        read_impl!(u32, read_u32, read_u32_vec);
        read_impl!(u64, read_u64, read_u64_vec);
        read_impl!(usize, read_usize, read_usize_vec, read_usize_pair_vec);
        read_impl!(i32, read_i32, read_i32_vec, read_i32_pair_vec);
        read_impl!(i64, read_i64, read_i64_vec, read_i64_pair_vec);
        read_impl!(i128, read_i128, read_i128_vec);
        fn refill_buffer(&mut self) -> bool {
            if self.at == self.buf_read {
                self.at = 0;
                self.buf_read = match &mut self.input {
                    InputSource::Stdin(stdin) => stdin.read(&mut self.buf).unwrap(),
                    InputSource::File(file) => file.read(&mut self.buf).unwrap(),
                    InputSource::Slice => 0,
                };
                self.buf_read != 0
            } else {
                true
            }
        }
        pub fn is_eol(&self) -> bool {
            self.eol
        }
    }
    pub trait Readable {
        fn read(input: &mut Input) -> Self;
    }
    impl Readable for u8 {
        fn read(input: &mut Input) -> Self {
            input.read_char()
        }
    }
    impl<T: Readable> Readable for Vec<T> {
        fn read(input: &mut Input) -> Self {
            let size = input.read();
            input.read_vec(size)
        }
    }
    impl<T: Readable, const SIZE: usize> Readable for [T; SIZE] {
        fn read(input: &mut Input) -> Self {
            unsafe {
                let mut res = MaybeUninit::<[T; SIZE]>::uninit();
                for i in 0..SIZE {
                    let ptr: *mut T = (*res.as_mut_ptr()).as_mut_ptr();
                    ptr.add(i).write(input.read::<T>());
                }
                res.assume_init()
            }
        }
    }
    macro_rules! read_integer {
        ($($t:ident)+) => {
            $(impl Readable for $t {
                fn read(input : & mut Input) -> Self {
                    input.skip_whitespace();
                    let mut c = input.get().unwrap();
                    let sgn = match c {
                        b'-' => {
                            c = input.get().unwrap();
                            true
                        }
                        b'+' => {
                            c = input.get().unwrap();
                            false
                        } _ => false,
                    };
                    let mut res = 0;
                    loop { assert!(c.is_ascii_digit());
                        res *= 10;
                        let d = (c - b'0') as $t;
                        if sgn { res -= d; } else { res += d; }
                        match input.get() {
                            None => break,
                            Some(ch) => {
                                if ch.is_ascii_whitespace() { break; } else { c = ch; } }
                        }
                    }
                    res
                }
            })+
        };
    }
    read_integer!(i8 i16 i32 i64 i128 isize u16 u32 u64 u128 usize);
    macro_rules! tuple_readable {
        ($($name:ident)+) => {
            impl <$($name : Readable),+> Readable for ($($name,)+) { fn read(input : & mut
            Input) -> Self { ($($name ::read(input),)+) } }
        };
    }
    tuple_readable! {
        T
    }
    tuple_readable! {
        T U
    }
    tuple_readable! {
        T U V
    }
    tuple_readable! {
        T U V X
    }
    tuple_readable! {
        T U V X Y
    }
    tuple_readable! {
        T U V X Y Z
    }
    tuple_readable! {
        T U V X Y Z A
    }
    tuple_readable! {
        T U V X Y Z A B
    }
    tuple_readable! {
        T U V X Y Z A B C
    }
    tuple_readable! {
        T U V X Y Z A B C D
    }
    tuple_readable! {
        T U V X Y Z A B C D E
    }
    tuple_readable! {
        T U V X Y Z A B C D E F
    }
}

pub mod output {
    use std::cmp::Reverse;
    use std::fs::File;
    use std::io::{StdoutLock, Write};
    #[derive(Copy, Clone)]
    pub enum BoolOutput {
        YesNo,
        YesNoCaps,
        PossibleImpossible,
        Custom(&'static str, &'static str),
    }
    impl BoolOutput {
        pub fn output(&self, output: &mut Output, val: bool) {
            (if val { self.yes() } else { self.no() }).write(output);
        }
        fn yes(&self) -> &str {
            match self {
                BoolOutput::YesNo => "Yes",
                BoolOutput::YesNoCaps => "YES",
                BoolOutput::PossibleImpossible => "Possible",
                BoolOutput::Custom(yes, _) => yes,
            }
        }
        fn no(&self) -> &str {
            match self {
                BoolOutput::YesNo => "No",
                BoolOutput::YesNoCaps => "NO",
                BoolOutput::PossibleImpossible => "Impossible",
                BoolOutput::Custom(_, no) => no,
            }
        }
    }
    enum OutputDest<'s> {
        Stdout(StdoutLock<'static>),
        File(File),
        Buf(&'s mut Vec<u8>),
    }
    pub struct Output<'s> {
        output: OutputDest<'s>,
        buf: Vec<u8>,
        at: usize,
        bool_output: BoolOutput,
        precision: Option<usize>,
        separator: u8,
    }
    impl<'s> Output<'s> {
        pub fn buf(buf: &'s mut Vec<u8>) -> Self {
            Self::new(OutputDest::Buf(buf))
        }
        fn new(output: OutputDest<'s>) -> Self {
            Self {
                output,
                buf: vec![0; Self::DEFAULT_BUF_SIZE],
                at: 0,
                bool_output: BoolOutput::YesNoCaps,
                precision: None,
                separator: b' ',
            }
        }
    }
    impl Output<'static> {
        pub fn stdout() -> Self {
            Self::new(OutputDest::Stdout(std::io::stdout().lock()))
        }
        pub fn file(file: File) -> Self {
            Self::new(OutputDest::File(file))
        }
    }
    impl Output<'_> {
        const DEFAULT_BUF_SIZE: usize = 4096;
        pub fn flush(&mut self) {
            if self.at != 0 {
                match &mut self.output {
                    OutputDest::Stdout(stdout) => {
                        stdout.write_all(&self.buf[..self.at]).unwrap();
                        stdout.flush().unwrap();
                    }
                    OutputDest::File(file) => {
                        file.write_all(&self.buf[..self.at]).unwrap();
                        file.flush().unwrap();
                    }
                    OutputDest::Buf(buf) => buf.extend_from_slice(&self.buf[..self.at]),
                }
                self.at = 0;
            }
        }
        pub fn print<T: Writable>(&mut self, s: T) {
            s.write(self);
        }
        pub fn print_line<T: Writable>(&mut self, s: T) {
            self.print(s);
            self.put(b'\n');
        }
        pub fn put(&mut self, b: u8) {
            self.buf[self.at] = b;
            self.at += 1;
            if self.at == self.buf.len() {
                self.flush();
            }
        }
        pub fn print_per_line<T: Writable>(&mut self, arg: &[T]) {
            self.print_per_line_iter(arg.iter());
        }
        pub fn print_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
            let mut first = true;
            for e in iter {
                if first {
                    first = false;
                } else {
                    self.put(self.separator);
                }
                e.write(self);
            }
        }
        pub fn print_line_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
            self.print_iter(iter);
            self.put(b'\n');
        }
        pub fn print_per_line_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
            for e in iter {
                e.write(self);
                self.put(b'\n');
            }
        }
        pub fn set_bool_output(&mut self, bool_output: BoolOutput) {
            self.bool_output = bool_output;
        }
        pub fn set_precision(&mut self, precision: usize) {
            self.precision = Some(precision);
        }
        pub fn reset_precision(&mut self) {
            self.precision = None;
        }
        pub fn get_precision(&self) -> Option<usize> {
            self.precision
        }
        pub fn separator(&self) -> u8 {
            self.separator
        }
        pub fn set_separator(&mut self, separator: u8) {
            self.separator = separator;
        }
    }
    impl Write for Output<'_> {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let mut start = 0usize;
            let mut rem = buf.len();
            while rem > 0 {
                let len = (self.buf.len() - self.at).min(rem);
                self.buf[self.at..self.at + len].copy_from_slice(&buf[start..start + len]);
                self.at += len;
                if self.at == self.buf.len() {
                    self.flush();
                }
                start += len;
                rem -= len;
            }
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            self.flush();
            Ok(())
        }
    }
    pub trait Writable {
        fn write(&self, output: &mut Output);
    }
    impl Writable for &str {
        fn write(&self, output: &mut Output) {
            output.write_all(self.as_bytes()).unwrap();
        }
    }
    impl Writable for String {
        fn write(&self, output: &mut Output) {
            output.write_all(self.as_bytes()).unwrap();
        }
    }
    impl Writable for char {
        fn write(&self, output: &mut Output) {
            output.put(*self as u8);
        }
    }
    impl Writable for u8 {
        fn write(&self, output: &mut Output) {
            output.put(*self);
        }
    }
    impl<T: Writable> Writable for [T] {
        fn write(&self, output: &mut Output) {
            output.print_iter(self.iter());
        }
    }
    impl<T: Writable, const N: usize> Writable for [T; N] {
        fn write(&self, output: &mut Output) {
            output.print_iter(self.iter());
        }
    }
    impl<T: Writable + ?Sized> Writable for &T {
        fn write(&self, output: &mut Output) {
            T::write(self, output)
        }
    }
    impl<T: Writable> Writable for Vec<T> {
        fn write(&self, output: &mut Output) {
            self.as_slice().write(output);
        }
    }
    impl Writable for () {
        fn write(&self, _output: &mut Output) {}
    }
    macro_rules! write_to_string {
        ($($t:ident)+) => {
            $(impl Writable for $t { fn write(& self, output : & mut Output) { self
            .to_string().write(output); } })+
        };
    }
    write_to_string!(u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
    macro_rules! tuple_writable {
        ($name0:ident $($name:ident : $id:tt)*) => {
            impl <$name0 : Writable, $($name : Writable,)*> Writable for ($name0, $($name,)*)
            { fn write(& self, out : & mut Output) { self.0.write(out); $(out.put(out
            .separator); self.$id .write(out);)* } }
        };
    }
    tuple_writable! {
        T
    }
    tuple_writable! {
        T U : 1
    }
    tuple_writable! {
        T U : 1 V : 2
    }
    tuple_writable! {
        T U : 1 V : 2 X : 3
    }
    tuple_writable! {
        T U : 1 V : 2 X : 3 Y : 4
    }
    tuple_writable! {
        T U : 1 V : 2 X : 3 Y : 4 Z : 5
    }
    tuple_writable! {
        T U : 1 V : 2 X : 3 Y : 4 Z : 5 A : 6
    }
    tuple_writable! {
        T U : 1 V : 2 X : 3 Y : 4 Z : 5 A : 6 B : 7
    }
    tuple_writable! {
        T U : 1 V : 2 X : 3 Y : 4 Z : 5 A : 6 B : 7 C : 8
    }
    impl<T: Writable> Writable for Option<T> {
        fn write(&self, output: &mut Output) {
            match self {
                None => (-1).write(output),
                Some(t) => t.write(output),
            }
        }
    }
    impl Writable for bool {
        fn write(&self, output: &mut Output) {
            let bool_output = output.bool_output;
            bool_output.output(output, *self)
        }
    }
    impl<T: Writable> Writable for Reverse<T> {
        fn write(&self, output: &mut Output) {
            self.0.write(output);
        }
    }
}
