// build-pass (FIXME(62277): could be check-pass?)
// pretty-expanded FIXME #23616

trait Foo {
    #[allow(anonymous_parameters)]
    fn quux(u8) {}
}

fn main() {}
