use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Deref},
    str::FromStr,
};

use ecow::EcoString;

#[derive(Default)]
pub struct SecretString(EcoString);

impl Deref for SecretString {
    type Target = EcoString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for SecretString {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<EcoString> for SecretString {
    fn from(value: EcoString) -> Self {
        Self(value)
    }
}

impl From<&str> for SecretString {
    fn from(value: &str) -> Self {
        Self::from(EcoString::from(value))
    }
}

impl FromStr for SecretString {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl Clone for SecretString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"********\"")
    }
}

impl Display for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Add for SecretString {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for SecretString {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0.push_str(rhs.as_str());
    }
}

impl Add<&str> for SecretString {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: &str) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<&str> for SecretString {
    #[inline]
    fn add_assign(&mut self, rhs: &str) {
        self.0.push_str(rhs);
    }
}
