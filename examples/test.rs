
extern crate nest_struct;
// use nest_struct::dsl;

// macro_rules! ns {
//     ($($i:ident $b:tt),*) => {
//         $(
//             ns! $b
//         )*
//     };
// }

#[nest_struct::dsl]
fn test() {
    Depolyment {
        a: Metadata {}
    }
}

fn main() {

}
