use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Draft,
    Finished,
    Approved,
    Locked,
}
