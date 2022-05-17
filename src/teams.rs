use std::fmt::{Display, Formatter};
use num_enum::{TryFromPrimitive, IntoPrimitive};

pub const TEAMS: [Team; 31] = [
    Team::All,
    Team::ArizonaDiamondbacks,
    Team::AtlantaBraves,
    Team::BaltimoreOrioles,
    Team::BostonRedSox,
    Team::ChicagoCubs,
    Team::ChicagoWhiteSox,
    Team::CincinnatiReds,
    Team::ClevelandGuardians,
    Team::ColoradoRockies,
    Team::DetroitTigers,
    Team::HoustonAstros,
    Team::KansasCityRoyals,
    Team::LosAngelesAngels,
    Team::LosAngelesDodgers,
    Team::MiamiMarlins,
    Team::MilwaukeeBrewers,
    Team::MinnesotaTwins,
    Team::NewYorkMets,
    Team::NewYorkYankees,
    Team::OaklandAthletics,
    Team::PhiladelphiaPhillies,
    Team::PittsburghPirates,
    Team::SanDiegoPadres,
    Team::SanFranciscoGiants,
    Team::SeattleMariners,
    Team::StLouisCardinals,
    Team::TampaBayRays,
    Team::TexasRangers,
    Team::TorontoBlueJays,
    Team::WashingtonNationals,
];

#[derive(IntoPrimitive, TryFromPrimitive, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Team {
    All = 0,
    ArizonaDiamondbacks = 29,
    AtlantaBraves = 15,
    BaltimoreOrioles = 1,
    BostonRedSox = 2,
    ChicagoCubs = 16,
    ChicagoWhiteSox = 4,
    CincinnatiReds = 17,
    ClevelandGuardians = 5,
    ColoradoRockies = 27,
    DetroitTigers = 6,
    HoustonAstros = 18,
    KansasCityRoyals = 7,
    LosAngelesAngels = 3,
    LosAngelesDodgers = 19,
    MiamiMarlins = 28,
    MilwaukeeBrewers = 8,
    MinnesotaTwins = 9,
    NewYorkMets = 21,
    NewYorkYankees = 10,
    OaklandAthletics = 11,
    PhiladelphiaPhillies = 22,
    PittsburghPirates = 23,
    SanDiegoPadres = 25,
    SanFranciscoGiants = 26,
    SeattleMariners = 12,
    StLouisCardinals = 24,
    TampaBayRays = 30,
    TexasRangers = 13,
    TorontoBlueJays = 14,
    WashingtonNationals = 20,
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Team::All => write!(f, "All"),
            Team::ArizonaDiamondbacks => write!(f, "Arizona Diamondbacks"),
            Team::AtlantaBraves => write!(f, "Atlanta Braves"),
            Team::BaltimoreOrioles => write!(f, "Baltimore Orioles"),
            Team::BostonRedSox => write!(f, "Boston Red Sox"),
            Team::ChicagoCubs => write!(f, "Chicago Cubs"),
            Team::ChicagoWhiteSox => write!(f, "Chicago White Sox"),
            Team::CincinnatiReds => write!(f, "Cincinnati Reds"),
            Team::ClevelandGuardians => write!(f, "Cleveland Guardians"),
            Team::ColoradoRockies => write!(f, "Colorado Rockies"),
            Team::DetroitTigers => write!(f, "Detroit Tigers"),
            Team::HoustonAstros => write!(f, "Houston Astros"),
            Team::KansasCityRoyals => write!(f, "Kansas City Royals"),
            Team::LosAngelesAngels => write!(f, "Los Angeles Angels"),
            Team::LosAngelesDodgers => write!(f, "Los Angeles Dodgers"),
            Team::MiamiMarlins => write!(f, "Miami Marlins"),
            Team::MilwaukeeBrewers => write!(f, "Milwaukee Brewers"),
            Team::MinnesotaTwins => write!(f, "Minnesota Twins"),
            Team::NewYorkMets => write!(f, "New York Mets"),
            Team::NewYorkYankees => write!(f, "New York Yankees"),
            Team::OaklandAthletics => write!(f, "Oakland Athletics"),
            Team::PhiladelphiaPhillies => write!(f, "Philadelphia Phillies"),
            Team::PittsburghPirates => write!(f, "Pittsburgh Pirates"),
            Team::SanDiegoPadres => write!(f, "San Diego Padres"),
            Team::SanFranciscoGiants => write!(f, "San Francisco Giants"),
            Team::SeattleMariners => write!(f, "Seattle Mariners"),
            Team::StLouisCardinals => write!(f, "St. Louis Cardinals"),
            Team::TampaBayRays => write!(f, "Tampa Bay Rays"),
            Team::TexasRangers => write!(f, "Texas Rangers"),
            Team::TorontoBlueJays => write!(f, "Toronto Blue Jays"),
            Team::WashingtonNationals => write!(f, "Washington Nationals"),
        }
    }
}