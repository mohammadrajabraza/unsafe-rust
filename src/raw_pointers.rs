fn main() {

    // Formatters {:b} => Binary, {:x} => Hexadecimal, 
    // {:p} => pointer reference

// -----------------------------------------------------------
// ------- Primitive and reference types' dereferencing -------
// -----------------------------------------------------------

    // printing integer and its reference
    let mut integer = 26 ;
    println!("Value of integer : {}",integer);
    // println!("Reference of &integer : {}\n",&integer);

    //printing string and its integer
    let string = "Hello, wordl!".to_string();
    println!("Value of string : {}",string);
    // println!("Reference of &string : {:?}\n",&string);

// sudo syntax < let variable-name = memory-address as type-of-pointer type>

// -----------------------------------------------------------
// --- Dereferencing of raw pointers from valid reference ----
// -----------------------------------------------------------

    // immutable raw pointer
    let ref1 = &integer as *const i32;
    println!("{:p}", ref1); // accessing pointer reference
    // println!("{}",*ref1); // accessing value
    
    // Notice that derefencing of smart pointers and references don't required-
    // to be done in unsafe block
    
    //mutable raw pointer
    let ref2 = &mut integer as *mut i32;
    println!("{:?}\n", ref2); // accessing pointer reference
    // println!("{}",*ref2); // accessing value


// -----------------------------------------------------------
// ------- Trying changing values using raw pointers ---------
// -----------------------------------------------------------

// ------- First trying using immutable raw pointers ---------

    // Case 1 : Using an immutable raw pointer
    // ref1 = 77;
    // println!("{}",integer);

    // Case 2 : Using an immutable raw pointer by dereferencing
    // *ref1 = 77;
    // println!("{}",integer);

    // Case 3: Same as case 2 but inside unsafe block
    // unsafe { *ref1 = 77; }
    // println!("{}",integer);



    // ------- Next trying using mutable raw pointers ---------

    // Case 1 : Using an mutable raw pointer
    // ref2 = 77;
    // println!("{}",integer);

    // Case 2 : Using an mutable raw pointer by dereferencing
    // *ref2 = 77;
    // println!("{}",integer);

    // Case 3: Same as case 2 but inside unsafe block
    unsafe { *ref2 = 77; }
    println!("{}",integer);



// -----------------------------------------------------------
// Dereferencing of raw pointers whose validity not guaranteed
// -----------------------------------------------------------

    // Picked up a random address of memory
    let address = 0x01234abcusize;
    
    // let raw_ptr_const = address as *const u32;
    // println!("{:?}", *raw_ptr_const);
    // unsafe{ println!("{:?}", *raw_ptr_const); }

    // creating a mutable raw pointer
    // let raw_ptr_mut = address as *mut u32;
    // unsafe{ println!("{:?}", *raw_ptr_mut); }

}



// Link to formatters: https://doc.rust-lang.org/std/fmt/