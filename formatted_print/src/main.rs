use std::fmt;

fn main() {
    #[allow(dead_code)]

    struct Structure(i32);

    impl fmt::Display for Structure {
      // This trait requires `fmt` with this exact signature.
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          // Write strictly the first element into the supplied output
          // stream: `f`. Returns `fmt::Result` which indicates whether the
          // operation succeeded or failed. Note that `write!` uses syntax which
          // is very similar to `println!`.
          write!(f, "{}", self.0)
      }
    }

    println!("This struct `{}` won't print...", Structure(3));
    println!("Pi is rougly {:.*}", 3, 3.141592);
}


