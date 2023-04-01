// Tests provided by exercism.org
#[cfg(test)]
mod tests {
    use crate::hashmap;
    use std::collections::HashMap;
    #[test]
    fn test_empty() {
        let expected: HashMap<u32, u32> = HashMap::new();
        let computed: HashMap<u32, u32> = hashmap!();
        assert_eq!(computed, expected);
    }
    #[test]
    fn test_single() {
        let mut expected = HashMap::new();
        expected.insert(1, "one");
        assert_eq!(hashmap!(1 => "one"), expected);
    }
    #[test]
    fn test_no_trailing_comma() {
        let mut expected = HashMap::new();
        expected.insert(1, "one");
        expected.insert(2, "two");
        assert_eq!(hashmap!(1 => "one", 2 => "two"), expected);
    }
    #[test]
    fn test_trailing_comma() {
        let mut expected = HashMap::new();
        expected.insert('h', 89);
        expected.insert('a', 1);
        expected.insert('s', 19);
        expected.insert('h', 8);
        assert_eq!(
            hashmap!(
            'h' => 89,
            'a' => 1,
            's' => 19,
            'h' => 8,
        ),
            expected
        );
    }
    #[test]
    fn test_nested() {
        let mut expected = HashMap::new();
        expected.insert("non-empty", {
            let mut subhashmap = HashMap::new();
            subhashmap.insert(23, 623);
            subhashmap.insert(34, 21);
            subhashmap
        });
        expected.insert("empty", HashMap::new());
        assert_eq!(
            hashmap!(
            "non-empty" => hashmap!(
                23 => 623,
                34 => 21
            ),
            "empty" => hashmap!()
        ),
            expected
        );
    }
    mod test {
        #[test]
            fn type_not_in_scope() {
            use crate::hashmap;
            let _empty: ::std::collections::HashMap<(), ()> = hashmap!();
            let _without_comma = hashmap!(23=> 623, 34 => 21);
            let _with_trailing = hashmap!(23 => 623, 34 => 21,);
        }
        #[test]
            fn test_macro_out_of_scope() {
            let _empty: ::std::collections::HashMap<(), ()> = crate::hashmap!();
            let _without_comma = crate::hashmap!(23=> 623, 34 => 21);
            let _with_trailing = crate::hashmap!(23 => 623, 34 => 21,);
        }
    }
    #[test]
    fn test_type_override() {
        // The macro should always use std::collections::HashMap and ignore crate::std::collections::HashMap
        mod std {
            pub mod collections {
                pub struct HashMap;
                impl HashMap {
                    #[allow(dead_code)]
                    pub fn new() -> Self {
                        panic!("Do not allow users to override which HashMap is used");
                    }
                    #[allow(dead_code)]
                    pub fn insert<K, V>(&mut self, _key: K, _val: V) {
                        panic!("Do not allow users to override which HashMap is used");
                    }
                }
            }
        }
        let _empty: ::std::collections::HashMap<(), ()> = hashmap!();
        let _without_comma = hashmap!(1 => 2, 3 => 4);
        let _with_trailing = hashmap!(1 => 2, 3 => 4,);
    }
    #[test]
    fn test_multi_args_multi_comma() {
        use crate::hashmap;
        hashmap!(1 => 2, 2 => 3,);
    }
}