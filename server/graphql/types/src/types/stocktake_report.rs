use async_graphql::{self, dataloader::DataLoader, Context, Enum, ErrorExtensions, Object, Result};
use chrono::{DateTime, NaiveDate, Utc};
use repository::{unknown_user, StocktakeRow, StocktakeStatus};
use serde::Serialize;

use super::{InvoiceNode, StocktakeLineReportConnector, UserNode};
use graphql_core::{
    generic_inputs::DataSortInput,
    loader::{
        InvoiceByIdLoader, StocktakeBatchParams, StocktakeLineReportByStocktakeIdLoader, UserLoader,
    },
    standard_graphql_error::StandardGraphqlError,
    ContextExt,
};

/// This enum is used to represent stocktake report status in graphql schema
#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")] // only needed to be comparable in tests
pub enum StocktakeReportNodeStatus {
    New,
    Finalised,
}

/// This struct is used to represent stocktake report in graphql schema
pub struct StocktakeReportNode {
    /// Stocktake row for reporting
    pub stocktake: StocktakeRow,
}

#[Object]
impl StocktakeReportNode {
    pub async fn id(&self) -> &str {
        &self.stocktake.id
    }

    pub async fn store_id(&self) -> &str {
        &self.stocktake.store_id
    }

    /// User that created stocktake, if user is not found in system default unknown user is returned
    pub async fn user(&self, ctx: &Context<'_>) -> Result<UserNode> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();

        let user = loader
            .load_one(self.stocktake.user_id.clone())
            .await?
            .unwrap_or(unknown_user());

        Ok(UserNode::from_domain(user))
    }

    pub async fn stocktake_number(&self) -> i64 {
        self.stocktake.stocktake_number
    }

    pub async fn comment(&self) -> &Option<String> {
        &self.stocktake.comment
    }

    pub async fn description(&self) -> &Option<String> {
        &self.stocktake.description
    }

    pub async fn is_locked(&self) -> bool {
        self.stocktake.is_locked
    }

    pub async fn status(&self) -> StocktakeReportNodeStatus {
        StocktakeReportNodeStatus::from_domain(&self.stocktake.status)
    }

    pub async fn created_datetime(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.stocktake.created_datetime, Utc)
    }

    pub async fn stocktake_date(&self) -> &Option<NaiveDate> {
        &self.stocktake.stocktake_date
    }

    pub async fn finalised_datetime(&self) -> Option<DateTime<Utc>> {
        self.stocktake
            .finalised_datetime
            .map(|dt| DateTime::<Utc>::from_utc(dt, Utc))
    }

    pub async fn inventory_addition_id(&self) -> &Option<String> {
        &self.stocktake.inventory_addition_id
    }

    pub async fn inventory_reduction_id(&self) -> &Option<String> {
        &self.stocktake.inventory_reduction_id
    }

    pub async fn inventory_addition(&self, ctx: &Context<'_>) -> Result<Option<InvoiceNode>> {
        if let Some(ref addition_id) = self.stocktake.inventory_addition_id {
            let loader = ctx.get_loader::<DataLoader<InvoiceByIdLoader>>();
            let invoice = loader.load_one(addition_id.clone()).await?.ok_or(
                StandardGraphqlError::InternalError(format!(
                    "Cannot find inventory addition {}",
                    addition_id
                ))
                .extend(),
            )?;
            Ok(Some(InvoiceNode { invoice }))
        } else {
            Ok(None)
        }
    }

    pub async fn inventory_reduction(&self, ctx: &Context<'_>) -> Result<Option<InvoiceNode>> {
        if let Some(ref reduction_id) = self.stocktake.inventory_reduction_id {
            let loader = ctx.get_loader::<DataLoader<InvoiceByIdLoader>>();
            let invoice = loader.load_one(reduction_id.clone()).await?.ok_or(
                StandardGraphqlError::InternalError(format!(
                    "Cannot find inventory reduction {}",
                    reduction_id
                ))
                .extend(),
            )?;
            Ok(Some(InvoiceNode { invoice }))
        } else {
            Ok(None)
        }
    }

    pub async fn lines(
        &self,
        ctx: &Context<'_>,
        sort: Option<DataSortInput>,
    ) -> Result<StocktakeLineReportConnector> {
        let loader = ctx.get_loader::<DataLoader<StocktakeLineReportByStocktakeIdLoader>>();
        let lines_option: Option<Vec<repository::StocktakeLineReport>> = loader
            .load_one(StocktakeBatchParams::new(
                self.stocktake.id.clone(),
                sort.map(|s| s.to_domain()),
            ))
            .await?;

        let result = match lines_option {
            None => StocktakeLineReportConnector::empty(),
            Some(lines) => StocktakeLineReportConnector::from_domain_vec(lines),
        };

        Ok(result)
    }
}

impl StocktakeReportNode {
    /// Convert stocktake row to stocktake report node
    pub fn from_domain(stocktake: StocktakeRow) -> StocktakeReportNode {
        StocktakeReportNode { stocktake }
    }
}

impl StocktakeReportNodeStatus {
    /// Convert stocktake report status to stocktake status
    pub fn to_domain(self) -> StocktakeStatus {
        match self {
            StocktakeReportNodeStatus::New => StocktakeStatus::New,
            StocktakeReportNodeStatus::Finalised => StocktakeStatus::Finalised,
        }
    }

    /// Convert stocktake status to stocktake report status
    pub fn from_domain(status: &StocktakeStatus) -> StocktakeReportNodeStatus {
        match status {
            StocktakeStatus::New => StocktakeReportNodeStatus::New,
            StocktakeStatus::Finalised => StocktakeReportNodeStatus::Finalised,
        }
    }
}

#[cfg(test)]
mod test {
    use async_graphql::{EmptyMutation, Object};

    use graphql_core::{assert_graphql_query, test_helpers::setup_graphl_test};
    use repository::{
        mock::{mock_user_account_a, MockDataInserts},
        unknown_user, StocktakeRow,
    };
    use serde_json::json;
    use util::inline_init;

    use crate::types::StocktakeReportNode;

    #[actix_rt::test]
    async fn graphql_stocktake_report_user_loader() {
        #[derive(Clone)]
        struct TestQuery;

        let (_, _, _, settings) = setup_graphl_test(
            TestQuery,
            EmptyMutation,
            "graphql_stocktake_report_user_loader",
            MockDataInserts::none().user_accounts(),
        )
        .await;

        #[Object]
        impl TestQuery {
            pub async fn test_query_user_exists(&self) -> StocktakeReportNode {
                StocktakeReportNode {
                    stocktake: inline_init(|r: &mut StocktakeRow| {
                        r.user_id = mock_user_account_a().id;
                    }),
                }
            }
            pub async fn test_query_user_does_not_exist(&self) -> StocktakeReportNode {
                StocktakeReportNode {
                    stocktake: inline_init(|r: &mut StocktakeRow| {
                        r.user_id = "does not exist".to_string()
                    }),
                }
            }
        }

        let expected = json!({
            "testQueryUserExists": {
                "user": {
                    "userId": mock_user_account_a().id
                }
            },
            "testQueryUserDoesNotExist": {
                "user": {
                    "userId": unknown_user().user_row.id
                }
            },
        }
        );

        let query = r#"
        query {
            testQueryUserExists {
                ...user
            }
            testQueryUserDoesNotExist {
                ...user
            }         
        }
        fragment user on StocktakeReportNode {
            user {
                userId
            }
        }
        "#;

        assert_graphql_query!(&settings, &query, &None, expected, None);
    }
}
