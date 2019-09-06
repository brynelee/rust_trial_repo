pub mod lifetime_test {

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    pub fn lifetime_test_func1() {
        let string1 = String::from("I'm a long string.");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is: {}", result);
        }
    }

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    pub fn lieftime_test_func2() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };

        println!("the part of the sentence is: {}", i.part);
    }

}
