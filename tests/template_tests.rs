#[macro_use]
extern crate maplit;

extern crate rxr;
use rxr::template::Template;

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_mapping() -> HashMap<String, String> {
        hashmap!{
            String::from("key_a") => String::from("val_a"),
            String::from("key_b") => String::from("val_b"),
            String::from("key_c") => String::from("val_c"),
        }
    }

    fn get_expansion_items() -> Vec<&'static str> {
        vec!["alpha", "bravo", "charlie", "delta"]
    }

    #[test]
    fn compile_vectors() {
        let template = Template::new("{key_a} : {{$i}} - {{$val}}");
        let mappings = get_mapping();
        let items = get_expansion_items();

        let compiled = template.expand(&items, &mappings).unwrap();

        let expected = vec![
            String::from("val_a : 0 - alpha"),
            String::from("val_a : 1 - bravo"),
            String::from("val_a : 2 - charlie"),
            String::from("val_a : 3 - delta"),
        ];

        assert_eq!(compiled, expected);
    }

    #[test]
    fn compile_vectors_with_math_expression() {
        let template = Template::new("{key_b} : {{$i+1}} - {{$val}}");
        let mappings = get_mapping();
        let items = get_expansion_items();

        let compiled = template.expand(&items, &mappings).unwrap();

        let expected = vec![
            String::from("val_b : 1 - alpha"),
            String::from("val_b : 2 - bravo"),
            String::from("val_b : 3 - charlie"),
            String::from("val_b : 4 - delta"),
        ];

        assert_eq!(compiled, expected);
    }

    #[test]
    fn compile_vectors_with_math_expression_2() {
        let template = Template::new("{key_c} : {{($i+1) * 2}} - {{$val}}");
        let mappings = get_mapping();
        let items = get_expansion_items();

        let compiled = template.expand(&items, &mappings).unwrap();

        let expected = vec![
            String::from("val_c : 2 - alpha"),
            String::from("val_c : 4 - bravo"),
            String::from("val_c : 6 - charlie"),
            String::from("val_c : 8 - delta"),
        ];

        assert_eq!(compiled, expected);
    }
}
