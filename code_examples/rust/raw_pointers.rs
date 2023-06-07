
#[repr(C)]
struct A {
    first: i8,
    last: i8
}

fn main() {
    // creates a byte array with 2 elements and zero initialized
    let mut buffer: [i8; 2] = [0; 2];

    // creates a raw pointer to a mutable array by casting the reference
    let buffer_ptr = &mut buffer as *mut [i8];

    // creates a raw pointer to immutable i8 by casting the reference
    let sec_elmt_ptr = &buffer[1] as *const i8;

    // should be kind the same as above
    let also_sec_ptr = unsafe {&(*sec_elmt_ptr) as *const i8};

    // creates a raw pointer to a mutable struct of type A by casting a raw 
    // pointer that points to a immutable array
    let struct_ptr = &buffer as *const [i8] as *mut A;

    // dereferencing the buffer data using the raw pointer
    unsafe {
        (*buffer_ptr)[0] = 10;
        (*buffer_ptr)[1] = 12;
    }

    // reading the buffer array like it was a struct of type A
    // and taking a look at the addresses
    println!("(*struct_ptr).first is: {}", unsafe { (*struct_ptr).first });
    println!("(*struct_ptr).last is: {}", unsafe { (*struct_ptr).last });
    println!("struct_ptr is pointing to: {:?}", struct_ptr);
    println!("sec_elmt_ptr is pointing to: {:?}", sec_elmt_ptr);
    println!("also_sec_ptr is pointing to: {:?}", also_sec_ptr);

    // creates a immutable u64 by casting a pointer
    let addr_to_u64 = sec_elmt_ptr as u64;
    println!("and the casted value is: {:#X}", addr_to_u64);
    println!("and now i can do math on it like +4 = {:#X}", addr_to_u64 + 4);
}
