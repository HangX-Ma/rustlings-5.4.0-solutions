// strings4.rs

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // 字符串字面值就是 slice
    string("red".to_string()); // 转为 String 类型
    string(String::from("hi")); // 相当于 new 了一个 String
    string("rust is fun!".to_owned()); // to_owned() 将 &str 类型转为了 String
    string_slice("nice weather".into()); // 和 from() 是一致的
    string(format!("Interpolation {}", "Station")); // format!() 宏可用来拼接 String
    string_slice(&String::from("abc")[0..1]); // &str
    string_slice("  hello there ".trim()); // trim() 返回的是 &str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // 同理返回类型为 String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // 同上
}
