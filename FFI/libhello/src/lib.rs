// http://bitboom.github.io/calling-rust-from-cpp

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

// // src/lib.rs
// #[no_mangle]
// pub extern fn hello() {
//     println!("Hello world!");
// }

// Same with above
#[no_mangle]
pub extern "C" fn hello() {
    println!("Hello world!");
}