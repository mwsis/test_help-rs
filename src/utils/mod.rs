// mod.rs : src/utils

use crate::macros::declare_and_publish;


declare_and_publish!(crate
    compare,
    compare_approximate_equality_by_margin,
    compare_approximate_equality_by_multiplier,
    compare_approximate_equality_by_zero_margin_or_multiplier,
);


// ///////////////////////////// end of file //////////////////////////// //
