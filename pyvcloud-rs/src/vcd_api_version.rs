use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// Representation of a vCloud Director API version.
///
/// Pre-release versions are considered equal if they share the same base
/// version and pre-release label regardless of the numeric suffix. This
/// mirrors the behaviour of the Python `VCDApiVersion` class.
#[derive(Debug, Clone)]
pub struct VcdApiVersion {
    major: u32,
    minor: u32,
    patch: u32,
    original: String,
    pre: Option<String>,
    label: Option<String>,
}

impl VcdApiVersion {
    fn base_cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch))
    }

    fn full_cmp(&self, other: &Self) -> Ordering {
        match self.base_cmp(other) {
            Ordering::Equal => match (&self.pre, &other.pre) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(a), Some(b)) => a.cmp(b),
            },
            o => o,
        }
    }

    /// Return true if this is a pre-release version.
    pub fn is_prerelease(&self) -> bool {
        self.pre.is_some()
    }
}

impl FromStr for VcdApiVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let original = s.to_string();
        let (base, pre) = if let Some(idx) = s.find('-') {
            (&s[..idx], Some(&s[idx + 1..]))
        } else {
            (s, None)
        };
        let mut numbers = base.split('.');
        let major: u32 = numbers.next().ok_or(())?.parse().map_err(|_| ())?;
        let minor: u32 = numbers.next().unwrap_or("0").parse().map_err(|_| ())?;
        let patch: u32 = numbers.next().unwrap_or("0").parse().map_err(|_| ())?;
        let pre_owned = pre.map(|p| p.to_string());
        let label = pre.map(|p| p.split('-').next().unwrap_or(p).to_string());
        Ok(Self {
            major,
            minor,
            patch,
            original,
            pre: pre_owned,
            label,
        })
    }
}

impl fmt::Display for VcdApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.original)
    }
}

impl PartialEq for VcdApiVersion {
    fn eq(&self, other: &Self) -> bool {
        if self.is_prerelease()
            && other.is_prerelease()
            && self.base_cmp(other) == Ordering::Equal
            && self.label == other.label
        {
            true
        } else {
            self.major == other.major
                && self.minor == other.minor
                && self.patch == other.patch
                && self.pre == other.pre
        }
    }
}

impl Eq for VcdApiVersion {}

impl PartialOrd for VcdApiVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VcdApiVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_prerelease()
            && other.is_prerelease()
            && self.base_cmp(other) == Ordering::Equal
            && self.label == other.label
        {
            Ordering::Equal
        } else {
            self.full_cmp(other)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prerelease_label_equality() {
        let a: VcdApiVersion = "37.0.0-alpha-1".parse().unwrap();
        let b: VcdApiVersion = "37.0.0-alpha-2".parse().unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn prerelease_vs_release() {
        let a: VcdApiVersion = "37.0.0-alpha-1".parse().unwrap();
        let b: VcdApiVersion = "37.0.0".parse().unwrap();
        assert!(a < b);
    }

    #[test]
    fn ordering_different_base() {
        let a: VcdApiVersion = "36.0".parse().unwrap();
        let b: VcdApiVersion = "37.0.0-alpha".parse().unwrap();
        assert!(a < b);
    }
}
