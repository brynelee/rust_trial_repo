mod area_support;
mod struct_test;
mod string_test;
mod enum_test;

fn main() {

    area_support::area::area_demo();

    string_test::string_slice_demo::string_and_slice_demo();

    struct_test::struct_demo::struct_demo_func();

    enum_test::enum_test::demo_of_enum_func();

    enum_test::enum_test::demo_of_option_t_func();

}
