// Cougr Smart Account: Session Key Intent Expiry & Collision Prevention (Issue #495)
pub struct SessionKeyIntent {
    pub session_id: [u8; 32],
    pub valid_until_ledger: u32,
}

impl SessionKeyIntent {
    pub fn is_valid(&self, current_ledger: u32) -> bool {
        current_ledger <= self.valid_until_ledger
    }
}
