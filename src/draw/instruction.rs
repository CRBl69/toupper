use core::fmt::Debug;
use serde::{Deserialize, Serialize};

pub trait Instruction<'a>
where
    Self: Debug + Send,
{
}
