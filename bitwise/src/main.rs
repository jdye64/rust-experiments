const SECURITY_CHECK_FLAG: u8 = 0b00000001;
const TWO_FACTOR_AUTH_FLAG: u8 = 0b00000010;
const JUST_A_FLAG_FLAG: u8 = 0b00000100;

fn main() {
    let a:i64 = 2;     // Bit presentation 10
    let b:i64 = 3;     // Bit presentation 11

    println!("{}", format!("A -> 2 as binary: {a:#066b}"));
    println!("{}", format!("B -> 3 as binary: {b:#066b}"));
 
    let mut result:i64;
 
    result = a & b;
    println!("(a & b) => {} Binary => {:#066b}", result, result);
 
    result = a | b;
    println!("(a | b) => {} Binary => {:#066b}", result, result);
 
    result = a ^ b;
    println!("(a ^ b) => {} Binary => {:#066b}", result, result);
 
    result = !b;
    println!("(!b) => {} Binary => {:#066b}", result, result);
 
    result = a << b;
    println!("(a << b) => {} Binary => {:#066b}", result, result);
 
    result = a >> b;
    println!("(a >> b) => {} Binary => {:#066b}", result, result);


    // Create flag using bitwise operations
    let mut flags: u8 = 0;
    println!("SizeOf u8: {} byte(s)", std::mem::size_of::<u8>());
    println!("SizeOf usize: {} byte(s)", std::mem::size_of::<usize>());

    println!("Starting Flags => {:#010b}", flags);

    // Enable security check flag
    flags = flags | SECURITY_CHECK_FLAG;

    println!("Flags for security check => {:#010b}", flags);

    flags = flags | TWO_FACTOR_AUTH_FLAG;

    println!("Security and two-factor auth => {:#010b}", flags);

    // Remove security. This is "invalid" can't have two-factor auth enabled but not security in general add checks
    flags = flags ^ SECURITY_CHECK_FLAG; // left with just two-factor auth enabled

    println!("two-factor auth NO security => {:#010b}", flags);

    let valid_security_flag: u8 = 0b00000011;

    // Ensure that the correct combination of bits are enabled with XOR
    if flags ^ valid_security_flag != 0 {
        println!("Security must be enabled for two-factor auth to be enabled!");
    }
 }
