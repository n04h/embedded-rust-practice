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
    println!("PIE = {:.1}", PIE); // 小数点以下桁数を指定

    println!("safe X = {}", X);
    unsafe {
        Y += 1;
        println!("unsafe Y = {}", Y);
    }

    let ary: [i32; 5] = [1, 2, 3, 4, 5];
    println!("ary = {:?}", ary);
    println!("ary[2] = {}", ary[2]);

    let slice_ary: &[i32] = &ary[0..3]; // aryを参照しているだけでslice_aryが値を持っているわけではない
    println!("slice_ary = {:?}", slice_ary);

    // なので可変にしてスライスしたものを変更すると参照先の内容も変わる
    let mut mut_ary = [1, 2, 3, 4];
    let slice_mut_ary = &mut mut_ary[1..3];

    slice_mut_ary[0] = 9;

    println!("slice_mut_ary = {:?}", slice_mut_ary);
    println!("mut_ary = {:?}", mut_ary);

    // タプル型
    let t: (u8, i32, usize) = (1, -42, 1000);
    println!("t.0 = {}", t.0);
    println!("t.1 = {}", t.1);
    println!("t.2 = {}", t.2);
}
