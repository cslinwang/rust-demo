/// demo code for primitive_types
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

pub fn main() {
    let x = 5;
    let y = 6;
    add_with_extra(x, y);
    let _a = 8;
    // dynamic type
    let _b: Vec<f64> = Vec::new();
    let (_a, _c) = ("hi", false);
}
