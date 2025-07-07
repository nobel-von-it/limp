use crate::{error::LimpResult, models::execute::Actor};

impl Actor {
    pub fn init(&mut self, name: String, deps: Option<Vec<String>>) -> LimpResult<()> {
        Ok(())
    }
}
