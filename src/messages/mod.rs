use vizia::prelude::*;
use serde::{Deserialize, Serialize};


pub trait MessageTrait<'a>: Sized + Deserialize<'a> + Serialize {
    fn from_msg(msg: &'a str) -> Self {
        ron::from_str(msg).unwrap()
    }

    fn to_msg(&'a self) -> String {
        ron::to_string(self).unwrap()
    }

    fn from_bytes(bytes: &'a [u8]) -> Self {
        println!("DBG: {:?}", String::from_utf8(bytes.to_vec()).unwrap());
        ron::de::from_bytes(bytes).unwrap()
    }

    fn to_bytes(&'a self) -> Vec<u8> {
        self.to_msg().into_bytes()
    }
}


#[derive(Deserialize, Serialize, Debug, Clone, Data, Lens)]
pub struct UserMsg {
    pub username: String,
    pub message: String,
}

impl MessageTrait<'_> for UserMsg {}
