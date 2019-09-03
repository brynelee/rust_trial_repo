pub mod vector_demo {

    pub fn v_demo_func() {
        let v: Vec<i32> = Vec::new();

        let v = vec![1, 2, 3];

        println!("v is {:?}", v);

        let mut v1: Vec<i32> = Vec::new();

        v1.push(5);
        v1.push(7);
        v1.push(10);

        println!("v1 is {:?}", v1);
    }

    //引用元素

    pub fn element_retrieve() {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }

    //遍历元素

    pub fn iteration_vector_func() {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
        println!("the vector now is {:?}", v);

    }

    //存储多种类型的元素

    pub fn demo_of_various_type_of_elements_func() {

        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        //println!("the vector of row[0] is {:?}", Some(&row[0]));

    }

}
