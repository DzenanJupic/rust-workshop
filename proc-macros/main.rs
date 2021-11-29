// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run


macro_rules! generate_test {
    ($struct:ident $fn:ident) => {
        concat_idents::concat_idents!(fn_ident = $fn, _, $struct {
            #[test]
            fn fn_ident() {
                // -- snip --
            }
        });
    };
}

generate_test!(u32 add);

fn main() {}
