//! Functionality for converting raw ingredient strings into structured data
//! with canonical units (ideally SI units, with some exceptions).
//!
//! For example, the string "1/2 cup sugar" should be parsed into an
//! ingredient with name "sugar", amount "1/2", and unit "cup"; and
//! formatted as "142g sugar" (since 1 cup is 284g).
//! TODO: figure out how to determine whether the 'cup' refers to volume or mass...
//! Maybe just hardcode some common ingredients, their state, and their densities?

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("empty ingredient")]
    Empty,
    #[error("no regex match")]
    NoMatch(ScrapedIngredient),
    #[error("parsing amount from {0}")]
    ParsingAmount(String),
    #[error("parsing unit from {0}")]
    ParsingUnit(String),
    #[error("couldn't parse amount and unit {0}")]
    AmountAndUnit(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrapedIngredient {
    pub raw: String,
    pub name: Option<String>,
    pub amount: Option<f64>,
    pub unit: Option<Unit>,
    pub instructions: Option<String>,
}

impl ScrapedIngredient {
    pub fn canonicalize(&self) -> Option<(f64, Unit)> {
        match (self.amount, &self.unit) {
            (Some(amount), Some(unit)) => {
                Some((amount * unit.canonicalize(), unit.canonical_unit()))
            }
            _ => None,
        }
    }
}

impl fmt::Display for ScrapedIngredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.canonicalize(), &self.name) {
            (Some((mut amount, mut unit)), Some(name)) => {
                if amount < 1.0 && unit.smaller_prefix().is_some() {
                    amount *= 1000.0;
                    unit = unit.smaller_prefix().unwrap();
                } else if amount > 1000.0 && unit.larger_prefix().is_some() {
                    amount /= 1000.0;
                    unit = unit.larger_prefix().unwrap();
                }
                if amount > 10.0 {
                    amount = amount.round();
                }
                if amount.fract() == 0.0 {
                    write!(f, "{}{} {}", amount as u64, unit, name)?;
                    if let Some(instructions) = &self.instructions {
                        write!(f, ", {}", instructions)?;
                    }
                    Ok(())
                } else {
                    write!(f, "{}{} {}", amount, unit, name)
                }
            }
            _ => write!(f, "{}", self.raw),
        }
    }
}

trait Canonicalize {
    fn canonicalize(&self) -> f64;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Unit {
    Mass(MassUnit),
    Volume(VolumeUnit),
    Spoon(SpoonUnit),
    // Could be something like 'small pack', 'pinch of', 'handful', etc.
    Other(String),
}

impl Unit {
    fn canonical_unit(&self) -> Self {
        match self {
            Self::Mass(_) => Self::Mass(MassUnit::Grams),
            Self::Volume(_) => Self::Volume(VolumeUnit::Litres),
            _ => self.clone(),
        }
    }

    fn smaller_prefix(&self) -> Option<Self> {
        match self {
            Self::Mass(mass) => mass.smaller_prefix().map(Self::Mass),
            Self::Volume(volume) => volume.smaller_prefix().map(Self::Volume),
            Self::Spoon(_) | Self::Other(_) => None,
        }
    }

    fn larger_prefix(&self) -> Option<Self> {
        match self {
            Self::Mass(mass) => mass.larger_prefix().map(Self::Mass),
            Self::Volume(volume) => volume.larger_prefix().map(Self::Volume),
            Self::Spoon(_) | Self::Other(_) => None,
        }
    }
}

impl Canonicalize for Unit {
    fn canonicalize(&self) -> f64 {
        match self {
            Self::Mass(mass) => mass.canonicalize(),
            Self::Volume(volume) => volume.canonicalize(),
            Self::Spoon(spoon) => spoon.canonicalize(),
            Self::Other(_) => 1.0,
        }
    }
}

impl FromStr for Unit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(mass) = MassUnit::from_str(s) {
            Ok(Self::Mass(mass))
        } else if let Ok(volume) = VolumeUnit::from_str(s) {
            Ok(Self::Volume(volume))
        } else if let Ok(spoon) = SpoonUnit::from_str(s) {
            Ok(Self::Spoon(spoon))
        } else {
            Ok(Self::Other(s.to_string()))
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mass(mass) => write!(f, "{}", mass),
            Self::Volume(volume) => write!(f, "{}", volume),
            Self::Spoon(spoon) => write!(f, "{}", spoon),
            Self::Other(other) => write!(f, "{}", other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MassUnit {
    Milligrams,
    Grams,
    Kilograms,
    Pounds,
    Ounces,
}

impl MassUnit {
    fn smaller_prefix(&self) -> Option<Self> {
        match self {
            Self::Milligrams => None,
            Self::Grams => Some(Self::Milligrams),
            Self::Kilograms => Some(Self::Grams),
            Self::Pounds => None,
            Self::Ounces => None,
        }
    }

    fn larger_prefix(&self) -> Option<Self> {
        match self {
            Self::Milligrams => Some(Self::Grams),
            Self::Grams => Some(Self::Kilograms),
            Self::Kilograms => None,
            Self::Pounds => Some(Self::Ounces),
            Self::Ounces => None,
        }
    }
}

impl FromStr for MassUnit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mg" | "milligram" | "milligrams" => Ok(Self::Milligrams),
            "g" | "gram" | "grams" => Ok(Self::Grams),
            "kg" | "kilogram" | "kilograms" => Ok(Self::Kilograms),
            "lb" | "pound" | "pounds" => Ok(Self::Pounds),
            "oz" | "ounce" | "ounces" => Ok(Self::Ounces),
            _ => Err(Error::ParsingUnit(s.to_string())),
        }
    }
}

impl Canonicalize for MassUnit {
    fn canonicalize(&self) -> f64 {
        match self {
            Self::Grams => 1.0,
            Self::Kilograms => 1000.0,
            Self::Milligrams => 0.001,
            Self::Pounds => 453.592_37,
            Self::Ounces => 28.349_523_125,
        }
    }
}

impl fmt::Display for MassUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Grams => "g",
                Self::Kilograms => "kg",
                Self::Milligrams => "mg",
                Self::Pounds => "lb",
                Self::Ounces => "oz",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VolumeUnit {
    Millilitres,
    Litres,
    Teaspoons,
    Tablespoons,
    Cups,
    Pints,
    Quarts,
    Gallons,
}

impl VolumeUnit {
    fn smaller_prefix(&self) -> Option<Self> {
        match self {
            Self::Millilitres => None,
            Self::Litres => Some(Self::Millilitres),
            Self::Teaspoons => None,
            Self::Tablespoons => None,
            Self::Cups => None,
            Self::Pints => None,
            Self::Quarts => None,
            Self::Gallons => None,
        }
    }

    fn larger_prefix(&self) -> Option<Self> {
        match self {
            Self::Millilitres => Some(Self::Litres),
            Self::Litres => None,
            Self::Teaspoons => None,
            Self::Tablespoons => None,
            Self::Cups => None,
            Self::Pints => None,
            Self::Quarts => None,
            Self::Gallons => None,
        }
    }
}

impl Canonicalize for VolumeUnit {
    fn canonicalize(&self) -> f64 {
        match self {
            Self::Millilitres => 0.001,
            Self::Litres => 1.0,
            Self::Teaspoons => 0.005,
            Self::Tablespoons => 0.015,
            Self::Cups => 0.284,
            Self::Pints => 0.568,
            Self::Quarts => 0.946_353,
            Self::Gallons => 3.785_412,
        }
    }
}

impl FromStr for VolumeUnit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ml" | "millilitre" | "millilitres" => Ok(Self::Millilitres),
            "l" | "litre" | "litres" => Ok(Self::Litres),
            "tsp" | "teaspoon" | "teaspoons" => Ok(Self::Teaspoons),
            "tbsp" | "tablespoon" | "tablespoons" => Ok(Self::Tablespoons),
            "cup" | "cups" => Ok(Self::Cups),
            "pint" | "pints" => Ok(Self::Pints),
            "quart" | "quarts" => Ok(Self::Quarts),
            "gallon" | "gallons" => Ok(Self::Gallons),
            _ => Err(Error::ParsingUnit(s.to_string())),
        }
    }
}

impl fmt::Display for VolumeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Millilitres => "ml",
                Self::Litres => "l",
                Self::Teaspoons => "tsp",
                Self::Tablespoons => "tbsp",
                Self::Cups => "cup",
                Self::Pints => "pint",
                Self::Quarts => "quart",
                Self::Gallons => "gallon",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpoonUnit {
    Teaspoons,
    Tablespoons,
}

impl FromStr for SpoonUnit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tsp" | "teaspoon" | "teaspoons" => Ok(Self::Teaspoons),
            "tbsp" | "tablespoon" | "tablespoons" => Ok(Self::Tablespoons),
            _ => Err(Error::ParsingUnit(s.to_string())),
        }
    }
}

impl Canonicalize for SpoonUnit {
    fn canonicalize(&self) -> f64 {
        match self {
            Self::Teaspoons => 0.005,
            Self::Tablespoons => 0.015,
        }
    }
}

impl fmt::Display for SpoonUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Teaspoons => "tsp",
                Self::Tablespoons => "tbsp",
            }
        )
    }
}

#[cfg(feature = "scraper")]
static INGREDIENT_REGEX: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
    regex::Regex::new(
        r"^(?P<amount>[0-9¼½¾⅓⅔⅛⅜⅝⅞⅙⅚⅕⅖⅗⅘./]*)?\s*(x\s*)?((?P<unit>ml|millilitre|l|litre|tsp|teaspoon|tbsp|cup|kg|g|gram|oz|ounce|pinch of|pinch|handful of|handful|(small|large) pack) )?\s?(?P<rest>(?P<ingredient>[^,\n]*)((,\s*)(?P<instructions>.*))?)$",
    ).unwrap()
});

#[cfg(feature = "scraper")]
impl FromStr for ScrapedIngredient {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(caps) = INGREDIENT_REGEX.captures(s) else {
            return Err(Error::NoMatch(ScrapedIngredient {
                raw: s.to_string(),
                name: None,
                amount: None,
                unit: None,
                instructions: None,
            }));
        };
        Ok(ScrapedIngredient {
            raw: s.to_string(),
            name: caps
                .name("ingredient")
                .map(|m| m.as_str().trim().to_string()),
            amount: caps.name("amount").and_then(|m| {
                m.as_str()
                    .trim()
                    .parse::<Amount>()
                    .ok()
                    // TODO: write `Amount` type with `as_f64` method
                    // and proper `FromStr` impl which handles fractions (Unicode or with /)
                    .map(|amount| amount.as_f64())
            }),
            unit: caps
                .name("unit")
                .and_then(|m| m.as_str().trim().parse::<Unit>().ok()),
            instructions: caps.name("instructions").map(|m| m.as_str().to_string()),
        })
    }
}

#[cfg(feature = "scraper")]
struct Amount(f64);

#[cfg(feature = "scraper")]
impl Amount {
    fn as_f64(&self) -> f64 {
        self.0
    }
}

#[cfg(feature = "scraper")]
impl FromStr for Amount {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(match s.trim() {
            "¼" => 0.25,
            "½" => 0.5,
            "¾" => 0.75,
            "⅓" => 0.33,
            "⅔" => 0.66,
            "⅛" => 0.125,
            "⅜" => 0.375,
            "⅝" => 0.625,
            "⅞" => 0.875,
            "⅙" => 0.166,
            "⅚" => 0.833,
            "⅕" => 0.2,
            "⅖" => 0.4,
            "⅗" => 0.6,
            "⅘" => 0.8,
            other => match other.parse::<f64>() {
                Ok(n) if n > 0.0 => n,
                Ok(_) => return Err(Error::ParsingAmount(s.to_string())),
                Err(_) => match other.split_once('/') {
                    None => return Err(Error::ParsingAmount(s.to_string())),
                    Some((num, denom)) => {
                        match (num.trim().parse::<f64>(), denom.trim().parse::<f64>()) {
                            (Ok(num), Ok(denom)) if num > 0.0 && denom > 0.0 => num / denom,
                            _ => return Err(Error::ParsingAmount(s.to_string())),
                        }
                    }
                },
            },
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::ScrapedIngredient;

    use super::*;

    #[test]
    fn parse() {
        let cases = [
            (
                "1/2 cup sugar",
                ScrapedIngredient {
                    raw: "1/2 cup sugar".to_string(),
                    name: Some("sugar".to_string()),
                    amount: Some(0.5),
                    unit: Some(Unit::Volume(VolumeUnit::Cups)),
                    instructions: None,
                },
            ),
            (
                "1 tomato, chopped",
                ScrapedIngredient {
                    raw: "1 tomato, chopped".to_string(),
                    name: Some("tomato".to_string()),
                    amount: Some(1.0),
                    unit: None,
                    instructions: Some("chopped".to_string()),
                },
            ),
            (
                "200g cashews",
                ScrapedIngredient {
                    raw: "200g cashews".to_string(),
                    name: Some("cashews".to_string()),
                    amount: Some(200.0),
                    unit: Some(Unit::Mass(MassUnit::Grams)),
                    instructions: None,
                },
            ),
            (
                "100ml milk",
                ScrapedIngredient {
                    raw: "100ml milk".to_string(),
                    name: Some("milk".to_string()),
                    amount: Some(100.0),
                    unit: Some(Unit::Volume(VolumeUnit::Millilitres)),
                    instructions: None,
                },
            ),
            (
                "400g rigatoni or penne",
                ScrapedIngredient {
                    raw: "400g rigatoni or penne".to_string(),
                    name: Some("rigatoni or penne".to_string()),
                    amount: Some(400.0),
                    unit: Some(Unit::Mass(MassUnit::Grams)),
                    instructions: None,
                },
            ),
            (
                "4 garlic cloves, sliced",
                ScrapedIngredient {
                    raw: "4 garlic cloves, sliced".to_string(),
                    name: Some("garlic cloves".to_string()),
                    amount: Some(4.0),
                    unit: None,
                    instructions: Some("sliced".to_string()),
                },
            ),
            (
                "125g ball mozzarella, chopped into chunks",
                ScrapedIngredient {
                    raw: "125g ball mozzarella, chopped into chunks".to_string(),
                    name: Some("ball mozzarella".to_string()),
                    amount: Some(125.0),
                    unit: Some(Unit::Mass(MassUnit::Grams)),
                    instructions: Some("chopped into chunks".to_string()),
                },
            ),
            (
                "¼ white cabbage, finely shredded",
                ScrapedIngredient {
                    raw: "¼ white cabbage, finely shredded".to_string(),
                    name: Some("white cabbage".to_string()),
                    amount: Some(0.25),
                    unit: None,
                    instructions: Some("finely shredded".to_string()),
                },
            ),
            (
                "0.8kg lamb, shoulder or leg, cut into large chunks",
                ScrapedIngredient {
                    raw: "0.8kg lamb, shoulder or leg, cut into large chunks".to_string(),
                    // Can't really do anything about the "shoulder or leg" part.
                    name: Some("lamb".to_string()),
                    amount: Some(0.8),
                    unit: Some(Unit::Mass(MassUnit::Kilograms)),
                    instructions: Some("shoulder or leg, cut into large chunks".to_string()),
                },
            ),
            (
                "tomato ketchup, to serve (optional)",
                ScrapedIngredient {
                    raw: "tomato ketchup, to serve (optional)".to_string(),
                    name: Some("tomato ketchup".to_string()),
                    amount: None,
                    unit: None,
                    instructions: Some("to serve (optional)".to_string()),
                },
            ),
        ];

        for (input, expected) in cases.iter() {
            let actual = input.parse::<ScrapedIngredient>().unwrap();
            assert_eq!(actual, *expected);
        }
    }

    #[test]
    fn display() {
        let cases = [
            (
                ScrapedIngredient {
                    raw: "1/2 cup sugar".to_string(),
                    name: Some("sugar".to_string()),
                    amount: Some(0.5),
                    unit: Some(Unit::Volume(VolumeUnit::Cups)),
                    instructions: None,
                },
                "142ml sugar", // TODO: figure out how to convert this from volume to mass...
            ),
            (
                ScrapedIngredient {
                    raw: "1 tomato, chopped".to_string(),
                    name: Some("tomato".to_string()),
                    amount: Some(1.0),
                    unit: None,
                    instructions: Some("chopped".to_string()),
                },
                "1 tomato, chopped",
            ),
            (
                ScrapedIngredient {
                    raw: "200g cashews".to_string(),
                    name: Some("cashews".to_string()),
                    amount: Some(200.0),
                    unit: Some(Unit::Mass(MassUnit::Grams)),
                    instructions: None,
                },
                "200g cashews",
            ),
            (
                ScrapedIngredient {
                    raw: "100ml milk".to_string(),
                    name: Some("milk".to_string()),
                    amount: Some(100.0),
                    unit: Some(Unit::Volume(VolumeUnit::Millilitres)),
                    instructions: None,
                },
                "100ml milk",
            ),
            (
                ScrapedIngredient {
                    raw: "400g rigatoni or penne".to_string(),
                    name: Some("rigatoni or penne".to_string()),
                    amount: Some(400.0),
                    unit: Some(Unit::Mass(MassUnit::Grams)),
                    instructions: None,
                },
                "400g rigatoni or penne",
            ),
            (
                ScrapedIngredient {
                    raw: "4 garlic cloves, sliced".to_string(),
                    name: Some("garlic cloves".to_string()),
                    amount: Some(4.0),
                    unit: None,
                    instructions: Some("sliced".to_string()),
                },
                "4 garlic cloves, sliced",
            ),
            (
                ScrapedIngredient {
                    raw: "125g ball mozzarella, chopped into chunks".to_string(),
                    name: Some("ball mozzarella".to_string()),
                    amount: Some(125.0),
                    unit: Some(Unit::Mass(MassUnit::Grams)),
                    instructions: Some("chopped into chunks".to_string()),
                },
                "125g ball mozzarella, chopped into chunks",
            ),
            (
                ScrapedIngredient {
                    raw: "¼ white cabbage, finely shredded".to_string(),
                    name: Some("white cabbage".to_string()),
                    amount: Some(0.25),
                    unit: None,
                    instructions: Some("finely shredded".to_string()),
                },
                "¼ white cabbage, finely shredded",
            ),
            (
                ScrapedIngredient {
                    raw: "0.8kg lamb, shoulder or leg, cut into large chunks".to_string(),
                    // Can't really do anything about the "shoulder or leg" part.
                    name: Some("lamb".to_string()),
                    amount: Some(0.8),
                    unit: Some(Unit::Mass(MassUnit::Kilograms)),
                    instructions: Some("shoulder or leg, cut into large chunks".to_string()),
                },
                "800g lamb, shoulder or leg, cut into large chunks",
            ),
            (
                ScrapedIngredient {
                    raw: "tomato ketchup, to serve (optional)".to_string(),
                    name: Some("tomato ketchup".to_string()),
                    amount: None,
                    unit: None,
                    instructions: Some("to serve (optional)".to_string()),
                },
                "tomato ketchup, to serve (optional)",
            ),
        ];

        for (input, expected) in cases.iter() {
            let actual = input.to_string();
            assert_eq!(actual, *expected);
        }
    }

    #[test]
    fn round_trip() {
        let cases = [
            ("1/2 cup sugar", "142ml sugar"), // TODO: figure out how to convert this from volume to mass...
            ("1 tomato, chopped", "1 tomato, chopped"),
            ("200g cashews", "200g cashews"),
            ("100ml milk", "100ml milk"),
            ("400g rigatoni or penne", "400g rigatoni or penne"),
            ("4 garlic cloves, sliced", "4 garlic cloves, sliced"),
            (
                "125g ball mozzarella, chopped into chunks",
                "125g ball mozzarella, chopped into chunks",
            ),
            (
                "¼ white cabbage, finely shredded",
                "¼ white cabbage, finely shredded",
            ),
            (
                "0.8kg lamb, shoulder or leg, cut into large chunks",
                "800g lamb, shoulder or leg, cut into large chunks",
            ),
            (
                "tomato ketchup, to serve (optional)",
                "tomato ketchup, to serve (optional)",
            ),
            ("6 oz cream cheese, cold", "170g cream cheese, cold"),
            ("½ teaspoon ground ginger", "2.5ml ground ginger"),
        ];
        for (input, expected) in cases.iter() {
            let actual = input.parse::<ScrapedIngredient>().unwrap().to_string();
            assert_eq!(actual, *expected);
        }
    }
}
