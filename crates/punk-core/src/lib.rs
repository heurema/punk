//! Minimal side-effect-free active-core helpers for Punk.
//!
//! This crate exposes deterministic validation and exact-byte hashing helpers.
//! It does not normalize artifact bytes, read files, write schemas, write
//! proofpacks, write gate decisions, expose CLI behavior, or touch `.punk/`
//! runtime state.

use sha2::{Digest as ShaDigest, Sha256};

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const ARTIFACT_HASH_POLICY_VERSION: &str = "artifact-hash-policy.v0.1";
pub const CANONICAL_SHA256_DIGEST_PREFIX: &str = "sha256:";
pub const CANONICAL_SHA256_DIGEST_HEX_LEN: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArtifactDigest(String);

impl ArtifactDigest {
    pub fn new(value: impl Into<String>) -> Result<Self, ArtifactHashPolicyError> {
        let value = value.into();
        validate_artifact_digest(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RepoRelativeArtifactRef(String);

impl RepoRelativeArtifactRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ArtifactHashPolicyError> {
        let value = value.into();
        validate_repo_relative_artifact_ref(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactHashPolicyError {
    EmptyDigest,
    PlaceholderDigest,
    BareDigest,
    UnsupportedDigestAlgorithm,
    InvalidDigestLength { expected: usize, actual: usize },
    InvalidDigestHex,
    EmptyArtifactRef,
    AbsoluteArtifactRef,
    HomeArtifactRef,
    UrlArtifactRef,
    BackslashArtifactRef,
    InvalidArtifactRefSegment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArtifactHashPolicyCapabilities {
    pub validates_digest_format: bool,
    pub validates_repo_relative_refs: bool,
    pub computes_hashes: bool,
    pub normalizes_artifact_bytes: bool,
    pub writes_runtime_state: bool,
}

pub const ARTIFACT_HASH_POLICY_CAPABILITIES: ArtifactHashPolicyCapabilities =
    ArtifactHashPolicyCapabilities {
        validates_digest_format: true,
        validates_repo_relative_refs: true,
        computes_hashes: true,
        normalizes_artifact_bytes: false,
        writes_runtime_state: false,
    };

pub fn is_canonical_artifact_digest(value: &str) -> bool {
    validate_artifact_digest(value).is_ok()
}

pub fn compute_artifact_digest(bytes: &[u8]) -> ArtifactDigest {
    let digest = Sha256::digest(bytes);
    let mut value = String::with_capacity(
        CANONICAL_SHA256_DIGEST_PREFIX.len() + CANONICAL_SHA256_DIGEST_HEX_LEN,
    );
    value.push_str(CANONICAL_SHA256_DIGEST_PREFIX);
    push_lower_hex_bytes(&mut value, digest.iter());

    ArtifactDigest::new(value).expect("computed SHA-256 digest should match artifact hash policy")
}

pub fn validate_artifact_digest(value: &str) -> Result<(), ArtifactHashPolicyError> {
    if value.is_empty() {
        return Err(ArtifactHashPolicyError::EmptyDigest);
    }

    if matches!(value, "unknown" | "pending" | "n/a") {
        return Err(ArtifactHashPolicyError::PlaceholderDigest);
    }

    let Some((algorithm, digest)) = value.split_once(':') else {
        if value.len() == CANONICAL_SHA256_DIGEST_HEX_LEN
            && value.chars().all(|character| character.is_ascii_hexdigit())
        {
            return Err(ArtifactHashPolicyError::BareDigest);
        }

        return Err(ArtifactHashPolicyError::UnsupportedDigestAlgorithm);
    };

    if algorithm != "sha256" {
        return Err(ArtifactHashPolicyError::UnsupportedDigestAlgorithm);
    }

    if digest.len() != CANONICAL_SHA256_DIGEST_HEX_LEN {
        return Err(ArtifactHashPolicyError::InvalidDigestLength {
            expected: CANONICAL_SHA256_DIGEST_HEX_LEN,
            actual: digest.len(),
        });
    }

    if !digest
        .chars()
        .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
    {
        return Err(ArtifactHashPolicyError::InvalidDigestHex);
    }

    Ok(())
}

pub fn is_valid_repo_relative_artifact_ref(value: &str) -> bool {
    validate_repo_relative_artifact_ref(value).is_ok()
}

pub fn validate_repo_relative_artifact_ref(value: &str) -> Result<(), ArtifactHashPolicyError> {
    if value.is_empty() {
        return Err(ArtifactHashPolicyError::EmptyArtifactRef);
    }

    if value.starts_with('/') {
        return Err(ArtifactHashPolicyError::AbsoluteArtifactRef);
    }

    if value.starts_with('~') {
        return Err(ArtifactHashPolicyError::HomeArtifactRef);
    }

    if value.contains('\\') {
        return Err(ArtifactHashPolicyError::BackslashArtifactRef);
    }

    if has_url_scheme(value) {
        return Err(ArtifactHashPolicyError::UrlArtifactRef);
    }

    if value
        .split('/')
        .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
        return Err(ArtifactHashPolicyError::InvalidArtifactRefSegment);
    }

    Ok(())
}

fn push_lower_hex_bytes<'a>(output: &mut String, bytes: impl IntoIterator<Item = &'a u8>) {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
}

fn has_url_scheme(value: &str) -> bool {
    let Some(colon_index) = value.find(':') else {
        return false;
    };
    let first_slash_index = value.find('/').unwrap_or(value.len());

    if colon_index > first_slash_index {
        return false;
    }

    let scheme = &value[..colon_index];

    !scheme.is_empty()
        && scheme
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_alphabetic())
        && scheme.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.')
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_sha256() -> String {
        format!("sha256:{}", "0123456789abcdef".repeat(4))
    }

    #[test]
    fn canonical_sha256_digest_is_valid() {
        let digest = ArtifactDigest::new(valid_sha256()).expect("digest should be valid");

        assert_eq!(
            digest.as_str(),
            "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        );
        assert!(is_canonical_artifact_digest(digest.as_str()));
    }

    #[test]
    fn digest_validation_rejects_unsupported_or_bare_algorithms() {
        let lowercase_bare = "0123456789abcdef".repeat(4);

        assert_eq!(
            validate_artifact_digest(&lowercase_bare),
            Err(ArtifactHashPolicyError::BareDigest)
        );
        assert_eq!(
            validate_artifact_digest(&format!("SHA256:{}", lowercase_bare)),
            Err(ArtifactHashPolicyError::UnsupportedDigestAlgorithm)
        );
        assert_eq!(
            validate_artifact_digest(&format!("sha-256:{}", lowercase_bare)),
            Err(ArtifactHashPolicyError::UnsupportedDigestAlgorithm)
        );
        assert_eq!(
            validate_artifact_digest(&format!("sha512:{}", lowercase_bare)),
            Err(ArtifactHashPolicyError::UnsupportedDigestAlgorithm)
        );
        assert_eq!(
            validate_artifact_digest(&format!("md5:{}", lowercase_bare)),
            Err(ArtifactHashPolicyError::UnsupportedDigestAlgorithm)
        );
    }

    #[test]
    fn digest_validation_rejects_uppercase_hex_and_bad_lengths() {
        assert_eq!(
            validate_artifact_digest(&format!("sha256:{}", "ABCDEF0123456789".repeat(4))),
            Err(ArtifactHashPolicyError::InvalidDigestHex)
        );
        assert_eq!(
            validate_artifact_digest("sha256:decisionhash"),
            Err(ArtifactHashPolicyError::InvalidDigestLength {
                expected: 64,
                actual: 12,
            })
        );
        assert_eq!(
            validate_artifact_digest("sha256:"),
            Err(ArtifactHashPolicyError::InvalidDigestLength {
                expected: 64,
                actual: 0,
            })
        );
    }

    #[test]
    fn digest_validation_rejects_empty_and_placeholder_values() {
        assert_eq!(
            validate_artifact_digest(""),
            Err(ArtifactHashPolicyError::EmptyDigest)
        );
        assert_eq!(
            validate_artifact_digest("unknown"),
            Err(ArtifactHashPolicyError::PlaceholderDigest)
        );
        assert_eq!(
            validate_artifact_digest("pending"),
            Err(ArtifactHashPolicyError::PlaceholderDigest)
        );
        assert_eq!(
            validate_artifact_digest("n/a"),
            Err(ArtifactHashPolicyError::PlaceholderDigest)
        );
    }

    #[test]
    fn repo_relative_artifact_refs_are_valid() {
        for value in [
            "work/reports/2026-04-25-example.md",
            "evals/specs/artifact-hash-policy.v0.1.md",
            ".punk/runs/run_123.json",
        ] {
            let artifact_ref = RepoRelativeArtifactRef::new(value).expect("ref should be valid");

            assert_eq!(artifact_ref.as_str(), value);
            assert!(is_valid_repo_relative_artifact_ref(value));
        }
    }

    #[test]
    fn repo_relative_artifact_ref_rejects_absolute_home_url_and_backslash() {
        assert_eq!(
            validate_repo_relative_artifact_ref("/Users/name/project/work/report.md"),
            Err(ArtifactHashPolicyError::AbsoluteArtifactRef)
        );
        assert_eq!(
            validate_repo_relative_artifact_ref("~/.punk/runs/run_123.json"),
            Err(ArtifactHashPolicyError::HomeArtifactRef)
        );
        assert_eq!(
            validate_repo_relative_artifact_ref("https://example.com/report.md"),
            Err(ArtifactHashPolicyError::UrlArtifactRef)
        );
        assert_eq!(
            validate_repo_relative_artifact_ref("work\\reports\\report.md"),
            Err(ArtifactHashPolicyError::BackslashArtifactRef)
        );
    }

    #[test]
    fn repo_relative_artifact_ref_rejects_empty_dot_and_parent_segments() {
        assert_eq!(
            validate_repo_relative_artifact_ref(""),
            Err(ArtifactHashPolicyError::EmptyArtifactRef)
        );
        assert_eq!(
            validate_repo_relative_artifact_ref("../work/report.md"),
            Err(ArtifactHashPolicyError::InvalidArtifactRefSegment)
        );
        assert_eq!(
            validate_repo_relative_artifact_ref("work//report.md"),
            Err(ArtifactHashPolicyError::InvalidArtifactRefSegment)
        );
        assert_eq!(
            validate_repo_relative_artifact_ref("work/./report.md"),
            Err(ArtifactHashPolicyError::InvalidArtifactRefSegment)
        );
    }

    #[test]
    fn computed_empty_bytes_digest_matches_known_vector() {
        let digest = compute_artifact_digest(b"");

        assert_eq!(
            digest.as_str(),
            "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert!(is_canonical_artifact_digest(digest.as_str()));
    }

    #[test]
    fn computed_known_bytes_digest_matches_known_vector() {
        let digest = compute_artifact_digest(b"abc");

        assert_eq!(
            digest.as_str(),
            "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(validate_artifact_digest(digest.as_str()), Ok(()));
    }

    #[test]
    fn computed_digest_preserves_exact_bytes_without_normalization() {
        let unix_line = compute_artifact_digest(b"line\n");
        let windows_line = compute_artifact_digest(b"line\r\n");
        let trailing_space = compute_artifact_digest(b"line\n ");

        assert_ne!(unix_line, windows_line);
        assert_ne!(unix_line, trailing_space);
        assert_ne!(windows_line, trailing_space);
    }

    #[test]
    fn helper_capabilities_claim_hashing_without_runtime_or_normalization() {
        assert!(ARTIFACT_HASH_POLICY_CAPABILITIES.validates_digest_format);
        assert!(ARTIFACT_HASH_POLICY_CAPABILITIES.validates_repo_relative_refs);
        assert!(ARTIFACT_HASH_POLICY_CAPABILITIES.computes_hashes);
        assert!(!ARTIFACT_HASH_POLICY_CAPABILITIES.normalizes_artifact_bytes);
        assert!(!ARTIFACT_HASH_POLICY_CAPABILITIES.writes_runtime_state);
    }
}
