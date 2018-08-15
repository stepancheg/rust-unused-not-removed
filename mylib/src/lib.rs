pub trait FooTrait {
    fn foo_trait_fn_proper(&self);
    fn foo_trait_fn_never_called(&self);
}

pub struct BarStruct {}

impl FooTrait for BarStruct {
    fn foo_trait_fn_proper(&self) {}
    fn foo_trait_fn_never_called(&self) {}
}

pub fn not_a_foo_trait_fn() {
}

#[inline(never)]
fn zzz(foo: &FooTrait) {
    foo.foo_trait_fn_proper();
}

#[inline(never)]
pub fn some_function() {
    let bs = BarStruct {};
    zzz(&bs);
}
