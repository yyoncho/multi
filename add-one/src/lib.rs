
#[repr(transparent)]
#[derive(PartialEq, Eq, Clone)]
pub struct LispObject(pub i32);

pub fn foo(f: LispObject) -> LispObject {
    f
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}
