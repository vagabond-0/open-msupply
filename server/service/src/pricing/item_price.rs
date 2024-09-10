use repository::{
    EqualFilter, MasterListFilter, MasterListLineFilter, MasterListLineRepository,
    MasterListRepository, PatientFilter,
};
use repository::{PatientRepository, RepositoryError};

use crate::service_provider::ServiceContext;

pub struct ItemPriceLookup {
    pub item_id: String,
    pub customer_name_id: Option<String>, // Unused right now, but could be used to get discount for a specific name
}

#[derive(Debug, PartialEq)]
pub struct ItemPrice {
    pub item_id: String,
    pub default_price_per_unit: f64,
    pub discount_percentage: f64,
    pub calculated_price_per_unit: f64, // Only populated if we have a default price, without a default price we can't calculate the price
}

pub fn get_pricing_for_item(
    ctx: &ServiceContext,
    ItemPriceLookup {
        customer_name_id,
        item_id,
    }: ItemPriceLookup,
) -> Result<ItemPrice, RepositoryError> {
    // 1. Get the default price list & price per unit for the item
    let default_price_per_unit = MasterListLineRepository::new(&ctx.connection)
        .query_one(
            MasterListLineFilter::new()
                .item_id(EqualFilter::equal_to(&item_id))
                .master_list(MasterListFilter::new().is_default_price_list(true)),
        )?
        .map(|m| m.price_per_unit)
        .unwrap_or(0.0);

    // 2. Check if we have a name, and that name is not a patient

    let is_patient = if let Some(customer_name_id) = customer_name_id {
        PatientRepository::new(&ctx.connection)
            .query_one(
                PatientFilter::new().id(EqualFilter::equal_to(&customer_name_id)),
                None,
            )?
            .is_some()
    } else {
        false
    };

    let discount_percentage = if is_patient {
        0.0 // Patients get no discount
    } else {
        // 2.A Lookup the discount list
        // Find the first discount list that has the item (not trying to be clever here, just using the first one found)
        MasterListRepository::new(&ctx.connection)
            .query_one(
                MasterListFilter::new()
                    .is_discount_list(true)
                    .item_id(EqualFilter::equal_to(&item_id)),
            )?
            .map(|m| m.discount_percentage)
            .unwrap_or(0.0)
    };

    // 3. Calculate the price if we are able to
    let calculated_price = default_price_per_unit * (1.0 - discount_percentage / 100.0);

    // 4. Return the pricing data
    Ok(ItemPrice {
        item_id,
        default_price_per_unit,
        discount_percentage,
        calculated_price_per_unit: calculated_price,
    })
}
