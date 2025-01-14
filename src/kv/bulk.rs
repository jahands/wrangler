use cloudflare::endpoints::workerskv::delete_bulk::DeleteBulk;
use cloudflare::endpoints::workerskv::write_bulk::KeyValuePair;
use cloudflare::endpoints::workerskv::write_bulk::WriteBulk;
use cloudflare::framework::apiclient::ApiClient;

use crate::commands::kv::format_error;
use crate::http;
use crate::settings::global_user::GlobalUser;
use crate::settings::toml::Target;

pub const MAX_PAIRS: usize = 10000;

pub fn put(
    client: &impl ApiClient,
    target: &Target,
    namespace_id: &str,
    pairs: &[KeyValuePair],
) -> Result<(), failure::Error> {
    match client.request(&WriteBulk {
        account_identifier: &target.account_id,
        namespace_identifier: namespace_id,
        bulk_key_value_pairs: pairs.to_owned(),
    }) {
        Ok(_) => Ok(()),
        Err(e) => failure::bail!("{}", format_error(e)),
    }
}

pub fn delete(
    target: &Target,
    user: &GlobalUser,
    namespace_id: &str,
    keys: Vec<String>,
) -> Result<(), failure::Error> {
    let client = http::cf_v4_client(user)?;

    let response = client.request(&DeleteBulk {
        account_identifier: &target.account_id,
        namespace_identifier: namespace_id,
        bulk_keys: keys,
    });

    match response {
        Ok(_) => Ok(()),
        Err(e) => failure::bail!("{}", format_error(e)),
    }
}
