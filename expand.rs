#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[cfg(test)]
mod tests {
    use forms::form;
    use futures_signals::signal::Mutable;
    pub struct Field<T> {
        pub label: String,
        pub value: Mutable<T>,
        pub error: Mutable<Option<String>>,
        pub regex: Option<String>,
        pub required: bool,
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::it_works"]
    pub const it_works: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::it_works"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/main.rs",
            start_line: 15usize,
            start_col: 8usize,
            end_line: 15usize,
            end_col: 16usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(it_works())),
    };
    fn it_works() {
        let data = Field::<String> {
            label: "First Name",
            value: futures_signals::signal::Mutable::new(),
            error: futures_signals::signal::Mutable::new(),
            regex: ".{3,}",
            required: true,
        };
    }
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&it_works])
}
