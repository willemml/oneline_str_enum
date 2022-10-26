#[derive(Debug)]
pub enum Error {
    MissingArgument,
    InvalidArgument,
    InvalidVariant,
    EmptyString,
}

#[macro_export]
macro_rules! parseable_enum {
    ($([$($attr:meta),*])? $name:ident, $($variant:ident$(($($param:ident: $type:ident),*))?)*) => {
		$($(#[$attr]) *)?
        pub enum $name {
			$($variant$({$($param: $type),*})?,)*
		}

		impl std::str::FromStr for $name {
			type Err = $crate::Error;

			fn from_str(s: &str) -> Result<Self, Self::Err> {
				let lexed = simple_string_lexer::split_str(s);
				let mut split = lexed.iter();
				if let Some(first) = split.next() {
				match first.as_str() {
					$(stringify!($variant) => {
						$($(
							let $param: $type;
							if let Some(next) = split.next() {
								$param = $type::from_str(next).map_err(|_| $crate::Error::InvalidArgument)?;
							} else {
								return Err($crate::Error::MissingArgument);
							}
						)*)?
						Ok(Self::$variant $({ $($param),*})?)
					}),*
					_ => Err($crate::Error::InvalidVariant),
				}
				} else {
					Err($crate::Error::EmptyString)
				}
			}
		}

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
					match &self {
						$(Self::$variant $({$($param),*})? => {
							write!(f, concat!(stringify!($variant),$($(" {",ident_to_empty_string!($type),"}"),*)?) $(,$($param),*)?)
						}),*
					}
			}
		}
    };
}

#[allow(unused)]
macro_rules! ident_to_empty_string {
    ($type:ident) => {
        ""
    };
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    parseable_enum!([doc = "Test doc.", derive(Debug, PartialEq)] Test, Item1(a:i64) Item2 Item3(b:String) Item4(c:u32, d: String, e:f32));

    #[test]
    fn parse() {
        let string = "Item1 2";
        let parse = Test::from_str(string).expect("Parse failed.");
        assert_eq!(Test::Item1 { a: 2 }, parse);
        let string = "Item2";
        let parse = Test::from_str(string).expect("Parse failed.");
        assert_eq!(Test::Item2, parse);
        let string = "Item3 hello";
        let parse = Test::from_str(string).expect("Parse failed.");
        assert_eq!(
            Test::Item3 {
                b: String::from("hello")
            },
            parse
        );
        let string = "Item3 \"hello person\"";
        let parse = Test::from_str(string).expect("Parse failed");
        assert_eq!(
            Test::Item3 {
                b: String::from("hello person")
            },
            parse
        );
        let string = "Item4 54 testingParser 3.2";
        let parse = Test::from_str(string).expect("Parse failed.");
        assert_eq!(
            Test::Item4 {
                c: 54,
                d: String::from("testingParser"),
                e: 3.2
            },
            parse
        );
        let string = "Item4 54 'this is a test' 3.2";
        let parse = Test::from_str(string).expect("Parse failed");
        assert_eq!(
            Test::Item4 {
                c: 54,
                d: "this is a test".to_string(),
                e: 3.2
            },
            parse
        )
    }

    #[test]
    fn display() {
        let string = Test::Item1 { a: 2 }.to_string();
        assert_eq!(&string, "Item1 2");
        let string = Test::Item2.to_string();
        assert_eq!(&string, "Item2");
        let string = Test::Item3 {
            b: String::from("woohoo!"),
        }
        .to_string();
        assert_eq!(&string, "Item3 woohoo!");
        let string = Test::Item4 {
            c: 54,
            d: String::from("testingParser"),
            e: 3.2,
        }
        .to_string();
        assert_eq!(&string, "Item4 54 testingParser 3.2");
    }

    #[test]
    fn parse_display() {
        let obj = Test::Item4 {
            c: 54,
            d: String::from("testingParser"),
            e: 3.2,
        };
        assert_eq!(Test::from_str(&obj.to_string()).unwrap(), obj);
    }
}
