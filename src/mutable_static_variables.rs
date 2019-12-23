//SCREAMING_CAMEL_CASE

// Case 1: where you need to modify string so often.
const CONNECTION_STRING : &str = "postgres://rajabraza:password@localhost/web_site";
// static CONNECTION_STRING : &str = "postgres://rajabraza:password@localhost/web_site";
static mut VISITORS_COUNT : u32 = 0;

fn main() {

    unsafe { println!("Visitors count = {}",VISITORS_COUNT); }

    // unsafe {
    //     VISITORS_COUNT += 2;
    // }

    increment_count(2);
    unsafe { println!("Visitors count = {}",VISITORS_COUNT); }
    // unsafe {    println!("Counters : {}",VISITORS_COUNT); }
    // for i in 0..10
    // {
    //     println!("Connection String = {}", CONNECTION_STRING);    
    // }


}


fn increment_count(increment_factor : u8){
    unsafe{
        VISITORS_COUNT += increment_factor as u32;
    }
}
