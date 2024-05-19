use std::path::Path;

/// True if the path goes above the current directory.
pub fn path_spills_up<PathRef: AsRef<Path>>(path: PathRef) -> bool {
    let mut depth = 0;
    for component in path.as_ref().components() {
        match component {
            std::path::Component::Normal(_) => depth += 1,
            std::path::Component::ParentDir => depth -= 1,
            _ => {}
        }
        if depth < 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn confinement() {
        assert_eq!(path_spills_up("one"), false);
        assert_eq!(path_spills_up("one/two"), false);
        assert_eq!(path_spills_up("/one/two"), false);
        assert_eq!(path_spills_up("../one"), true);
        assert_eq!(path_spills_up("../../one"), true);
        assert_eq!(path_spills_up("one/../two/.."), false);
        assert_eq!(path_spills_up("one/../two/../.."), true);
        assert_eq!(path_spills_up("/../../one"), true);
    }
}
