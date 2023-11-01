use async_graphql::*;
use graphql_core::generic_filters::EqualFilterStringInput;
use graphql_core::generic_inputs::PrintReportSortInput;
use graphql_core::pagination::PaginationInput;
use graphql_core::standard_graphql_error::{validate_auth, StandardGraphqlError};
use graphql_core::ContextExt;
use graphql_types::types::{StocktakeLineConnector, StocktakeLineNode};
use repository::*;
use serde::Serialize;
use service::auth::Resource;
use service::auth::ResourceAccessRequest;
use service::stocktake_line::query::GetStocktakeLinesError;
use service::ListError;
use strum::{EnumIter, IntoEnumIterator};

#[derive(InputObject, Clone)]
pub struct StocktakeLineFilterInput {
    pub id: Option<EqualFilterStringInput>,
    pub stocktake_id: Option<EqualFilterStringInput>,
    pub location_id: Option<EqualFilterStringInput>,
}

impl From<StocktakeLineFilterInput> for StocktakeLineFilter {
    fn from(f: StocktakeLineFilterInput) -> Self {
        StocktakeLineFilter {
            id: f.id.map(EqualFilter::from),
            stocktake_id: f.stocktake_id.map(EqualFilter::from),
            location_id: f.location_id.map(EqualFilter::from),
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Serialize, EnumIter)]
#[graphql(rename_items = "camelCase")]
pub enum StocktakeLineSortFieldInput {
    ItemCode,
    ItemName,
    /// Stocktake line batch
    Batch,
    /// Stocktake line expiry date
    ExpiryDate,
    /// Stocktake line pack size
    PackSize,
    /// Stocktake line item stock location name
    LocationName,
}

#[derive(InputObject)]
pub struct StocktakeLineSortInput {
    /// Sort query result by `key`
    pub key: StocktakeLineSortFieldInput,
    /// Sort query result is sorted descending or ascending (if not provided the default is
    /// ascending)
    pub desc: Option<bool>,
}

impl StocktakeLineSortInput {
    pub fn to_domain(self) -> StocktakeLineSort {
        use StocktakeLineSortField as to;
        use StocktakeLineSortFieldInput as from;
        let key = match self.key {
            from::ItemCode => to::ItemCode,
            from::ItemName => to::ItemName,
            from::Batch => to::Batch,
            from::ExpiryDate => to::ExpiryDate,
            from::PackSize => to::PackSize,
            from::LocationName => to::LocationName,
        };

        StocktakeLineSort {
            key,
            desc: self.desc,
        }
    }
}

#[derive(Union)]
pub enum StocktakesLinesResponse {
    Response(StocktakeLineConnector),
}

pub fn stocktake_lines(
    ctx: &Context<'_>,
    store_id: &str,
    page: Option<PaginationInput>,
    filter: Option<StocktakeLineFilterInput>,
    sort: Option<StocktakeLineSortInput>,
) -> Result<StocktakesLinesResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::QueryStocktake,
            store_id: Some(store_id.to_string()),
        },
    )?;

    let service_provider = ctx.service_provider();
    let service_ctx = service_provider.context(store_id.to_string(), user.user_id)?;
    let service = &service_provider.stocktake_line_service;

    let stocktake_filter = filter.clone().map(StocktakeLineFilter::from);

    let mut stocktake_id = "".to_string();
    if let Some(f) = stocktake_filter {
        if let Some(id) = f.stocktake_id {
            if let Some(id) = id.equal_to {
                stocktake_id = id;
            }
        }
    }

    let stocktake_lines = service.get_stocktake_lines(
        &service_ctx,
        store_id,
        &stocktake_id,
        page.map(PaginationOption::from),
        filter.map(StocktakeLineFilter::from),
        sort.map(|s| s.to_domain()),
    );

    if let Ok(stocktake_lines) = stocktake_lines {
        Ok(StocktakesLinesResponse::Response(StocktakeLineConnector {
            total_count: stocktake_lines.count,
            nodes: stocktake_lines
                .rows
                .into_iter()
                .map(|line| StocktakeLineNode::from_domain(line))
                .collect(),
        }))
    } else {
        let err = stocktake_lines.unwrap_err();
        let formatted_error = format!("{:#?}", err);
        let graphql_error = match err {
            GetStocktakeLinesError::DatabaseError(err) => err.into(),
            GetStocktakeLinesError::InvalidStore => {
                StandardGraphqlError::BadUserInput(formatted_error)
            }
            GetStocktakeLinesError::InvalidStocktake => {
                StandardGraphqlError::BadUserInput(formatted_error)
            }
            GetStocktakeLinesError::ListError(err) => match err {
                ListError::DatabaseError(err) => err.into(),
                ListError::LimitBelowMin(_) => StandardGraphqlError::BadUserInput(formatted_error),
                ListError::LimitAboveMax(_) => StandardGraphqlError::BadUserInput(formatted_error),
            },
        };
        Err(graphql_error.extend())
    }
}

pub fn report_sort_to_typed_sort<T: IntoEnumIterator + Serialize>(
    sort: Option<PrintReportSortInput>,
) -> Option<(T, Option<bool>)> {
    match sort {
        None => None,
        Some(PrintReportSortInput { key, desc }) => T::iter()
            .find(|variant| {
                let variant_name = serde_json::to_string(variant)
                    .unwrap_or_default()
                    .trim_matches('"')
                    .to_lowercase();
                variant_name == key.to_lowercase()
            })
            .map(|variant| (variant, desc)),
    }
}

#[cfg(test)]
mod test {
    use async_graphql::EmptyMutation;
    use async_graphql::*;
    use chrono::NaiveDate;
    use graphql_core::assert_graphql_query;
    use graphql_core::generic_inputs::PrintReportSortInput;
    use graphql_core::test_helpers::setup_graphl_test;
    use repository::mock::mock_stocktake_line_a;
    use repository::{mock::MockDataInserts, StorageConnectionManager};
    use repository::{
        PaginationOption, StocktakeLine, StocktakeLineFilter, StocktakeLineRow, StocktakeLineSort,
    };
    use serde::Serialize;
    use serde_json::json;
    use service::{
        service_provider::{ServiceContext, ServiceProvider},
        stocktake_line::{query::GetStocktakeLinesError, StocktakeLineServiceTrait},
        ListResult,
    };
    use strum::EnumIter;
    use util::inline_init;

    use crate::stocktake_line_queries::report_sort_to_typed_sort;
    use crate::StocktakeLineQueries;

    type ServiceMethod = dyn Fn(
            &ServiceContext,
            &str,
            &str,
            Option<PaginationOption>,
            Option<StocktakeLineFilter>,
            Option<StocktakeLineSort>,
        ) -> Result<ListResult<StocktakeLine>, GetStocktakeLinesError>
        + Send
        + Sync;

    pub struct TestService(pub Box<ServiceMethod>);

    impl StocktakeLineServiceTrait for TestService {
        fn get_stocktake_lines(
            &self,
            ctx: &ServiceContext,
            store_id: &str,
            stocktake_id: &str,
            pagination: Option<PaginationOption>,
            filter: Option<StocktakeLineFilter>,
            sort: Option<StocktakeLineSort>,
        ) -> Result<ListResult<StocktakeLine>, GetStocktakeLinesError> {
            (self.0)(ctx, store_id, stocktake_id, pagination, filter, sort)
        }
    }

    pub fn service_provider(
        test_service: TestService,
        connection_manager: &StorageConnectionManager,
    ) -> ServiceProvider {
        let mut service_provider = ServiceProvider::new(connection_manager.clone(), "app_data");
        service_provider.stocktake_line_service = Box::new(test_service);
        service_provider
    }

    #[test]
    fn test_report_sort_to_typed_sort() {
        #[derive(Debug, Enum, Copy, Clone, PartialEq, Eq, Serialize, EnumIter)]
        enum SortField {
            ItemCode,
            ItemName,
        }

        assert_eq!(
            report_sort_to_typed_sort::<SortField>(Some(PrintReportSortInput {
                key: "batch".to_string(),
                desc: Some(false)
            })),
            None
        );
        assert_eq!(
            report_sort_to_typed_sort::<SortField>(Some(PrintReportSortInput {
                key: "ItemCode".to_string(),
                desc: Some(true)
            })),
            Some((SortField::ItemCode, Some(true)))
        );
        assert_eq!(
            report_sort_to_typed_sort::<SortField>(Some(PrintReportSortInput {
                key: "ItemName".to_string(),
                desc: None
            })),
            Some((SortField::ItemName, None))
        );
    }

    #[actix_rt::test]
    async fn test_graphql_stocktake_lines_query() {
        let (_, _, connection_manager, settings) = setup_graphl_test(
            StocktakeLineQueries,
            EmptyMutation,
            "omsupply-database-gql-stocktake_lines_query",
            MockDataInserts::all(),
        )
        .await;

        let query = r#"query QueryStocktakeLines($storeId: String, $page: PaginationInput, $filter: StocktakeLineFilterInput, $sort: [StocktakeLineSortInput!]) {
            stocktakeLines(storeId: $storeId, page: $page, filter: $filter, sort: $sort) {
                ... on StocktakeLineConnector {
                    totalCount
                    nodes {
                        id
                        stocktakeId
                        batch
                        expiryDate
                        item {
                            code
                            name
                        }
                    }
                }
            }
        }"#;

        let test_service = TestService(Box::new(|_, _, _, _, _, _| {
            Ok(ListResult {
                rows: vec![inline_init(|r: &mut StocktakeLine| {
                    r.line = inline_init(|l: &mut StocktakeLineRow| {
                        l.id = "id".to_string();
                        l.batch = Some("batch".to_string());
                        l.expiry_date = Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
                        l.stocktake_id = "stocktake_id".to_string();
                        l.item_id = mock_stocktake_line_a().item_id;
                    });
                })],
                count: 1,
            })
        }));

        let expected = json!({
            "stocktakeLines": {
                "totalCount": 1,
                "nodes": [{
                    "id": "id",
                    "batch": "batch",
                    "expiryDate": NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                    "stocktakeId": "stocktake_id",
                    "item": {
                        "code": "item_a_code",
                        "name": "Item A"
                    }
                }]
            }
        });

        let variables = Some(json!({
            "storeId": "store_id",
            "filter": {"stocktakeId": {"equalTo": "stocktake_id"}},
            "page": {
                "first": 10,
                "offset": 0
            }
        }));

        assert_graphql_query!(
            &settings,
            query,
            &variables,
            &expected,
            Some(service_provider(test_service, &connection_manager))
        );
    }
}
