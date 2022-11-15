use std::path::Path;

use crate::with_paths;

#[test]
fn test_with_paths() {
    let p1 = "hello";
    let p2 = "world";
    let p3 = "some/other/path";
    with_paths! {
        path = p1 => assert_eq!(path, Path::new(p1))
    }

    let result = Path::new(p1).join(p2);
    with_paths! {
        path = p1 / p2
        => assert_eq!(path, result)
    }

    let result = Path::new(p1).join(p2).join(p3);
    with_paths! {
        path = p1 / p2 / p3
        => assert_eq!(path, result)
    }
}

#[test]
fn test_with_paths_funky() {
    let p1 = "hello//";
    let p2 = "/world/";
    let p3 = "some/other/path//";
    with_paths! {
        path = p1 => assert_eq!(path, Path::new(p1))
    }

    let result = Path::new(p1).join(p2);
    with_paths! {
        path = p1 / p2
        => assert_eq!(path, result)
    }

    let result = Path::new(p1).join(p2).join(p3);
    with_paths! {
        path = p1 / p2 / p3
        => assert_eq!(path, result)
    }
}

#[test]
fn test_with_paths_funky_2() {
    let p1 = "hello//";
    let p2 = "/world/";
    let p3 = "some/other/path";
    let p4 = "/actual_root"; // p4 should be selected as the actual root path

    let result = Path::new(p1).join(p2).join(p3).join(p4);
    with_paths! {
        path = p1 / p2 / p3 / p4
        => assert_eq!(path, result)
    }
}
