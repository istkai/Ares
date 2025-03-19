use std::collections::HashMap;

#[derive(Default)]
pub struct Form<'a> {
    pub form: HashMap<&'a str, String>,
    pub(crate) target_uri: &'a str
}

impl<'a> Form<'a> {
    pub fn new(form: HashMap<&'a str, String>, target_uri: &'a str) -> Self {
        Form {
            form,
            target_uri
        }
    }
}