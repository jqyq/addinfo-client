use std::any::type_name;

use serde::{Deserialize, Deserializer, Serializer};
use tracing::warn;

pub mod additional_link;
pub mod additional_links;
pub mod attachment;
pub mod attachments;
pub mod chrono_naive_date_time;
pub mod client_header_lines;
pub mod cm_server_type;
pub mod cm_target_system;
pub mod cm_target_systems;
pub mod coord;
pub mod coords;
pub mod creation_time;
pub mod expiration_time;
pub mod generic_params;
pub mod gid;
pub mod gids;
pub mod header_line;
pub mod heartbeat;
pub mod heartbeat_notification;
pub mod ics_in_out_channels_request;
pub mod ics_task;
pub mod image;
pub mod info_link;
pub mod info_text;
pub mod interchange;
pub mod interchange_type;
pub mod interchanges;
pub mod itd_add_info_request;
pub mod itd_additional_travel_information;
pub mod itd_additional_travel_informations;
pub mod itd_banner_info_list;
pub mod itd_date;
pub mod itd_date_time;
pub mod itd_info_link_list;
pub mod itd_operator;
pub mod itd_request;
pub mod itd_time;
pub mod itd_train;
pub mod itd_version_info;
pub mod last_modification_time;
pub mod line;
pub mod lines;
pub mod message_transfer_mode;
pub mod message_type;
pub mod output_client_text;
pub mod param;
pub mod param_list;
pub mod partial_nets;
pub mod place;
pub mod priority;
pub mod publication_duration;
pub mod rejection_date;
pub mod sending_format;
pub mod sending_mode;
pub mod source_system;
pub mod status;
pub mod stop;
pub mod stops;
pub mod sync_mode;
pub mod time_mode;
pub mod timestamp;
pub mod trip;
pub mod trips;
pub mod validity_fixed_dates;
pub mod validity_period;

// The AddInfo interface is notorious for randomly not sending specific fields.
// For this reason, we treat most fields as optional and either deserialize them
// into `Some(T)` type if present or into the `None` variant of `Option` if not
// present. Furthermore, some fields may not be deserializable, such as dates and
// times that have values set to -1. Should the deserialization throw an error,
// we will also convert this into a `None` value. We don't want to error as this
// may lead to further messages in the same XML structure not being parsed.
fn try_deserialize_verbose<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    match Option::<T>::deserialize(deserializer) {
        // Leave deserializable values untouched.
        Ok(v) => Ok(v),
        // Fields for which deserialization has failed will be
        // treated as if they hadn't been there in the first place.
        Err(e) => {
            // Logging this as a warning may be undesired given ICS
            // will intentionally send invalid values in some occasions.
            warn!("failed to deserialize into {}: {e}", type_name::<T>());
            // Suppress the error so that deserialization of other data can proceed.
            Ok(None)
        }
    }
}

// Some values aren't always present and we actually don't need them.
// We could simply not include them in our struct defintions but we want
// to emphasize their potential presence. This for instance includes the
// utc attribute of some itdDateTime objects. Given we rely on the
// individual year, month, day, hour, minute, second values, we want to
// silently discard the utc attribute if it's not present as otherwise our
// logs will be spammed with warnings if using the verbose function above.
fn try_deserialize_silent<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    match Option::<T>::deserialize(deserializer) {
        // Leave deserializable values untouched.
        Ok(v) => Ok(v),
        // Silently discard any non-deserializable values into `None`.
        Err(_) => Ok(None),
    }
}

// ICS has different ways of storing booleans. For instance, false may
// be modeled as either the integers -1 or 0 or the literal "false" based
// on the field and possibly depending on the application version as well.
// We don't need to worry about the "false" case as the standard library
// offers the required conversion by default. However, for the integer
// cases we want to convert 1 into true and anything else into false.
fn string_into_bool_infallible<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(d).is_ok_and(|s| s == "1"))
}

// We need to serialize bools into ints when sending requests to ICS.
// We convert true into 1 and false into the default impl of i8, namely 0.
fn bool_into_i8_infallible<S>(b: &bool, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_i8(b.then(|| 1).unwrap_or_default())
}

// Interchanges will either have the direction set to "BOTH" or not have
// it set at all. Whilst we could simply test against the presence of a
// direction and not care about the value at all, specifically testing
// against "BOTH" may be more robust in case of any future changes.
// We deserialize "BOTH" into true and anything else into false.
fn direction_into_bool_infallible<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(d).is_ok_and(|s| s == "BOTH"))
}
