const PIE: f32 = 3.14;
static X: i32 = 123;
static mut Y: i32 = 987;

fn main() {
    println!("Hello, {}!", "world");

    let mut a = [1,2,3];
    println!("a = {:?}", a);

    a = [4,5,6];
    println!("a = {:?}", a);

    println!("PIE = {}", PIE);

    println!("safe X = {}", X);
    unsafe {
        Y += 1;
        println!("unsafe Y = {}", Y);
    }
}
