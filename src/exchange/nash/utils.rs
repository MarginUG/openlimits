use nash_native_client::Client;
use std::convert::TryFrom;
use crate::model::Paginator;
use crate::errors::OpenLimitsError;
use super::NashParameters;
use super::shared::Result;
use super::shared::timestamp_to_utc_datetime;

pub async fn client_from_params_failable(params: NashParameters) -> Result<Client> {
    let client = match params.credentials {
        Some(credentials) => {
            Client::from_keys(
                &credentials.secret,
                &credentials.session,
                params.affiliate_code,
                params.turn_off_sign_states,
                params.client_id,
                params.environment,
                params.timeout,
            )
            .await?
        }
        None => {
            Client::from_keys_path(
                None,
                None,
                true,
                params.client_id,
                params.environment,
                params.timeout,
            )
            .await?
        }
    };

    if let Some(interval) = params.sign_states_loop_interval {
        client.start_background_sign_states_loop(interval);
    }
    if let Some(interval) = params.fill_pool_loop_interval {
        client.start_background_fill_pool_loop(interval, params.fill_pool_loop_blockchains);
    }

    Ok(client)
}

pub fn try_split_paginator(
    paginator: Option<Paginator>,
) -> Result<(
    Option<String>,
    Option<i64>,
    Option<nash_protocol::types::DateTimeRange>,
)> {
    Ok(match paginator {
        Some(paginator) => (
            paginator.before,
            match paginator.limit {
                Some(v) => Some(i64::try_from(v).map_err(|_| {
                    OpenLimitsError::InvalidParameter(
                        "Couldn't convert paginator limit to i64".to_string(),
                    )
                })?),
                None => None,
            },
            if paginator.start_time.is_some() && paginator.end_time.is_some() {
                Some(nash_protocol::types::DateTimeRange {
                    start: paginator.start_time.map(timestamp_to_utc_datetime).unwrap(),
                    stop: paginator.end_time.map(timestamp_to_utc_datetime).unwrap(),
                })
            } else {
                None
            },
        ),
        None => (None, None, None),
    })
}