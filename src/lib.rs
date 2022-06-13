use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt;

struct StringParts {
    alpha: String,
    numeric: Option<u64>,
    remainder: Option<String>,
}

impl StringParts {
    fn split(s: &str) -> StringParts {
        // find first number
        let index = match s.find(|c: char| c.is_numeric()) {
            Some(n) => n,
            None => {
                return StringParts {
                    alpha: String::from(s),
                    numeric: None,
                    remainder: None,
                };
            }
        };

        let (alpha, num) = s.split_at(index);

        // find end of first part
        let index = match num.find(|c: char| !c.is_numeric()) {
            Some(n) => n,
            None => {
                return StringParts {
                    alpha: String::from(alpha),
                    numeric: Some(num.parse::<u64>().unwrap()),
                    remainder: None,
                };
            }
        };

        let (num, rem) = num.split_at(index);

        StringParts {
            alpha: String::from(alpha),
            numeric: Some(num.parse::<u64>().unwrap()),
            remainder: Some(String::from(rem)),
        }
    }

    fn join(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for StringParts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.numeric {
            Some(n) => match &self.remainder {
                Some(s) => {
                    write!(f, "{}{}{}", self.alpha, n, s)
                }
                None => {
                    write!(f, "{}{}", self.alpha, n)
                }
            },
            None => {
                write!(f, "{}", self.alpha)
            }
        }
    }
}

impl PartialEq for StringParts {
    fn eq(&self, other: &StringParts) -> bool {
        self.join() == other.join()
    }

    fn ne(&self, other: &StringParts) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for StringParts {
    fn partial_cmp(&self, other: &StringParts) -> Option<Ordering> {
        if self.gt(other) {
            Some(Ordering::Greater)
        } else if self.lt(other) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }

    fn lt(&self, other: &StringParts) -> bool {
        !self.ge(other)
    }

    fn le(&self, other: &StringParts) -> bool {
        if self.eq(other) {
            return true;
        }
        self.lt(other)
    }

    fn gt(&self, other: &StringParts) -> bool {
        if self.alpha != other.alpha {
            return self.alpha > other.alpha;
        }

        if self.numeric != other.numeric {
            match self.numeric {
                None => {
                    return false;
                }
                Some(n) => match other.numeric {
                    None => {
                        return true;
                    }
                    Some(n2) => {
                        return n > n2;
                    }
                },
            }
        }

        if self.remainder != other.remainder {
            match &self.remainder {
                None => return false,
                Some(r1) => match &other.remainder {
                    None => {
                        return false;
                    }
                    Some(r2) => {
                        let remainder_self = StringParts::split(&r1);
                        let remainder_other = StringParts::split(&r2);

                        return remainder_self.gt(&remainder_other);
                    }
                },
            }
        } else {
            return false;
        }
    }

    fn ge(&self, other: &StringParts) -> bool {
        if self.eq(other) {
            return true;
        }
        self.gt(other)
    }
}

/// Sorts a vector of &str in a natural way
/// Under the hood it's running `sort_by`
///
/// # Arguments
///
/// * `vals` - A vector of string slices
///
/// # Examples
/// ```
/// use natural_sort::natural_sort;
/// let mut list = vec!["z10a", "b23g", "z999", "z10", "x12z34", "x12z101", "z9", "z3", "z101", "z5"];
/// let expected = vec!["b23g", "x12z34", "x12z101", "z3", "z5", "z9", "z10", "z10a", "z101", "z999"];
/// natural_sort(&mut list);
/// assert_eq!(list, expected);
/// ```
pub fn natural_sort(vals: &mut Vec<&str>) {
    vals.sort_by(|a, b| {
        let sa = StringParts::split(a);
        let sb = StringParts::split(b);

        sa.partial_cmp(&sb).unwrap_or(Ordering::Equal)
    })
}

#[test]
fn test_natural_sort() {
    let mut list = vec![
        "z10a", "b23g", "z999", "z10", "x12z34", "x12z101", "z9", "z3", "z101", "z5",
    ];
    let expected = vec![
        "b23g", "x12z34", "x12z101", "z3", "z5", "z9", "z10", "z10a", "z101", "z999",
    ];

    natural_sort(&mut list);

    assert_eq!(list, expected);
}

#[test]
fn test_partial_ord() {
    fn comp(lhs: &str, rhs: &str) -> Option<Ordering> {
        StringParts::split(lhs).partial_cmp(&StringParts::split(rhs))
    }

    assert_eq!(comp("asdf", "asdf"), Some(Ordering::Equal));
    assert_eq!(comp("asd", "asdf"), Some(Ordering::Less));
    assert_eq!(comp("asff", "asef"), Some(Ordering::Greater));
    assert_eq!(comp("asdf1", "asdf"), Some(Ordering::Greater));

    assert_eq!(comp("123", "123"), Some(Ordering::Equal));
    assert_eq!(comp("123", "321"), Some(Ordering::Less));
    assert_eq!(comp("123", "12"), Some(Ordering::Greater));
    assert_eq!(comp("1", "11"), Some(Ordering::Less));

    assert_eq!(comp("asd123", "asd123"), Some(Ordering::Equal));
    assert_eq!(comp("asd123", "asd124"), Some(Ordering::Less));
    assert_eq!(comp("asd123", "asd122"), Some(Ordering::Greater));
    assert_eq!(comp("asd122", "asd13"), Some(Ordering::Greater));
    assert_eq!(comp("asd122", "asd1111"), Some(Ordering::Less));

    assert_eq!(comp("123a7", "123b1"), Some(Ordering::Less));
    assert_eq!(comp("124a7", "123b1"), Some(Ordering::Greater));
    assert_eq!(comp("a23b7", "a24b8"), Some(Ordering::Less));

    assert_eq!(comp("1", "a"), Some(Ordering::Less));
    assert_eq!(comp("a", "1"), Some(Ordering::Greater));
}
