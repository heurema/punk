//! Minimal bounded active-core helpers for Punk.
//!
//! This crate exposes deterministic validation and hashing helpers.
//! It can compute exact-byte digests for caller-provided bytes and for one
//! explicit regular file under one explicit repo root.
//! It does not normalize artifact bytes, write schemas, write proofpacks,
//! write gate decisions, expose CLI behavior, or touch `.punk/` runtime state.

use sha2::{Digest as ShaDigest, Sha256};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RepoRoot(PathBuf);

impl RepoRoot {
    pub fn new(value: impl Into<PathBuf>) -> Result<Self, FileArtifactHashError> {
        let value = value.into();

        if value.as_os_str().is_empty() {
            return Err(FileArtifactHashError::EmptyRepoRoot);
        }

        if !value.is_absolute() {
            return Err(FileArtifactHashError::RelativeRepoRoot);
        }

        Ok(Self(value))
    }

    pub fn as_path(&self) -> &Path {
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
pub enum FileArtifactHashError {
    EmptyRepoRoot,
    RelativeRepoRoot,
    OutsideRepoRoot,
    Missing,
    NotRegularFile,
    SymlinkNotSupported,
    ReadDenied,
    ReadError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArtifactHashPolicyCapabilities {
    pub validates_digest_format: bool,
    pub validates_repo_relative_refs: bool,
    pub computes_hashes: bool,
    pub normalizes_artifact_bytes: bool,
    pub writes_runtime_state: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileArtifactHashingCapabilities {
    pub computes_file_artifact_digests: bool,
    pub reads_regular_files: bool,
    pub follows_symlinks: bool,
    pub scans_directories: bool,
    pub verifies_referenced_artifact_bytes: bool,
    pub normalizes_artifact_bytes: bool,
    pub writes_runtime_state: bool,
    pub requires_runtime_storage: bool,
    pub writes_proofpack: bool,
    pub writes_cli_output: bool,
    pub creates_acceptance_claim: bool,
}

pub const ARTIFACT_HASH_POLICY_CAPABILITIES: ArtifactHashPolicyCapabilities =
    ArtifactHashPolicyCapabilities {
        validates_digest_format: true,
        validates_repo_relative_refs: true,
        computes_hashes: true,
        normalizes_artifact_bytes: false,
        writes_runtime_state: false,
    };

pub const FILE_ARTIFACT_HASHING_CAPABILITIES: FileArtifactHashingCapabilities =
    FileArtifactHashingCapabilities {
        computes_file_artifact_digests: true,
        reads_regular_files: true,
        follows_symlinks: false,
        scans_directories: false,
        verifies_referenced_artifact_bytes: false,
        normalizes_artifact_bytes: false,
        writes_runtime_state: false,
        requires_runtime_storage: false,
        writes_proofpack: false,
        writes_cli_output: false,
        creates_acceptance_claim: false,
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

pub fn compute_artifact_file_digest(
    repo_root: &RepoRoot,
    artifact_ref: &RepoRelativeArtifactRef,
) -> Result<ArtifactDigest, FileArtifactHashError> {
    let artifact_path = artifact_path(repo_root, artifact_ref)?;
    let metadata = fs::symlink_metadata(&artifact_path).map_err(file_artifact_hash_error)?;
    let file_type = metadata.file_type();

    if file_type.is_symlink() {
        return Err(FileArtifactHashError::SymlinkNotSupported);
    }

    if !file_type.is_file() {
        return Err(FileArtifactHashError::NotRegularFile);
    }

    let bytes = fs::read(&artifact_path).map_err(file_artifact_hash_error)?;

    Ok(compute_artifact_digest(&bytes))
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

fn artifact_path(
    repo_root: &RepoRoot,
    artifact_ref: &RepoRelativeArtifactRef,
) -> Result<PathBuf, FileArtifactHashError> {
    let path = repo_root.as_path().join(artifact_ref.as_str());

    if !path.starts_with(repo_root.as_path()) {
        return Err(FileArtifactHashError::OutsideRepoRoot);
    }

    Ok(path)
}

fn file_artifact_hash_error(error: io::Error) -> FileArtifactHashError {
    match error.kind() {
        io::ErrorKind::NotFound => FileArtifactHashError::Missing,
        io::ErrorKind::PermissionDenied => FileArtifactHashError::ReadDenied,
        _ => FileArtifactHashError::ReadError,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::process;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static TEMP_PATH_COUNTER: AtomicU64 = AtomicU64::new(0);

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

    #[test]
    fn file_artifact_digest_hashes_regular_file_exact_bytes() {
        let temp_path = unique_temp_path();
        fs::create_dir_all(temp_path.join("artifacts")).expect("temp repo should be created");
        fs::write(temp_path.join("artifacts/output.bin"), b"abc")
            .expect("artifact file should be written");
        let repo_root = RepoRoot::new(temp_path.clone()).expect("repo root should be valid");
        let artifact_ref =
            RepoRelativeArtifactRef::new("artifacts/output.bin").expect("ref should be valid");

        let digest = compute_artifact_file_digest(&repo_root, &artifact_ref)
            .expect("file digest should compute");

        assert_eq!(digest, compute_artifact_digest(b"abc"));
        assert!(is_canonical_artifact_digest(digest.as_str()));
        assert!(
            !temp_path.join(".punk").exists(),
            "file digest helper must not write runtime state"
        );

        fs::remove_dir_all(temp_path).expect("temp repo should be removed");
    }

    #[test]
    fn file_artifact_digest_preserves_exact_file_bytes_without_normalization() {
        let temp_path = unique_temp_path();
        fs::create_dir_all(temp_path.join("artifacts")).expect("temp repo should be created");
        fs::write(temp_path.join("artifacts/unix.txt"), b"line\n")
            .expect("unix file should be written");
        fs::write(temp_path.join("artifacts/windows.txt"), b"line\r\n")
            .expect("windows file should be written");
        let repo_root = RepoRoot::new(temp_path.clone()).expect("repo root should be valid");
        let unix_ref =
            RepoRelativeArtifactRef::new("artifacts/unix.txt").expect("unix ref should be valid");
        let windows_ref = RepoRelativeArtifactRef::new("artifacts/windows.txt")
            .expect("windows ref should be valid");

        let unix_digest = compute_artifact_file_digest(&repo_root, &unix_ref)
            .expect("unix digest should compute");
        let windows_digest = compute_artifact_file_digest(&repo_root, &windows_ref)
            .expect("windows digest should compute");

        assert_eq!(unix_digest, compute_artifact_digest(b"line\n"));
        assert_eq!(windows_digest, compute_artifact_digest(b"line\r\n"));
        assert_ne!(unix_digest, windows_digest);

        fs::remove_dir_all(temp_path).expect("temp repo should be removed");
    }

    #[test]
    fn file_artifact_digest_reports_missing_and_directories() {
        let temp_path = unique_temp_path();
        fs::create_dir_all(temp_path.join("artifacts/dir")).expect("temp repo should be created");
        let repo_root = RepoRoot::new(temp_path.clone()).expect("repo root should be valid");
        let missing_ref =
            RepoRelativeArtifactRef::new("artifacts/missing.txt").expect("ref should be valid");
        let dir_ref = RepoRelativeArtifactRef::new("artifacts/dir").expect("ref should be valid");

        assert_eq!(
            compute_artifact_file_digest(&repo_root, &missing_ref),
            Err(FileArtifactHashError::Missing)
        );
        assert_eq!(
            compute_artifact_file_digest(&repo_root, &dir_ref),
            Err(FileArtifactHashError::NotRegularFile)
        );

        fs::remove_dir_all(temp_path).expect("temp repo should be removed");
    }

    #[test]
    fn file_artifact_digest_requires_explicit_absolute_repo_root_and_valid_refs() {
        assert_eq!(
            RepoRoot::new(PathBuf::new()),
            Err(FileArtifactHashError::EmptyRepoRoot)
        );
        assert_eq!(
            RepoRoot::new("relative/repo"),
            Err(FileArtifactHashError::RelativeRepoRoot)
        );
        assert_eq!(
            RepoRelativeArtifactRef::new("../outside.txt"),
            Err(ArtifactHashPolicyError::InvalidArtifactRefSegment)
        );
        assert_eq!(
            RepoRelativeArtifactRef::new("/absolute/outside.txt"),
            Err(ArtifactHashPolicyError::AbsoluteArtifactRef)
        );
    }

    #[test]
    fn file_artifact_hashing_capabilities_stay_non_authoritative() {
        let capabilities = FILE_ARTIFACT_HASHING_CAPABILITIES;

        assert!(capabilities.computes_file_artifact_digests);
        assert!(capabilities.reads_regular_files);
        assert!(!capabilities.follows_symlinks);
        assert!(!capabilities.scans_directories);
        assert!(!capabilities.verifies_referenced_artifact_bytes);
        assert!(!capabilities.normalizes_artifact_bytes);
        assert!(!capabilities.writes_runtime_state);
        assert!(!capabilities.requires_runtime_storage);
        assert!(!capabilities.writes_proofpack);
        assert!(!capabilities.writes_cli_output);
        assert!(!capabilities.creates_acceptance_claim);
    }

    #[cfg(unix)]
    #[test]
    fn file_artifact_digest_reports_symlinks_without_following() {
        use std::os::unix::fs::symlink;

        let temp_path = unique_temp_path();
        fs::create_dir_all(temp_path.join("artifacts")).expect("temp repo should be created");
        fs::write(temp_path.join("artifacts/target.txt"), b"secret target")
            .expect("target file should be written");
        symlink("target.txt", temp_path.join("artifacts/link.txt"))
            .expect("symlink should be created");
        let repo_root = RepoRoot::new(temp_path.clone()).expect("repo root should be valid");
        let link_ref =
            RepoRelativeArtifactRef::new("artifacts/link.txt").expect("ref should be valid");

        assert_eq!(
            compute_artifact_file_digest(&repo_root, &link_ref),
            Err(FileArtifactHashError::SymlinkNotSupported)
        );

        fs::remove_dir_all(temp_path).expect("temp repo should be removed");
    }

    fn unique_temp_path() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        let counter = TEMP_PATH_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "punk-core-file-hash-tests-{}-{}-{}",
            process::id(),
            unique,
            counter
        ))
    }
}
