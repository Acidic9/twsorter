pub struct TwClass<'t> {
    pub class: &'t str,
    pub state: Option<&'t str>,
}

impl<'t> TwClass<'t> {
    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }
}

impl<'t> From<&'t str> for TwClass<'t> {
    fn from(from_str: &'t str) -> Self {
        let mut parts = from_str.splitn(2, ":");
        let head = parts.next().unwrap_or_default();
        let tail = parts.next();
        if let Some(tail) = tail {
            Self {
                class: tail,
                state: Some(head),
            }
        } else {
            Self {
                class: head,
                state: None,
            }
        }
    }
}
