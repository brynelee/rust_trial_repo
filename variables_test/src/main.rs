mod area_support;
mod closure_demo;
mod enum_test;
mod gen_type_test;
mod hash_test;
mod lifetime_test;
mod oop_demo;
mod string_test;
mod struct_test;
mod thread_demo;
mod trait_test;
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

    hash_test::hash_demo::hash_demo_func();

    hash_test::hash_demo::hash_demo_func2();

    //generic type demos
    gen_type_test::gen_type_test::gen_type_test_func1();
    gen_type_test::gen_type_test::gen_type_test_func2();

    //trait demo
    trait_test::trait_test::trait_test_func1();

    //lifetime demo
    lifetime_test::lifetime_test::lifetime_test_func1();
    lifetime_test::lifetime_test::lieftime_test_func2();

    //closure demo
    closure_demo::closure_demo_func();
    closure_demo::cacher_demo::closure_demo_with_cacher_func();

    //thread demo
    thread_demo::thread_demo_func();
    thread_demo::thread_demo_func2();
    thread_demo::thread_demo_multi_receiver_func();
    thread_demo::thread_mutex_demo_func();
    
    //oop oop
    oop_demo::oop_demo_func();
}
