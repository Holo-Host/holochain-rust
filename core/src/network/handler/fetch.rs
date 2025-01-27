use crate::{
    action::{Action, ActionWrapper},
    context::Context,
    instance::dispatch_action,
    network::handler::{get_content_aspect, get_meta_aspects},
};
use holochain_persistence_api::cas::content::Address;
use lib3h_protocol::data_types::FetchEntryData;
use std::sync::Arc;

/// The network has requested a DHT entry from us.
/// Lets try to get it and trigger a response.
pub fn handle_fetch_entry(get_dht_data: FetchEntryData, context: Arc<Context>) {
    let address = Address::from(get_dht_data.entry_address.clone());
    let mut aspects = vec![];

    match get_content_aspect(&address, context.clone()) {
        Ok(content_aspect) => {
            aspects.push(content_aspect);
            match get_meta_aspects(&address, context.clone()) {
                Ok(mut meta_aspects) => aspects.append(&mut meta_aspects),
                Err(get_meta_error) => {
                    context.log(format!(
                        "error/net/handle_fetch_entry: Error getting meta aspects for entry ({:?}), error: {:?}",
                        address,
                        get_meta_error,
                    ));
                }
            }
        }
        Err(get_content_error) => {
            context.log(format!(
                "warn/net/handle_fetch_entry: Could not get content aspect of requested entry ({:?}), error: {:?}",
                address,
                get_content_error,
            ));
        }
    }

    let action_wrapper = ActionWrapper::new(Action::RespondFetch((get_dht_data, aspects)));
    dispatch_action(context.action_channel(), action_wrapper.clone());
}
