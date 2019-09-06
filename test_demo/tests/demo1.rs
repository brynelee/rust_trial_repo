mod common_mod;

#[test]
fn it_works() {
    common_mod::setup();
    assert_eq!(2 + 2, 4);
}
