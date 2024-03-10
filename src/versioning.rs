use semver::{BuildMetadata, Prerelease, Version};

pub fn up_major(base: &Version) -> Version {
    Version {
        major: base.major + 1,
        minor: 0,
        patch: 0,
        pre: Prerelease::EMPTY,
        build: BuildMetadata::EMPTY,
    }
}

pub fn up_minor(base: &Version) -> Version {
    Version {
        major: base.major,
        minor: base.minor + 1,
        patch: 0,
        pre: Prerelease::EMPTY,
        build: BuildMetadata::EMPTY,
    }
}

pub fn up_patch(base: &Version) -> Version {
    Version {
        major: base.major,
        minor: base.minor,
        patch: base.patch + 1,
        pre: Prerelease::EMPTY,
        build: BuildMetadata::EMPTY,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_up_major() {
        let before = Version::parse("1.2.3").unwrap();
        let after = up_major(&before);
        assert_eq!(after.major, before.major + 1);
        assert_eq!(after.minor, 0);
        assert_eq!(after.patch, 0);
    }

    #[test]
    fn test_up_minor() {
        let before = Version::parse("1.2.3").unwrap();
        let after = up_minor(&before);
        assert_eq!(after.major, before.major);
        assert_eq!(after.minor, before.minor + 1);
        assert_eq!(after.patch, 0);
    }

    #[test]
    fn test_up_patch() {
        let before = Version::parse("1.2.3").unwrap();
        let after = up_patch(&before);
        assert_eq!(after.major, before.major);
        assert_eq!(after.minor, before.minor);
        assert_eq!(after.patch, before.patch + 1);
    }
}
