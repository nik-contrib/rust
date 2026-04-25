#![feature(custom_inner_attributes)]
#![feature(stmt_expr_attributes)]

mod foo {
    #![rustfmt::skip]
}

const _: () = {
    #![rustfmt::skip]
};

fn main() {
    #![rustfmt::skip]

    mod inner {
        #![rustfmt::skip]
    }

    fn inner() {
        #![rustfmt::skip]
    }

    {
        #![rustfmt::skip]
    }
}
