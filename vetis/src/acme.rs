// Traits and essential types for ACME
pub trait Provider {
    pub fn name(&self) -> &str;
    pub fn create_cert(&self) -> &str;
    pub fn revoke_cert(&self) -> &str;
}
