use crate::dto::dto::DTO;

pub trait Handler<'a> {
    fn handle(&mut self, dto: &mut DTO<'a>);
}
