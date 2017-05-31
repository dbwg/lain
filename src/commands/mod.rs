macro_rules! doc_cmd {
    ($cname:ident, desc => $desc:expr) => {
        #[allow(non_upper_case_globals)]
        pub mod $cname {
            pub static desc: &'static str = $desc;
        }
    };

    ($cname:ident, desc => $desc:expr, usage => $usage:expr) => {
        #[allow(non_upper_case_globals)]
        pub mod $cname {
            pub static desc: &'static str = $desc;
            pub static usage: &'static str = $usage;
        }
    };

    ($cname:ident, desc => $desc:expr, usage => $usage:expr, example => $example:expr) => {
        #[allow(non_upper_case_globals)]
        pub mod $cname {
            pub static desc: &'static str = $desc;
            pub static usage: &'static str = $usage;
            pub static example: &'static str = $example;
        }
    };
}

pub mod meta;
pub mod owner;
