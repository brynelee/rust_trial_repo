pub mod struct_demo {

/*
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
*/

    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn area(&self) -> u32 {
            self.height * self.width
        }
    }

    pub fn struct_demo_func(){
        // struct_test demos

        let rect2 = Rectangle {
            height: 20,
            width: 10,
        };

        let area_result2 = rect2.area();

        println!("the rect is {}", area_result2);

    }


}

