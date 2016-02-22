use std::ops;
use std::cmp::Ordering;
use std::fmt;
use std::str;
use std::error;

#[derive(Debug, Clone, PartialEq)]
enum DecimalErrorKind {
    Empty,
    InvalidChar(char, u32),
}

impl DecimalErrorKind {
    fn desc(&self) -> String {
        match *self {
            DecimalErrorKind::Empty =>
                "cannot parse decimal from empty string".to_string(),
            DecimalErrorKind::InvalidChar(c, i) => {
                format!("invalid character '{}' found at index {}", c, i)
            }
        }

    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, Hash)]
pub struct Decimal {
    pub unscaled: i64,
    pub scale: u32
}

impl Decimal {
    pub fn new(unscaled: i64, scale: u32) -> Decimal {
        Decimal { unscaled: unscaled, scale: scale }
    }

    /// Add or truncate places to the right of the decimal.
    ///
    /// # Examples
    /// ```
    /// # use decimal::Decimal;
    /// assert_eq!(Decimal::new(1, 0).adjust_scale(2), Decimal::new(100, 2));
    /// assert_eq!(Decimal::new(10, 1).adjust_scale(2), Decimal::new(100, 2));
    /// assert_eq!(Decimal::new(1000, 3).adjust_scale(2), Decimal::new(100, 2));
    /// assert_eq!(Decimal::new(125, 2).adjust_scale(1), Decimal::new(12, 1));
    /// ```
    pub fn adjust_scale(&self, new_scale: u32) -> Decimal {
        match self.scale.cmp(&new_scale) {
            Ordering::Equal => self.clone(),
            Ordering::Greater => Decimal::new(downscale(&self.unscaled, self.scale - new_scale), new_scale),
            Ordering::Less => Decimal::new(upscale(&self.unscaled, new_scale - self.scale), new_scale)
        }
    }

    pub fn abs(self) -> Decimal {
        Decimal { unscaled: self.unscaled.abs(), scale: self.scale }
    }
}

/// `Decimal` is only `PartialOrd`, not `Ord`, because its ordering is not antisymmetric,
/// i.e., two decimals may compare `Ordering::Equal` but not be `==` to one another due
/// to differing scales. However note that all `Decimal`s are comparable, so
/// `partial_cmp` will never return `None`.
///
/// # Examples
/// ```
/// # use decimal::Decimal;
/// let one = Decimal::new(1, 0);
/// let two = Decimal::new(2, 0);
/// assert!(one < two);
/// let two_tenths = Decimal::new(2, 1);
/// assert!(one > two_tenths);
/// let one_point_oh = Decimal::new(10, 1);
/// assert_eq!(::std::cmp::Ordering::Equal, one.partial_cmp(&one_point_oh).unwrap());
/// ```
impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        Some(match self.scale.cmp(&other.scale) {
            Ordering::Equal => self.unscaled.cmp(&other.unscaled),
            Ordering::Greater => self.unscaled.cmp(&upscale(&other.unscaled, self.scale - other.scale)),
            _ => upscale(&self.unscaled, other.scale - self.scale).cmp(&other.unscaled)
        })
    }
}

/// # Examples
///
/// Using `FromStr` directly:
///
/// ```
/// # use decimal::Decimal;
/// use std::str::FromStr;
/// assert_eq!(Decimal::new(12345, 3), FromStr::from_str("12.345").unwrap());
/// ```
/// Using `&str.parse()`:
///
/// ```
/// # use decimal::Decimal;
/// assert_eq!(Decimal::new(-100, 2), "-1.00".parse().unwrap());
/// ```
impl str::FromStr for Decimal {
    type Err = ParseDecimalError;
    fn from_str(s: &str) -> Result<Decimal, ParseDecimalError> {
        let mut unscaled: i64 = 0;
        let mut scale: u32 = 0;
        let mut index: u32 = 0;
        let mut negative = false;
        let mut seen_decimal = false;
        for c in s.chars() {
            match c {
                '-' if index == 0 => negative = true,
                '.' => seen_decimal = true,
                c if c.is_digit(10) => {
                    unscaled = (unscaled * 10) + c.to_digit(10).unwrap() as i64;
                    if seen_decimal {
                        scale += 1;
                    }
                },
                c => return Err(ParseDecimalError::new(DecimalErrorKind::InvalidChar(c, index)))
            }
            index += 1;
        }
        if index == 0 {
            Err(ParseDecimalError::new(DecimalErrorKind::Empty))
        } else {
            Ok(Decimal::new(if negative { -unscaled } else { unscaled }, scale))
        }
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;
        if self.scale == 0 {
            try!(write!(fmt, "{}", self.unscaled));
            return Ok(());
        }
        let mut unscaled_str = format!("{}", self.unscaled);
        if self.unscaled.is_negative() {
            let _ = unscaled_str.remove(0);
            try!(fmt.write_char('-'));
        }
        let unscaled_len = unscaled_str.len() as u32; // assume all chars are 1-byte.
        if self.scale >= unscaled_len {
            try!(fmt.write_char('0'));
            try!(fmt.write_char('.'));
            for _ in 0..(self.scale - unscaled_len) {
                try!(fmt.write_char('0'));
            }
        } else {
            unscaled_str.insert((unscaled_len - self.scale) as usize, '.');
        }
        try!(fmt.write_str(&*unscaled_str));
        Ok(())
    }
}

impl ops::Add for Decimal {
    type Output = Decimal;
    fn add(self, other: Decimal) -> Decimal {
        match self.scale.cmp(&other.scale) {
            Ordering::Equal => Decimal::new(self.unscaled + other.unscaled, self.scale),
            Ordering::Less => self.adjust_scale(other.scale) + other,
            Ordering::Greater => self + other.adjust_scale(self.scale)
        }
    }
}

impl ops::Sub for Decimal {
    type Output = Decimal;
    fn sub(self, other: Decimal) -> Decimal {
        match self.scale.cmp(&other.scale) {
            Ordering::Equal => Decimal::new(self.unscaled - other.unscaled, self.scale),
            Ordering::Less => self.adjust_scale(other.scale) - other,
            Ordering::Greater => self - other.adjust_scale(self.scale)
        }
    }
}

impl ops::Mul for Decimal {
    type Output = Decimal;
    fn mul(self, other: Decimal) -> Decimal {
        Decimal::new(self.unscaled * other.unscaled, self.scale + other.scale)
    }
}

impl ops::Mul<i64> for Decimal {
    type Output = Decimal;
    fn mul(self, i: i64) -> Decimal {
        Decimal::new(self.unscaled * i, self.scale)
    }
}

/// I wasn't sure I could do this, but I can.
/// Apparently it won't be documented though.
impl ops::Mul<Decimal> for i64 {
    type Output = Decimal;
    fn mul(self, d: Decimal) -> Decimal {
        Decimal::new(self * d.unscaled, d.scale)
    }
}

impl ops::Div for Decimal {
    type Output = Decimal;
    fn div(self, other: Decimal) -> Decimal {
        let s = if other.scale > self.scale {
            self.adjust_scale(other.scale)
        } else {
            self
        };
        Decimal::new(s.unscaled / other.unscaled, s.scale - other.scale)
    }
}

impl ops::Rem for Decimal {
    type Output = Decimal;
    fn rem(self, other: Decimal) -> Decimal {
        let s = if other.scale > self.scale {
            self.adjust_scale(other.scale)
        } else {
            self
        };
        Decimal::new(s.unscaled % other.unscaled, s.scale)
    }
}

fn downscale(n: &i64, down_by: u32) -> i64 {
    let mut result = n.clone();
    for _ in 0..down_by {
        result = result / 10;
    }
    result
}

fn upscale(n: &i64, up_by: u32) -> i64 {
    let mut result = n.clone();
    for _ in 0..up_by {
        result = result * 10;
    }
    result
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseDecimalError {
    kind: DecimalErrorKind,
    desc: String,
}

impl ParseDecimalError {
    fn new(kind: DecimalErrorKind) -> Self {
        let desc = kind.desc();
        ParseDecimalError { kind: kind, desc: desc }
    }
}

impl fmt::Display for ParseDecimalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.desc.fmt(f)
    }
}

impl error::Error for ParseDecimalError {
    fn description(&self) -> &str {
        &self.desc
    }
}
