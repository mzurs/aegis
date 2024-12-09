pub mod xrc;

impl Default for ManagementCanister {
    fn default() -> Self {
        Self()
    }
}
pub struct ManagementCanister();

impl ManagementCanister {
    pub fn new() -> Self {
        Self::default()
    }
}
