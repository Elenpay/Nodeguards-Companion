use signer::psbt_details::PSBTDetails;
use yew::prelude::*;

/// Renders the transaction summary shown before signing.
///
/// Every value comes from the PSBT itself — nothing here may be sourced from
/// the requesting page (NGC-01). The rows come from
/// `PSBTDetails::display_fields`, which is what the regression tests assert on.
pub fn psbt_summary(details: &PSBTDetails) -> Html {
    html! {
        { for details.display_fields().into_iter().map(|(label, value)| html! {
            <div class="display-field">
                <strong>{format!("{label}:")}</strong>
                <span>{value}</span>
            </div>
        })}
    }
}
