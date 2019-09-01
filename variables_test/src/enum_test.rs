pub mod enum_test {

    enum UsState {
       Alabama,
       Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                //println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    // demo of Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    pub fn demo_of_option_t_func() {

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        println!("the six is {:?}", six);
        println!("the none is {:?}", none);

    }

    pub fn demo_of_enum_func(){

        let cents = value_in_cents(Coin::Dime);

        println!("the Dime is {}", cents);

        let cents2 = value_in_cents(Coin::Quarter(UsState::Alabama));

        println!("the Alabama quarter cents are {}", cents2);

    }

}