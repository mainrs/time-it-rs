use time_it::time_it;

fn main() {
    test(1, 1, "", "");
}

#[time_it]
fn test(a: usize, b: i16, v: &str, k: impl AsRef<str>) {
    println!("hello, world!");
}

struct S;
impl S {
    #[time_it]
    pub fn with_self(&mut self, v: i16) -> Option<Self> {
        todo!()
    }
}
