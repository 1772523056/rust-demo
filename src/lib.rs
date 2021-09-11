pub fn add_two(num: i32) -> i32 {
    num + 2
}
#[cfg(test)]
mod tests {
    // #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }




    pub mod a {
        #[derive(Debug)]
        pub struct Rectangle {
            pub length: i32,
            pub width: i32,
        }

        impl Rectangle {
            pub fn test(&self, re: &Rectangle) -> bool {
                println!("value : {:?}", "hkjhkjhkj");
                true
            }
        }
    }

    mod tests {
        use super::*;

        // #[test]
        // #[should_panic(expected = "Guess value must be less than or equal to 100")]
        fn larger_can_hold_smaller() {
            let larger = a::Rectangle { length: 40, width: 40 };
            let smaller = a::Rectangle { length: 10, width: 10 };
            assert!(larger.test(&smaller));
        }
    }
}
