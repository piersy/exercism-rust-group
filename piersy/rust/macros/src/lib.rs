// use super::std::collections::HashMap;
// We want to create hashmaps that look like this ('a' => 3, 'b' => 11, 'z' => 32)

#[macro_export]
macro_rules! hashmap {
    ( $($k:expr => $v:expr),*) => {{
        let mut temp_map = ::std::collections::HashMap::new();
        $(
            temp_map.insert($k,$v);
        )*
        temp_map
    }};
    ( $($k:expr => $v:expr),+ ,) => {{
        let mut temp_map = ::std::collections::HashMap::new();
        $(
            temp_map.insert($k,$v);
        )*
        temp_map
    }};
}

//pub fn a(hours: i32, minutes: i32) -> HashMap<i32, i32> {
//    let mut m = HashMap::new();
//    m.insert(hours, minutes);
//    m
//}

//// #[macro_export]
//// macro_rules! vec {
////     ( $( $x:expr ),* ) => {
////         {
////             let mut temp_vec = Vec::new();
////             $(
////                 temp_vec.push($x);
////             )*
////             temp_vec
////         }
////     };
//// }
////
////

// let v = vec!(1,2,3)
#[macro_export]
macro_rules! vec2 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
