use std::collections::HashSet;
use std::io;
use std::path::PathBuf;

use typship::utils::walkers::{walker_install, walker_publish};

fn walker_test_path() -> PathBuf {
    <&str as Into<PathBuf>>::into(file!())
        .parent()
        .unwrap()
        .join("walker_test")
}

fn test_files(
    files: Vec<&str>,
    walker: Vec<Result<ignore::DirEntry, ignore::Error>>,
) -> io::Result<()> {
    println!("Files: {:?}", files);
    println!("Walker: {:?}", walker);

    let mut files: HashSet<String> = files
        .into_iter()
        .map(|s| walker_test_path().join(s).to_string_lossy().into_owned())
        .collect();
    files.insert(walker_test_path().to_string_lossy().into_owned());

    for f in walker {
        assert!(f.is_ok(), "Walker dir not ok");
        let f = f.unwrap();
        let path = f.path().to_string_lossy().into_owned();
        assert!(files.contains(&path), "File {} not in the list", path);
        files.remove(&path);
    }

    assert!(
        files.is_empty(),
        "Not all files are walked: missing {:?}",
        files
    );

    Ok(())
}

#[test]
fn test_ignore() -> io::Result<()> {
    test_files(
        ["src", "src/lib.typ", "excludes_test.txt", "typst.toml"].into(),
        walker_publish(&walker_test_path()).collect(),
    )
}

#[test]
fn test_exclude() -> io::Result<()> {
    let walker = walker_install(&walker_test_path())
        .map_err(|_| panic!("Walker not ok"))
        .unwrap();
    test_files(
        ["src", "src/lib.typ", "typst.toml"].into(),
        walker.into_iter().collect(),
    )
}
