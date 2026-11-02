// Cougr Smart Account: Recovery Request Cleanup (Issue #496)
pub struct RecoveryRequestLedger {
    pub request_active: bool,
    pub approvals_count: u32,
    pub required_threshold: u32,
}

impl RecoveryRequestLedger {
    pub fn execute_and_clear(&mut self) -> Result<(), &'static str> {
        if !self.request_active {
            return Err("No active recovery request found");
        }
        if self.approvals_count < self.required_threshold {
            return Err("Approval count is below required recovery threshold");
        }
        self.request_active = false;
        self.approvals_count = 0;
        Ok(())
    }
}
