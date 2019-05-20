pub mod json;
pub mod parsed_to_render;
mod test;

pub use self::{
    json::{
        composition_to_vec_timed_op, to_json, vec_timed_op_to_vec_op4d, EventType, Op4D, TimedOp,
    },
    parsed_to_render::{
        filename_to_render, r_to_f64, render, sum_vec, to_wav, RenderReturn, RenderType,
    },
};
