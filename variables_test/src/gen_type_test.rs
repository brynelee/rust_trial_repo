pub mod gen_type_test {

    //demo of common function using generic type

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    pub fn gen_type_test_func1() {
        let number_list = vec![34, 50, 25, 100, 65];
        println!("largest number is {}", largest(&number_list));

        let char_list = vec!['y', 'm', 'a', 'q'];
        println!("largest char is {}", largest(&char_list));
    }

    //demo of more generic type function

    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    pub fn gen_type_test_func2() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

}
