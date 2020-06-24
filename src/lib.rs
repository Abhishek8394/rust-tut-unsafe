/// This function can now be called from C code!
#[no_mangle]
pub extern "C" fn hello_rust(){
    println!("Hello from rust!");
}
