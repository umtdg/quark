use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Matcher, Utf32Str};
use tauri::State;

use crate::app::state::ItemState;
use crate::error::Result;
use crate::item::ItemRef;

fn build_haystack(item_ref: &ItemRef) -> String {
    let mut fields = Vec::with_capacity(8);

    fields.push(item_ref.title.as_str());
    match &item_ref.data {
        crate::item::item_ref::ItemRefData::Login { login, urls } => {
            fields.push(login.as_str());
            for url in urls {
                fields.push(url.as_str());
            }
        }
        crate::item::item_ref::ItemRefData::CreditCard { masked_number } => {
            fields.push(masked_number.as_str());
        }
    }

    fields.join(" ")
}

#[tauri::command]
pub fn get_items(item_state: State<'_, ItemState>, query: String) -> Result<Vec<ItemRef>> {
    log::trace!("Getting decrypted item refs matching query: '{}'", query);

    let item_refs = item_state.get_decrypted_item_refs()?;

    let mut matcher = Matcher::new(nucleo_matcher::Config::DEFAULT);
    let pattern = Pattern::new(&query, CaseMatching::Ignore, Normalization::Smart, AtomKind::Substring);

    let mut buffer = Vec::new();
    let mut scored: Vec<(&ItemRef, u32)> = item_refs
        .iter()
        .filter_map(|item_ref| {
            buffer.clear();
            let haystack_str = build_haystack(item_ref);
            let haystack = Utf32Str::new(&haystack_str, &mut buffer);
            let score = pattern.score(haystack, &mut matcher)?;

            Some((item_ref, score))
        })
        .collect();

    scored.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(scored
        .into_iter()
        .map(|(item_ref, _)| item_ref.clone())
        .collect())
}
