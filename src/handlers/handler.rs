use crate::dto::dto::DTO;

pub trait Handler {
    fn handle(&mut self, dto: &mut DTO);
}
