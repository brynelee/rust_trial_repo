pub mod trait_test {

    pub fn trait_test_func1() {
        //定义trait
        pub trait Summary {
            //该method由type实现
            fn summarize_author(&self) -> String;
            //该method默认实现，并调用summarize_author
            fn summarize(&self) -> String {
                format!("from {}...", self.summarize_author())
            }
        }

        //定义struct
        pub struct NewsArticle {
            pub headline: String,
            pub author: String,
            pub content: String,
        }

        // struct只需要实现trait中的summarize_author
        impl Summary for NewsArticle {
            fn summarize_author(&self) -> String {
                format!("@{}", self.author)
            }
        }

        let article = NewsArticle {
            headline: String::from("good new !"),
            author: String::from("dorof lin"),
            content: String::from("sth ..."),
        };

        //new article: from @dorof lin...
        println!("new article: {}", article.summarize());
    }
}
