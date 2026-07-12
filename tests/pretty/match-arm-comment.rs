//@ compile-flags: --crate-type=lib
//@ pp-exact:match-arm-comment.pp

// A non-doc comment following an attribute on a match arm must stay after the
// attribute, not be reordered before it.
fn f(x: u8) {
    match x {
        #[cfg(all())]
        // this non-doc comment must stay after the attribute
        0 => {}
        _ => {}
    }
}
