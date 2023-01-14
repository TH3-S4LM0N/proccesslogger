// **C**SystemExt for **Crate**SystemExt to
// diff from sysinfo::SystemExt
pub trait CSystemExt {
    fn clone(&self) -> Self {
        *self
    }
}
impl CSystemExt for sysinfo::System {
    fn clone(&self) -> Self {
        *self
    }
}
