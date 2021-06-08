use std::{cmp::Ordering, path::Path};

use anyhow::Result;
use regex::Captures;
use tokio::fs::{read_to_string, write};

use crate::{
    config::{pattern::Pattern, Config},
    sort::tw_class::TwClass,
};

mod tw_class;

pub fn sort_classes(
    classes_order: &[impl AsRef<str>],
    states_order: &[impl AsRef<str>],
    classes_str: &str,
) -> String {
    // Preserve prefixing and suffixing whitespace (yes I made those words up)
    let whitespace_before = classes_str
        .chars()
        .take_while(|c| c.is_whitespace())
        .count();
    let whitespace_after = classes_str
        .chars()
        .rev()
        .take_while(|c| c.is_whitespace())
        .count();

    let mut classes: Vec<_> = classes_str
        .split(' ')
        .map(str::trim)
        .filter(|class| !class.is_empty())
        .collect();

    classes.sort_by(|a, b| {
        let a_class: TwClass = (*a).into();
        let b_class: TwClass = (*b).into();

        let a_class_index = classes_order
            .iter()
            .position(|c| c.as_ref() == a_class.class);
        let b_class_index = classes_order
            .iter()
            .position(|c| c.as_ref() == b_class.class);

        let a_state_index = a_class
            .state
            .and_then(|state| states_order.iter().position(|s| s.as_ref() == state));
        let b_state_index = b_class
            .state
            .and_then(|state| states_order.iter().position(|s| s.as_ref() == state));

        // A or B have unknown selector
        if a_class_index.is_some() && b_class_index.is_none() {
            // B has unknown class
            return Ordering::Greater;
        }
        if a_class_index.is_none() && b_class_index.is_some() {
            // A has unknown class
            return Ordering::Less;
        }

        // Sort by media query
        if !a_class.has_state() && b_class.has_state() {
            return Ordering::Less;
        }
        if a_class.has_state() && !b_class.has_state() {
            return Ordering::Greater;
        }

        // Both or none have a state at this point
        if let (Some(a_state_index), Some(b_state_index)) = (a_state_index, b_state_index) {
            if a_state_index < b_state_index {
                return Ordering::Less;
            }
            if a_state_index > b_state_index {
                return Ordering::Greater;
            }
        }

        // Sort based on sorted selector
        if let (Some(a_class_index), Some(b_class_index)) = (a_class_index, b_class_index) {
            if a_class_index < b_class_index {
                return Ordering::Less;
            }
            if a_class_index > b_class_index {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    });

    format!(
        "{}{}{}",
        " ".repeat(whitespace_before),
        classes.join(" "),
        " ".repeat(whitespace_after)
    )
}

pub async fn sort_file(
    path: impl AsRef<Path>,
    config: &Config,
    classes_order: &[impl AsRef<str>],
    states_order: &[impl AsRef<str>],
) -> Result<()> {
    if !path.as_ref().is_file() {
        return Ok(());
    }

    println!("Formatting file: {}", path.as_ref().display());

    let mut content = read_to_string(path.as_ref()).await?;

    for Pattern(ref ex) in &config.patterns {
        content = ex
            .replace_all(&content, |caps: &Captures| {
                let mut iter = caps
                    .iter()
                    .map(|m| m.map(|m| (m.start(), m.end())).unwrap_or((0, 0)));
                let before_range = iter.next().unwrap_or((0, 0));
                let classes_range = iter.next().unwrap_or((0, 0));
                let classes_start = classes_range.0 - before_range.0;
                let classes_end = classes_range.1 - classes_range.0;

                let capture_str = caps.get(0).unwrap().as_str();
                let before = &capture_str[0..classes_start];
                let classes = &capture_str[classes_start..classes_start + classes_end];
                let after = &capture_str[classes_start + classes_end..];

                let sorted_classes = sort_classes(classes_order, states_order, classes);

                [before, &sorted_classes, after].concat()
            })
            .to_string();
    }

    write(path.as_ref(), content).await?;

    Ok(())
}
