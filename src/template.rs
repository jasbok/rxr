use regex::Regex;

use eval::{Expr, Value};

use std::collections::HashMap;
use std::error::Error;

pub struct Template<'t> {
    template: &'t str,
    substitutions: Vec<&'t str>,
    expansions: Vec<&'t str>,
}

impl<'t, 'm> Template<'t> {
    pub fn new(template: &'t str) -> Template<'t> {
        lazy_static! {
            static ref RE_SUBSTITUTION : Regex = Regex::new("\\{[_a-zA-Z0-9]*?\\}").unwrap();
            static ref RE_EXPANSION : Regex = Regex::new("\\{{2}.*?\\}{2}").unwrap();
        }

        Template {
            template: template,
            substitutions: RE_SUBSTITUTION
                .captures_iter(template)
                .map(|cap| cap.get(0).unwrap().as_str())
                .collect(),
            expansions: RE_EXPANSION
                .captures_iter(template)
                .map(|cap| cap.get(0).unwrap().as_str())
                .collect(),
        }
    }

    pub fn substitute(&self, map: &HashMap<String, String>) -> Result<String, Box<Error>> {
        let mut substitute = String::from(self.template);

        for cap in &self.substitutions {
            let expr = &cap[1..cap.len() - 1];

            if let Some(mapping) = map.get(expr) {
                substitute = substitute.replace(cap, mapping);
            } else {
                let err = TemplateError::SubstitutionNoFound {
                    key: String::from(expr),
                };

                return Err(Box::new(err));
            }
        }

        Ok(substitute)
    }

    pub fn expand<T>(
        &self,
        input: &[T],
        map: &HashMap<String, String>,
    ) -> Result<Vec<String>, Box<Error>>
    where
        T: ToString,
    {
        let mut res = Vec::new();
        let template = self.substitute(map)?;

        for (i, val) in input.iter().map(|val| val.to_string()).enumerate() {
            let mut template = template.clone();

            for cap in &self.expansions {
                let expr = Expr::new(&cap[2..cap.len() - 2])
                    .value("$i", i)
                    .value("$val", &val)
                    .exec()?;

                let value = match expr {
                    Value::Number(val) => Some(val.to_string()),
                    Value::String(val) => Some(String::from(val)),
                    _ => None,
                };

                if let Some(value) = value {
                    template = template.replace(cap, &value);
                } else {
                    unimplemented!();
                }
            }

            res.push(template);
        }

        Ok(res)
    }
}

error_def! TemplateError {
    SubstitutionNoFound { key: String }
        => "Variant with args" ("This is a format string. flim is {}", key ),
}
