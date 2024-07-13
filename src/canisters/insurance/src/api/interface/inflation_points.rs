use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum Country {
    US,
    UK,
}

pub struct InfationData {
    pub country: Country,
    pub date: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FredInflationDataObservation {
    pub realtime_start: String,
    pub realtime_end: String,
    pub date: String,
    pub value: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FredInflationData {
    pub realtime_start: String,
    pub realtime_end: String,
    pub observation_start: String,
    pub observation_end: String,
    pub units: String,
    pub output_type: u32,
    pub file_type: String,
    pub order_by: String,
    pub sort_order: String,
    pub count: u32,
    pub offset: u32,
    pub limit: u32,
    pub observations: Vec<FredInflationDataObservation>,
}
