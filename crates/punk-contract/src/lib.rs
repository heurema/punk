//! Minimal contract lifecycle kernel for Punk Phase 3.
//!
//! This crate intentionally stays deterministic and side-effect free in this
//! first implementation step. It models a bounded contract, explicit scope,
//! minimal approval-for-run state, and validation only. No `.punk/` storage,
//! CLI behavior, gate decisions, proofpacks, or run execution are introduced
//! here.

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContractId(String);

impl ContractId {
    pub fn new(value: impl Into<String>) -> Result<Self, ContractError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(ContractError::EmptyContractId);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContractScope {
    refs: Vec<String>,
}

impl ContractScope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_ref(mut self, artifact_ref: impl Into<String>) -> Self {
        let artifact_ref = artifact_ref.into().trim().to_string();

        if !artifact_ref.is_empty() {
            self.refs.push(artifact_ref);
        }

        self
    }

    pub fn refs(&self) -> &[String] {
        &self.refs
    }

    pub fn is_empty(&self) -> bool {
        self.refs.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractStatus {
    Draft,
    ApprovedForRun,
}

impl ContractStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::ApprovedForRun => "approved_for_run",
        }
    }

    pub fn ready_for_bounded_work(self) -> bool {
        matches!(self, Self::ApprovedForRun)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractDraft {
    id: ContractId,
    title: String,
    scope: ContractScope,
}

impl ContractDraft {
    pub fn new(id: ContractId, title: impl Into<String>, scope: ContractScope) -> Self {
        Self {
            id,
            title: title.into(),
            scope,
        }
    }

    pub fn id(&self) -> &ContractId {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn scope(&self) -> &ContractScope {
        &self.scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contract {
    id: ContractId,
    title: String,
    scope: ContractScope,
    status: ContractStatus,
}

impl Contract {
    pub fn id(&self) -> &ContractId {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn scope(&self) -> &ContractScope {
        &self.scope
    }

    pub fn status(&self) -> ContractStatus {
        self.status
    }

    pub fn contract_valid(&self) -> bool {
        !self.title.trim().is_empty() && !self.scope.is_empty()
    }

    pub fn scope_valid(&self) -> bool {
        !self.scope.is_empty()
    }

    pub fn ready_for_bounded_work(&self) -> bool {
        self.contract_valid() && self.status.ready_for_bounded_work()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractError {
    EmptyContractId,
    EmptyTitle,
    EmptyScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractLifecycleBoundary {
    pub implies_final_acceptance: bool,
    pub writes_gate_decision: bool,
    pub creates_proofpack: bool,
    pub requires_runtime_storage: bool,
}

pub const fn contract_lifecycle_boundary() -> ContractLifecycleBoundary {
    ContractLifecycleBoundary {
        implies_final_acceptance: false,
        writes_gate_decision: false,
        creates_proofpack: false,
        requires_runtime_storage: false,
    }
}

pub fn validate_contract(draft: &ContractDraft) -> Result<(), ContractError> {
    if draft.title.trim().is_empty() {
        return Err(ContractError::EmptyTitle);
    }

    if draft.scope.is_empty() {
        return Err(ContractError::EmptyScope);
    }

    Ok(())
}

pub fn approve_contract(draft: ContractDraft) -> Result<Contract, ContractError> {
    validate_contract(&draft)?;

    Ok(Contract {
        id: draft.id,
        title: draft.title,
        scope: draft.scope,
        status: ContractStatus::ApprovedForRun,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_scope() -> ContractScope {
        ContractScope::new()
            .with_ref("work/goals/goal_add_contract_lifecycle_minimal.md")
            .with_ref("docs/product/CONTRACT-TRACKER.md")
    }

    fn valid_draft() -> ContractDraft {
        ContractDraft::new(
            ContractId::new("contract_local_001").expect("contract id should be valid"),
            "Minimal contract lifecycle kernel",
            valid_scope(),
        )
    }

    #[test]
    fn contract_with_empty_scope_is_invalid() {
        let draft = ContractDraft::new(
            ContractId::new("contract_local_002").expect("contract id should be valid"),
            "Scope must be explicit",
            ContractScope::new(),
        );

        assert_eq!(validate_contract(&draft), Err(ContractError::EmptyScope));
    }

    #[test]
    fn contract_with_explicit_scope_is_valid() {
        let draft = valid_draft();

        assert_eq!(validate_contract(&draft), Ok(()));
    }

    #[test]
    fn approved_contract_is_ready_for_bounded_work_but_not_final_acceptance() {
        let contract = approve_contract(valid_draft()).expect("contract should approve");
        let boundary = contract_lifecycle_boundary();

        assert_eq!(contract.status(), ContractStatus::ApprovedForRun);
        assert_eq!(contract.status().as_str(), "approved_for_run");
        assert!(contract.ready_for_bounded_work());
        assert!(!boundary.implies_final_acceptance);
    }

    #[test]
    fn contract_lifecycle_does_not_write_gate_decisions_or_create_proofpacks() {
        let boundary = contract_lifecycle_boundary();

        assert!(!boundary.writes_gate_decision);
        assert!(!boundary.creates_proofpack);
    }

    #[test]
    fn contract_lifecycle_requires_no_runtime_storage() {
        let boundary = contract_lifecycle_boundary();

        assert!(!boundary.requires_runtime_storage);
    }
}
