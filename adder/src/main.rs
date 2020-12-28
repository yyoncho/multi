use add_one::{self, LispObject, foo};

type LO = LispObject;

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for LO {
    fn summarize(&self) -> String {
        "aaa".into()
    }
}


fn main() {
    let l: LO = LispObject(10);
    l.summarize();
    foo(l);
    print!("Working")
}
