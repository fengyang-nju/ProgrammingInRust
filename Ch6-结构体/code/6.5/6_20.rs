enum Location {
    Africa,
    America,
    Asia,
    Europe,
    Oceania,
}

impl Location {
    fn to_string(&self) -> String {
        let str = match self {
            Location::Africa => "Africa",
            Location::America => "America",
            Location::Asia => "Asia",
            Location::Europe => "Europe",
            Location::Oceania => "Oceania",
        };
        str.to_string()
    }
}