use crate::{building::BuildingType, NumberItem};

#[derive(Debug)]
pub struct <Template>;
impl BuildingType for <Template> {
    fn perform_action(&self, contained_numbers: &[NumberItem]) -> Result<Option<NumberItem>, ()> {
        todo!()
    }

    fn get_input_count(&self) -> usize {
        todo!()
    }
}
