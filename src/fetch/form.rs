use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Form<'a> {
    pub form: HashMap<String, String>,
    pub(crate) target_uri: &'a str,
}

impl<'a> Form<'a> {
    pub fn new(form: HashMap<String, String>, target_uri: &'a str) -> Self {
        Form { form, target_uri }
    }
}
