#[derive(Debug)]
pub struct Parameter {
    pub(crate) name: String,
    pub(crate) takes_value: bool,
    pub(crate) description: String,
    pub(crate) alias: Option<String>,
    pub(crate) default: Option<String>,
}

impl Parameter {
    // TODO: Docstrings here
    /// A parameter that takes a value.
    /// Parameters do not have to start with - or --. This is up to the discretion of the user of this library.
    pub fn param(name: &str, description: &str) -> Parameter {
        return Parameter{name: String::from(name), takes_value: true,
            description: String::from(description), alias: None, default: None};
    }

    pub fn flag(name: &str, description: &str) -> Parameter {
        return Parameter{name: String::from(name), takes_value: false,
            description: String::from(description), alias: None, default: None};
    }

    pub fn alias(mut self, alias: &str) -> Parameter {
        self.alias = Some(String::from(alias));
        return self;
    }

    pub fn default<T>(mut self, default: &T) -> Parameter where T: ToString {
        self.default = Some(default.to_string());
        return self;
    }
}
