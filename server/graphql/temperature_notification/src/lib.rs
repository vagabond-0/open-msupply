use async_graphql::*;
use graphql_core::{
    pagination::PaginationInput,
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use graphql_types::types::*;
use repository::{
    temperature_breach::TemperatureBreachFilter, EqualFilter, PaginationOption,
    TemperatureBreachSort, TemperatureBreachSortField,
};
use service::auth::{Resource, ResourceAccessRequest};

#[derive(Default, Clone)]
pub struct TemperatureNotificationQueries;

#[Object]
impl TemperatureNotificationQueries {
    /// Query omSupply temperature notification entries
    pub async fn temperature_notifications(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        #[graphql(desc = "Pagination option (first and offset)")] page: Option<PaginationInput>,
    ) -> Result<TemperatureNotificationsResponse> {
        let user = validate_auth(
            ctx,
            &ResourceAccessRequest {
                resource: Resource::QueryTemperatureBreach,
                store_id: Some(store_id.clone()),
            },
        )?;

        let service_provider = ctx.service_provider();
        let service_context = service_provider.context(store_id.clone(), user.user_id)?;

        // construct filter
        let filter = TemperatureBreachFilter::new()
            .store_id(EqualFilter::equal_to(&store_id))
            .unacknowledged(true);

        let temperature_breaches = service_provider
            .temperature_breach_service
            .get_temperature_breaches(
                &service_context.connection,
                page.map(PaginationOption::from),
                Some(filter),
                Some(TemperatureBreachSort {
                    key: TemperatureBreachSortField::StartDatetime,
                    desc: Some(true),
                }),
            )
            .map_err(StandardGraphqlError::from_list_error)?;

        let temperature_excursions = service_provider
            .temperature_excursion_service
            .get_excursions(&service_context.connection, &store_id)
            .map_err(StandardGraphqlError::from_repository_error)?;

        Ok(TemperatureNotificationsResponse::Response(
            TemperatureNotificationConnector::from_domain(
                temperature_breaches,
                temperature_excursions,
            ),
        ))
    }
}

#[cfg(test)]
mod test {
    use async_graphql::EmptyMutation;
    use chrono::{Duration, NaiveDate};
    use graphql_core::assert_graphql_query;
    use graphql_core::test_helpers::setup_graphl_test;
    use repository::{
        mock::MockDataInserts, temperature_breach::TemperatureBreach, StorageConnection,
        StorageConnectionManager, TemperatureBreachRow, TemperatureBreachRowType,
    };
    use repository::{PaginationOption, TemperatureBreachFilter, TemperatureBreachSort};
    use serde_json::json;

    use service::temperature_breach::TemperatureBreachServiceTrait;
    use service::{service_provider::ServiceProvider, ListError, ListResult};

    use crate::TemperatureNotificationQueries;

    type GetTemperatureNotifications = dyn Fn(
            Option<PaginationOption>,
            Option<TemperatureBreachFilter>,
            Option<TemperatureBreachSort>,
        ) -> Result<ListResult<TemperatureBreach>, ListError>
        + Sync
        + Send;

    pub struct TestService(pub Box<GetTemperatureNotifications>);

    impl TemperatureBreachServiceTrait for TestService {
        fn get_temperature_breaches(
            &self,
            _: &StorageConnection,
            pagination: Option<PaginationOption>,
            filter: Option<TemperatureBreachFilter>,
            sort: Option<TemperatureBreachSort>,
        ) -> Result<ListResult<TemperatureBreach>, ListError> {
            (self.0)(pagination, filter, sort)
        }
    }

    pub fn service_provider(
        temperature_breach_service: TestService,
        connection_manager: &StorageConnectionManager,
    ) -> ServiceProvider {
        let mut service_provider = ServiceProvider::new(connection_manager.clone(), "app_data");
        service_provider.temperature_breach_service = Box::new(temperature_breach_service);
        service_provider
    }

    #[actix_rt::test]
    async fn test_graphql_temperature_notifications_success() {
        let (_, _, connection_manager, settings) = setup_graphl_test(
            TemperatureNotificationQueries,
            EmptyMutation,
            "test_graphql_temperature_notifications_success",
            MockDataInserts::all(),
        )
        .await;

        let query = r#"
        query {
            temperatureNotifications(storeId: \"store_a\") {
              ... on TemperatureNotificationConnector {
                breaches {
                  id
                  sensorId
                  unacknowledged
                }
              }
            }
        }
        "#;

        // Test single record
        let test_service = TestService(Box::new(|_, _, _| {
            Ok(ListResult {
                rows: vec![TemperatureBreach {
                    temperature_breach_row: TemperatureBreachRow {
                        id: "acknowledged_temperature_breach".to_owned(),
                        duration_milliseconds: 3600,
                        unacknowledged: false,
                        r#type: TemperatureBreachRowType::ColdConsecutive,
                        store_id: "store_a".to_string(),
                        location_id: None,
                        threshold_minimum: -273.0,
                        threshold_maximum: 2.0,
                        sensor_id: "sensor_1".to_owned(),
                        start_datetime: NaiveDate::from_ymd_opt(2022, 7, 1)
                            .unwrap()
                            .and_hms_opt(0, 0, 0)
                            .unwrap()
                            + Duration::seconds(47046),
                        end_datetime: Some(
                            NaiveDate::from_ymd_opt(2022, 7, 1)
                                .unwrap()
                                .and_hms_opt(0, 0, 0)
                                .unwrap()
                                + Duration::seconds(50646),
                        ),
                        threshold_duration_milliseconds: 3600,
                        comment: None,
                    },
                }],
                count: 1,
            })
        }));

        let expected = json!({
              "temperatureNotifications": {
                  "breaches": [
                      {
                          "id": "acknowledged_temperature_breach",
                          "sensorId": "sensor_1",
                          "unacknowledged": false,
                      },
                  ]
              }
          }
        );

        assert_graphql_query!(
            &settings,
            query,
            &None,
            &expected,
            Some(service_provider(test_service, &connection_manager))
        );

        // Test no records

        let test_service = TestService(Box::new(|_, _, _| {
            Ok(ListResult {
                rows: Vec::new(),
                count: 0,
            })
        }));

        let expected = json!({
              "temperatureNotifications": {
                  "breaches": [

                  ]
              }
          }
        );

        assert_graphql_query!(
            &settings,
            query,
            &None,
            &expected,
            Some(service_provider(test_service, &connection_manager))
        );
    }
}
