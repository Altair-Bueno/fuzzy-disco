use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct NewPostPayload <'a> {
    pub(crate) title: &'a str,
    pub(crate) caption : &'a str,
    pub(crate) audio: &'a str,
    pub(crate) photo: &'a str,
    pub(crate) visibility: &'a str,
}
