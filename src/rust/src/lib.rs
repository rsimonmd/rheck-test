use extendr_api::prelude::*;
use heck::ToSnekCase;

#[extendr]
fn to_snek_case(x: Strings) -> Strings {
    x.into_iter()
        .map(|xi| match xi.is_na() {
            true => Rstr::na(),
            false => Rstr::from(xi.as_str().to_snek_case()),
        })
        .collect::<Strings>()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod heck;
    fn to_snek_case;
}
