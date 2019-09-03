mod area_support;
mod struct_test;
mod string_test;
mod enum_test;
mod vector_test;

fn main() {

    area_support::area::area_demo();

    string_test::string_slice_demo::string_and_slice_demo();

    string_test::string_slice_demo::demo_string_connection_func();

    struct_test::struct_demo::struct_demo_func();

    enum_test::enum_test::demo_of_enum_func();

    enum_test::enum_test::demo_of_option_t_func();

    vector_test::vector_demo::v_demo_func();

    vector_test::vector_demo::element_retrieve();

    vector_test::vector_demo::iteration_vector_func();

}
