mod tests;

#[macro_export]
macro_rules! hashmap {
    ( $($($k:expr => $v:expr)+ $(,)?)*) => {
        {
            use ::std::collections::HashMap;
            let mut temp = HashMap::new();
            $(
            $(
                temp.insert($k, $v);
            )+
            )*
            temp
        }
    };
}