//@ compile-flags: --crate-type=lib
//@ pp-exact:stmt-attr-comment.pp

// A non-doc comment following an attribute on an expression/semi statement must
// stay after the attribute, not be reordered before it.
fn f() {
    #[allow(unused_unsafe)]
    // this non-doc comment must stay after the attribute
    unsafe {}
    #[allow(unused_unsafe)]
    // this non-doc comment must stay after the attribute
    unsafe {};
}
