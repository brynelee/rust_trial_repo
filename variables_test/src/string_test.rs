pub mod string_slice_demo {

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    pub fn string_and_slice_demo() {

        let my_string = String::from("hello world");

        // first_word 中传入 `String` 的 slice
        let word1 = first_word(&my_string[..]);

        println!("the word is {}", word1);

        let my_string_literal = "hello world";

        // first_word 中传入字符串字面值的 slice
        let word2 = first_word(&my_string_literal[..]);

        println!("the word is {}", word2);

        // 因为字符串字面值 **就是** 字符串 slice，
        // 这样写也可以，即不使用 slice 语法！
        let word3 = first_word(my_string_literal);

        println!("the word is {}", word3);

    }

}