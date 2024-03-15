#[derive(Default, Debug, Clone)]
pub struct CliSkippable<T>(pub Option<T>);

impl<T: std::fmt::Display> std::fmt::Display for CliSkippable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(s) => s.fmt(f),
            None => Ok(()),
        }
    }
}

impl<T> std::str::FromStr for CliSkippable<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(CliSkippable(None))
        } else {
            match T::from_str(s) {
                Ok(output) => Ok(CliSkippable(Some(output))),
                Err(e) => Err(format!("{e:?}")),
            }
        }
    }
}

impl<T> interactive_clap::ToCli for CliSkippable<T> {
    type CliVariant = CliSkippable<T>;
}
