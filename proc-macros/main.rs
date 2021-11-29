// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

concat_idents::concat_idents! {
    fn t() -> u32 {
        42
    }
}

fn main() {}
