fn main() {
    // 测试不可变 vector
    let v1 = vec![1, 2, 3];
    let first = v1[0];  //first 为integer，因为刚好v1用的是整型，会直接copy，在v2中验证了
    println!("first of v1: {}", first);
    let other_first = &v1[0];   // other first 为 &{integer}
    println!("other first of v1: {}", other_first);

    // FAILED
    // let v2 = vec![String::from("s1"), String::from("s2")];
    // let first = v2[0];
    // println!("first of v2: {}", first);
    // move occurs because value has type `String`, which does not implement the `Copy` trait

    // 测试可变 vector
    let mut v3 = vec![4, 5, 6];
    // failed
    // help: consider changing this to be a mutable reference: `&mut v3[0]`
    // let first = &v3[0];
    let first = &mut v3[0];
    *first += 10;
    println!("first of v3: {}", first);
}
