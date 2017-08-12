// implementation Deref trait
#[macro_export]
macro_rules! impl_deref{
    ($s:ident,$t:ident, $filed:ident) => (
    impl Deref for $s {
    type Target = $t; // 目标类型
    fn deref<'a>(&'a self) -> &'a $t{
        &self.$filed
    }
    }
    )
}

// calculate *mut u8 , *count u8 c string length
#[macro_export]
macro_rules! cstr_lenght {
    // `()` indicates that the macro takes no argument.
     // The `expr` designator is used for expressions.
     // The `ident` designator is used for variable/function names.
    ($s:ident) => (
        // The macro will expand into the contents of this block.
        // The `stringify!` macro converts an `ident` into a string.
        {
        let mut i =0;
        loop {
            if *$s.offset(i) == 0 {
                break;
            }
            i = i+ 1;
        }
        i
        }
    )
}
