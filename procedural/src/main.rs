use procedural::stack_string;

fn main() {
    let stack: [u8; 7] = stack_string!("olá❤"); 
	assert_eq!(stack, [111, 108, 195, 161, 226, 157, 164]);
	assert_eq!(str::from_utf8(&stack).unwrap(), "olá❤");
}