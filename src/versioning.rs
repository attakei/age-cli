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
