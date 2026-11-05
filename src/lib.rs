#[derive(Debug, Clone)]
pub struct ReleaseArtifact {
    pub artifact_name: String,
    pub sha256_checksum: String,
    pub target_arch: String,
}

#[derive(Debug, Default)]
pub struct ReleaseToolchain {
    artifacts: Vec<ReleaseArtifact>,
}

impl ReleaseToolchain {
    pub fn new() -> Self { Self::default() }
    pub fn register_artifact(&mut self, name: &str, sha: &str, arch: &str) {
        self.artifacts.push(ReleaseArtifact {
            artifact_name: name.to_string(),
            sha256_checksum: sha.to_string(),
            target_arch: arch.to_string(),
        });
    }
    pub fn has_artifact(&self, name: &str) -> bool {
        self.artifacts.iter().any(|a| a.artifact_name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_release_toolchain_artifact_registration() {
        let mut tc = ReleaseToolchain::new();
        tc.register_artifact("sigmaos-2026.11-x86_64.iso", "e3b0c442", "x86_64");
        assert!(tc.has_artifact("sigmaos-2026.11-x86_64.iso"));
    }
}
