fn main() {
    // new string
    println!("======================== new string");
    // `String::new()`
    let unmutable_string = String::new();
    println!("unmutable string: {}", unmutable_string);
    let mut mutable_string = String::new();
    mutable_string.push_str("foo");
    println!("mutable string: {}", mutable_string);

    // `to_string()` & `String::from()`
    let s1 = "str with UTF-8 你好";
    let s2 = s1.to_string();
    let s3 = String::from("another str with UTF-8 你好");
    println!("s1 after to_string(): {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);

    // update string
    println!("======================== update string");
    // string_var.push_str()
    let mut psh_s = String::new();
    let s = "str to be pushed 你好";
    psh_s.push_str(s);
    println!("psh_s: {}", psh_s);
    println!("s: {}", s);

    // 字符串拼接
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    // println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);

    // `format!`
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);

    let len = String::from("Hola").len();
    let len = String::from("Здравствуйте").len();
    println!("{}", len);

}
