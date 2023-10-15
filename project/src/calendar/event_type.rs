use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum EventType {
    Exkursion,
    Forschungspraktikum,
    Hauptseminar,
    KlinischeVisite,
    Kolloquium,
    Mentoring,
    Orientierungsveranstaltung,
    Praktikum,
    Projekt,
    Proseminar,
    Pruefungseinsicht,
    Repetitorium,
    Seminar,
    Tutorium,
    Vorlesung,
    VorlesungMitIntegriertenUebungen,
    Workshop,
    ZentralerHochschulsport,
    Uebung,
    Fachpruefung,
}

pub enum Filter {
    Include(HashSet<EventType>),
    Exclude(HashSet<EventType>),
    None,
}

#[derive(Debug)]
pub struct InvalidEventType {
    id: String,
}

impl EventType {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Exkursion => "EX",
            Self::Forschungspraktikum => "FO",
            Self::Hauptseminar => "HS",
            Self::KlinischeVisite => "KL",
            Self::Kolloquium => "KO",
            Self::Mentoring => "MT",
            Self::Orientierungsveranstaltung => "OV",
            Self::Praktikum => "PR",
            Self::Projekt => "PT",
            Self::Proseminar => "PS",
            Self::Pruefungseinsicht => "PE",
            Self::Repetitorium => "RE",
            Self::Seminar => "SE",
            Self::Tutorium => "TT",
            Self::Vorlesung => "VO",
            Self::VorlesungMitIntegriertenUebungen => "VI",
            Self::Workshop => "WS",
            Self::ZentralerHochschulsport => "ZH",
            Self::Uebung => "UE",
            Self::Fachpruefung => "FA",
        }
    }

    pub fn from_id(id: &str) -> Option<EventType> {
        match id {
            "EX" | "ex" => Some(EventType::Exkursion),
            "FO" | "fo" => Some(EventType::Forschungspraktikum),
            "HS" | "hs" => Some(EventType::Hauptseminar),
            "KL" | "kl" => Some(EventType::KlinischeVisite),
            "KO" | "ko" => Some(EventType::Kolloquium),
            "MT" | "mt" => Some(EventType::Mentoring),
            "OV" | "ov" => Some(EventType::Orientierungsveranstaltung),
            "PR" | "pr" => Some(EventType::Praktikum),
            "PT" | "pt" => Some(EventType::Projekt),
            "PS" | "ps" => Some(EventType::Proseminar),
            "PE" | "pe" => Some(EventType::Pruefungseinsicht),
            "RE" | "re" => Some(EventType::Repetitorium),
            "SE" | "se" => Some(EventType::Seminar),
            "TT" | "tt" => Some(EventType::Tutorium),
            "VO" | "vo" => Some(EventType::Vorlesung),
            "VI" | "vi" => Some(EventType::VorlesungMitIntegriertenUebungen),
            "WS" | "ws" => Some(EventType::Workshop),
            "ZH" | "zh" => Some(EventType::ZentralerHochschulsport),
            "UE" | "ue" => Some(EventType::Uebung),
            "FA" | "fa" => Some(EventType::Fachpruefung),
            _ => None,
        }
    }
}

impl Filter {
    pub fn new_include(types: HashSet<EventType>) -> Self {
        Self::Include(types)
    }

    pub fn new_exclude(types: HashSet<EventType>) -> Self {
        Self::Exclude(types)
    }

    pub fn new_none() -> Self {
        Self::None
    }

    pub fn contains(&self, typ: EventType) -> bool {
        match self {
            Self::Include(types) => types.contains(&typ),
            Self::Exclude(types) => !types.contains(&typ),
            Self::None => true,
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exkursion => write!(f, "Exkursion"),
            Self::Forschungspraktikum => write!(f, "Forschungspraktikum"),
            Self::Hauptseminar => write!(f, "Hauptseminar"),
            Self::KlinischeVisite => write!(f, "Klinische Visite"),
            Self::Kolloquium => write!(f, "Kolloquium"),
            Self::Mentoring => write!(f, "Mentoring"),
            Self::Orientierungsveranstaltung => write!(f, "Orientierungsveranstaltung"),
            Self::Praktikum => write!(f, "Praktikum"),
            Self::Projekt => write!(f, "Projekt"),
            Self::Proseminar => write!(f, "Proseminar"),
            Self::Pruefungseinsicht => write!(f, "Prüfungseinsicht"),
            Self::Repetitorium => write!(f, "Repetitorium"),
            Self::Seminar => write!(f, "Seminar"),
            Self::Tutorium => write!(f, "Tutorium"),
            Self::Vorlesung => write!(f, "Vorlesung"),
            Self::VorlesungMitIntegriertenUebungen => write!(f, "Vorlesung mit integrierten Übungen"),
            Self::Workshop => write!(f, "Workshop"),
            Self::ZentralerHochschulsport => write!(f, "Zentraler Hochschulsport"),
            Self::Uebung => write!(f, "Übung"),
            Self::Fachpruefung => write!(f, "Fachprüfung"),
        }
    }
}


impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<EventType>()
            .map_err(|e| de::Error::custom(format!("{}", e)))
    }
}

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.id())
    }
}

impl FromStr for EventType {
    type Err = InvalidEventType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(course_type) = EventType::from_id(s) {
            Ok(course_type)
        } else {
            Err(InvalidEventType {
                id: s.to_string(),
            })
        }
    }
}

impl fmt::Display for InvalidEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid course type: {}", self.id)
    }
}

impl std::error::Error for InvalidEventType {}