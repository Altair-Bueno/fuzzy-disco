use std::net::IpAddr;
use crate::mongo::user::Sesion;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct PublicSesionData {
    ip: Option<IpAddr>,
    date: String,
}
impl PublicSesionData {
    pub fn new(ip: Option<IpAddr>, date: String) -> Self {
        PublicSesionData { ip, date }
    }
    pub fn from_sesion(sesion:Sesion) -> Self {
        PublicSesionData {
            ip: sesion.ip(),
            date: sesion.date().to_string()
        }
    }
}