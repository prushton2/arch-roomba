//pub struct test {}

pub trait RoombaDisplay {
    fn name(&self) -> String;
    fn status(&self) -> bool;
    fn detail(&self) -> String;
}
