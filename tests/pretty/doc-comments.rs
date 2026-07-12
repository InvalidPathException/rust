//@ compile-flags: --crate-type=lib

//@ pp-exact

// some single-line non-doc comment

/// some single line outer-docs
fn a() {}

fn b() {
    //! some single line inner-docs
}

//////////////////////////////////
// some single-line non-doc comment preceded by a separator

//////////////////////////////////
/// some single-line outer-docs preceded by a separator
/// (and trailing whitespaces)
fn c() {}

/*
 * some multi-line non-doc comment
 */

/**
 * some multi-line outer-docs
 */
fn d() {}

fn e() {
    /*!
     * some multi-line inner-docs
     */
}

/********************************/
/*
 * some multi-line non-doc comment preceded by a separator
 */

/********************************/
/**
 * some multi-line outer-docs preceded by a separator
 */
fn f() {}

#[doc = "unsugared outer doc-comments work also"]
fn g() {}

fn h() {
    #![doc = "as do inner ones"]
}

/// outer-docs followed by a non-doc comment
// this non-doc comment must stay after the outer-docs
fn i() {}

struct SomeStruct {
    /// field outer-docs followed by a non-doc comment
    // this non-doc comment must stay after the outer-docs
    some_field: u8,
}

trait SomeTrait {
    /// associated-item outer-docs followed by a non-doc comment
    // this non-doc comment must stay after the outer-docs
    fn some_method();
}

extern "C" {
    /// foreign-item outer-docs followed by a non-doc comment
    // this non-doc comment must stay after the outer-docs
    fn some_fn();
}

fn outer() {
    /// nested-item outer-docs followed by a non-doc comment
    // this non-doc comment must stay after the outer-docs
    fn inner() {}
}

enum SomeEnum {
    /// variant outer-docs followed by a non-doc comment
    // this non-doc comment must stay after the outer-docs
    Variant,
}
