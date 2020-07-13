/// defining a custom macro myvec.
/// it has match patterns that matches code passed to 
/// it and outputs code to replace the passed in code with.
#[macro_export]
macro_rules! myvec {
    // The match arm below does following:
    // $() -> capture a part of match pattern
    // $x: expr -> match any expression and store it in $x
    // , -> match comma literally
    // * -> match 0 or more times
    ( $($x:expr),* ) => {
        // entire body below will be placed as new code.
        {
            let mut tmp = Vec::new();
            // replace code for each 0 or more times
            $(
                tmp.push($x);
            )*
            tmp
        }
    };
}

/// Experiment: custom macro for printing json format strings.
/// Kinda JSON that is `{key: value}` where key and value may or may not be valid json.
/// DOESN'T WORK
// #[macro_export]
// macro_rules! json{
//     ({$($x:ident : $v:expr),*}) => {
//         {
//             println!("{");
//             $(
//                 println!("\"{:?}\": {:?},", $x, $v);
//             )*
//             println!("}");
//         }
//     };
// }

/// Procedural macros must be defined in separate crate than the one being 
/// where it will be used.
/// Instead of matching like in declarative macro, procedural macro
/// takes some code as input, processes and produced new code to output.
pub trait HelloMacro{
    fn hello();
}
