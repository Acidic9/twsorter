use std::path::PathBuf;

use glob::glob;

pub fn files_from_globs(globs: &[impl AsRef<str>]) -> Vec<PathBuf> {
    globs
        .iter()
        .fold(Vec::with_capacity(globs.len()), |acc, pat| {
            if let Ok(paths) = glob(pat.as_ref()) {
                acc.into_iter()
                    .chain(paths.filter_map(Result::ok))
                    .collect()
            } else {
                acc
            }
        })
}
