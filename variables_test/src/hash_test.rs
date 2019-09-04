pub mod hash_demo {

    use std::collections::HashMap;

    pub fn hash_demo_func() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // 遍历得到 K/V
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    //基于 or_insert 方法返回的 &mut
    //通过 &mut 来更新现有的 V
    //但注意需通过 * 来 dereference

    pub fn hash_demo_func2() {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            // word 存在时得到现有count
            // word 不存在时得到初始 count(0)
            let count = map.entry(word).or_insert(0);
            // 通过 * 来 dereference 并更新
            *count += 1;
        }

        println!("the hashmap now is {:?}", map);
    }
}
