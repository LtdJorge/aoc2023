use tracing::{instrument, trace};

#[instrument]
pub fn to_json_pretty(input: Vec<String>) -> Vec<String> {
    let output = input
        .iter()
        .filter_map(|item| serde_json::to_string_pretty(item.as_str()).ok())
        .collect();
    trace!("Output: {:?}", output);
    output
}
