use serde::{Deserialize, Serialize};

use super::{
    creation_time::CreationTime, expiration_time::ExpirationTime, generic_params::GenericParams,
    info_link::InfoLink, last_modification_time::LastModificationTime, lines::Lines,
    message_type::MessageType, partial_nets::PartialNets, priority::Priority,
    publication_duration::PublicationDuration, rejection_date::RejectionDate,
    source_system::SourceSystem, stops::Stops, validity_fixed_dates::ValidityFixedDates,
    validity_period::ValidityPeriod,
};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ItdAdditionalTravelInformation {
    #[serde(rename = "@type")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub ty: Option<MessageType>,

    #[serde(rename = "@authorID")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_silent")]
    pub author_id: Option<u32>,

    #[serde(rename = "@authorFirstName")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_silent")]
    pub author_first_name: Option<String>,

    #[serde(rename = "@authorName")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_silent")]
    pub author_name: Option<String>,

    #[serde(rename = "@lastModifierFirstName")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_silent")]
    pub last_modifier_first_name: Option<String>,

    #[serde(rename = "@lastModifierName")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_silent")]
    pub last_modifier_name: Option<String>,

    #[serde(rename = "@providerCode")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub provider_code: Option<String>,

    #[serde(rename = "@infoID")]
    pub info_id: Option<String>,

    #[serde(rename = "@seqID")]
    pub seq_id: Option<u32>,

    #[serde(rename = "@deactivated")]
    pub deactivated: bool,

    #[serde(rename = "@priority")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub priority: Option<Priority>,

    #[serde(
        rename = "@publish",
        deserialize_with = "crate::xml::string_into_bool_infallible"
    )]
    pub publish: bool,

    #[serde(
        rename = "@valid",
        deserialize_with = "crate::xml::string_into_bool_infallible"
    )]
    pub valid: bool,

    #[serde(rename = "@affectTimetable")]
    pub affect_timetable: bool,

    #[serde(rename = "@affectAIRequest")]
    pub affect_ai_request: bool,

    #[serde(rename = "@affectTripRequest")]
    pub affect_trip_request: bool,

    #[serde(rename = "@affectDMRequest")]
    pub affect_dm_request: bool,

    #[serde(rename = "@affectPTKernel")]
    pub affect_pt_kernel: bool,

    #[serde(rename = "@useAsBannerInfo")]
    pub use_as_banner_info: bool,

    #[serde(rename = "@blockingType")]
    pub blocking_type: Option<u32>,

    pub source_system: SourceSystem,

    pub partial_nets: PartialNets,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub effect_condition: Option<String>,

    pub creation_time: CreationTime,

    pub last_modification_time: LastModificationTime,

    pub rejection_date: RejectionDate,

    pub publication_duration: PublicationDuration,

    pub validity_period: Vec<ValidityPeriod>,

    pub validity_fixed_dates: ValidityFixedDates,

    pub expiration_date_time: ExpirationTime,

    pub info_link: Vec<InfoLink>,

    pub concerned_lines: Lines,

    pub concerned_stops: Stops,

    pub generic_params: GenericParams,
}
