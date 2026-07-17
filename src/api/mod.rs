// mod.rs : src/api

use crate::macros::declare_and_publish;


declare_and_publish!(evaluate_scalar, evaluate_scalar_eq_approx);
declare_and_publish!(evaluate_vector, evaluate_vector_eq_approx);
declare_and_publish!(margin, margin);
declare_and_publish!(multiplier, multiplier);
declare_and_publish!(zero_margin_or_multiplier, zero_margin_or_multiplier);


// ///////////////////////////// end of file //////////////////////////// //
