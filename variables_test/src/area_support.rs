pub mod area {

    fn area_calc(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    pub fn area_demo() {

        let rect1 = (30, 50);

        let area_result = area_calc(rect1);

        println!("calculation result is {}", area_result);
    }

}



