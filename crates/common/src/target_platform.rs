use clap::{builder::PossibleValue, ValueEnum};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetPlatform {
    #[default]
    Transputer,
    EPOC16,
    X86_64,
}

impl ValueEnum for TargetPlatform {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            TargetPlatform::Transputer,
            TargetPlatform::EPOC16,
            TargetPlatform::X86_64,
        ]
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        Self::value_variants()
            .iter()
            .find(|v| {
                v.to_possible_value()
                    .expect("ValueEnum::value_variants contains only values with a corresponding ValueEnum::to_possible_value")
                    .matches(input, ignore_case)
            })
            .cloned()
            .ok_or_else(|| std::format!("Invalid variant: {}", input))
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            TargetPlatform::Transputer => {
                PossibleValue::new("Transputer").help("Parachute Transputer emulator")
            }
            TargetPlatform::EPOC16 => {
                PossibleValue::new("EPOC16").help("Psion EPOC16 (v20) architecture")
            }
            TargetPlatform::X86_64 => PossibleValue::new("X86_64").help("x86_64 architecture"),
        })
    }
}

impl std::fmt::Display for TargetPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for TargetPlatform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}
