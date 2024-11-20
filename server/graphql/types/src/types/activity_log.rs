use core::alloc;

use async_graphql::{dataloader::DataLoader, *};
use chrono::DateTime;
use chrono::Utc;
use graphql_core::{
    loader::{StoreByIdLoader, UserLoader},
    ContextExt,
};
use repository::{activity_log::ActivityLog, ActivityLogRow, ActivityLogType};
use service::ListResult;

use super::{StoreNode, UserNode};

#[derive(PartialEq, Debug)]
pub struct ActivityLogNode {
    activity_log: ActivityLog,
}

#[derive(SimpleObject)]
pub struct ActivityLogConnector {
    total_count: u32,
    nodes: Vec<ActivityLogNode>,
}

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
pub enum ActivityLogNodeType {
    UserLoggedIn,
    InvoiceCreated,
    InvoiceDeleted,
    InvoiceNumberAllocated,
    InvoiceStatusAllocated,
    InvoiceStatusPicked,
    InvoiceStatusShipped,
    InvoiceStatusDelivered,
    InvoiceStatusVerified,
    InventoryAdjustment,
    StocktakeCreated,
    StocktakeDeleted,
    StocktakeStatusFinalised,
    RequisitionCreated,
    RequisitionDeleted,
    RequisitionNumberAllocated,
    RequisitionApproved,
    RequisitionStatusSent,
    RequisitionStatusFinalised,
    StockLocationChange,
    StockCostPriceChange,
    StockSellPriceChange,
    StockExpiryDateChange,
    StockBatchChange,
    StockOnHold,
    StockOffHold,
    Repack,
    PrescriptionCreated,
    PrescriptionDeleted,
    PrescriptionStatusPicked,
    PrescriptionStatusVerified,
    SensorLocationChanged,
    AssetCreated,
    AssetUpdated,
    AssetDeleted,
    AssetLogCreated,
    AssetCatalogueItemCreated,
    QuantityForLineHasBeenSetToZero,
    AssetCatalogueItemPropertyCreated,
    AssetLogReasonCreated,
    AssetLogReasonDeleted,
    AssetPropertyCreated,
    VaccineCourseCreated,
    ProgramCreated,
    ProgramUpdated,
    VaccineCourseUpdated,
    RnrFormCreated,
    RnrFormUpdated,
    RnrFormFinalised,
    VaccinationCreated,
    VaccinationUpdated,
    VaccinationDeleted,
    DemographicIndicatorCreated,
    DemographicIndicatorUpdated,
    DemographicProjectionCreated,
    DemographicProjectionUpdated,
}
// Recursive expansion of Object macro
// ====================================

impl ActivityLogNode {
    pub async fn id(&self, _: &async_graphql::Context<'_>) -> async_graphql::Result<&str> {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: &str = { &self.row().id };
                    value
                }
                .await,
            )
        }
    }
    pub async fn r#type(
        &self,
        _: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<ActivityLogNodeType> {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: ActivityLogNodeType =
                        { ActivityLogNodeType::from_domain(&self.row().r#type) };
                    value
                }
                .await,
            )
        }
    }
    pub async fn store_id(
        &self,
        _: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<&Option<String>> {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: &Option<String> = { &self.row().store_id };
                    value
                }
                .await,
            )
        }
    }
    pub async fn record_id(
        &self,
        _: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<&Option<String>> {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: &Option<String> = { &self.row().record_id };
                    value
                }
                .await,
            )
        }
    }
    pub async fn datetime(
        &self,
        _: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<DateTime<Utc>> {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: DateTime<Utc> =
                        { DateTime::<Utc>::from_naive_utc_and_offset(self.row().datetime, Utc) };
                    value
                }
                .await,
            )
        }
    }
    pub async fn to(
        &self,
        _: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<&Option<String>> {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: &Option<String> = { &self.row().changed_to };
                    value
                }
                .await,
            )
        }
    }
    pub async fn from(
        &self,
        _: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<&Option<String>> {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: &Option<String> = { &self.row().changed_from };
                    value
                }
                .await,
            )
        }
    }
    pub async fn user(&self, ctx: &Context<'_>) -> Result<Option<UserNode>> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let user_id = match &self.row().user_id {
            Some(user_id) => user_id,
            None => return Ok(None),
        };
        let result = loader
            .load_one(user_id.clone())
            .await?
            .map(UserNode::from_domain);
        Ok(result)
    }
    pub async fn store(&self, ctx: &Context<'_>) -> Result<Option<StoreNode>> {
        let loader = ctx.get_loader::<DataLoader<StoreByIdLoader>>();
        let store_id = match &self.row().store_id {
            Some(store_id) => store_id,
            None => return Ok(None),
        };
        let result = loader.load_one(store_id.clone()).await?.unwrap();
        Ok(Some(StoreNode::from_domain(result)))
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    enum __FieldIdent {
        id,
        r#type,
        store_id,
        record_id,
        datetime,
        to,
        from,
        user,
        store,
    }
    impl __FieldIdent {
        fn from_name(__name: &async_graphql::Name) -> ::std::option::Option<__FieldIdent> {
            match __name.as_str() {
                "id" => ::std::option::Option::Some(__FieldIdent::id),
                "type" => ::std::option::Option::Some(__FieldIdent::r#type),
                "storeId" => ::std::option::Option::Some(__FieldIdent::store_id),
                "recordId" => ::std::option::Option::Some(__FieldIdent::record_id),
                "datetime" => ::std::option::Option::Some(__FieldIdent::datetime),
                "to" => ::std::option::Option::Some(__FieldIdent::to),
                "from" => ::std::option::Option::Some(__FieldIdent::from),
                "user" => ::std::option::Option::Some(__FieldIdent::user),
                "store" => ::std::option::Option::Some(__FieldIdent::store),
                _ => ::std::option::Option::None,
            }
        }
    }
    impl ActivityLogNode {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __id_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.id(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __type_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.r#type(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __store_id_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.store_id(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __record_id_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.record_id(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __datetime_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.datetime(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __to_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.to(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __from_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.from(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __user_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.user(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __store_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.store(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
    }
    #[allow(clippy::all, clippy::pedantic, clippy::suspicious_else_formatting)]
    #[allow(unused_braces, unused_variables, unused_parens, unused_mut)]
    impl async_graphql::resolver_utils::ContainerType for ActivityLogNode {
        async fn resolve_field(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let __field = __FieldIdent::from_name(&ctx.item.node.name.node);
            match __field {
                ::std::option::Option::Some(__FieldIdent::id) => {
                    return self.__id_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::r#type) => {
                    return self.__type_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::store_id) => {
                    return self.__store_id_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::record_id) => {
                    return self.__record_id_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::datetime) => {
                    return self.__datetime_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::to) => {
                    return self.__to_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::from) => {
                    return self.__from_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::user) => {
                    return self.__user_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::store) => {
                    return self.__store_resolver(&ctx).await;
                }
                None => {}
            }
            ::std::result::Result::Ok(::std::option::Option::None)
        }
        async fn find_entity(
            &self,
            ctx: &async_graphql::Context<'_>,
            params: &async_graphql::Value,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let params = match params {
                async_graphql::Value::Object(params) => params,
                _ => return ::std::result::Result::Ok(::std::option::Option::None),
            };
            let typename =
                if let ::std::option::Option::Some(async_graphql::Value::String(typename)) =
                    params.get("__typename")
                {
                    typename
                } else {
                    return ::std::result::Result::Err(async_graphql::ServerError::new(
                        r#""__typename" must be an existing string."#,
                        ::std::option::Option::Some(ctx.item.pos),
                    ));
                };
            ::std::result::Result::Ok(::std::option::Option::None)
        }
    }
    #[allow(clippy::all, clippy::pedantic)]
    impl async_graphql::OutputType for ActivityLogNode {
        fn type_name() -> ::std::borrow::Cow<'static, ::std::primitive::str> {
            ::std::borrow::Cow::Borrowed("ActivityLogNode")
        }
        fn create_type_info(
            registry: &mut async_graphql::registry::Registry,
        ) -> ::std::string::String {
            let ty = registry.create_output_type:: <Self,_>(async_graphql::registry::MetaTypeId::Object, |registry|async_graphql::registry::MetaType::Object {
                name: ::std::borrow::Cow::into_owned(::std::borrow::Cow::Borrowed("ActivityLogNode")),description: ::std::option::Option::None,fields:{
                    let mut fields = async_graphql::indexmap::IndexMap::new();
                    fields.insert(::std::borrow::ToOwned::to_owned("id"),async_graphql::registry::MetaField {
                        name: ::std::borrow::ToOwned::to_owned("id"),description: ::std::option::Option::None,args:{
                            let mut args = async_graphql::indexmap::IndexMap::new();
                            args
                        },ty: < &str as async_graphql::OutputType> ::create_type_info(registry),deprecation:async_graphql::registry::Deprecation::NoDeprecated,cache_control:async_graphql::CacheControl {
                            public:true,max_age:0i32,
                        },external:false,provides: ::std::option::Option::None,requires: ::std::option::Option::None,shareable:false,inaccessible:false,tags: (alloc::vec::Vec::new()),override_from: ::std::option::Option::None,visible: ::std::option::Option::None,compute_complexity: ::std::option::Option::None,directive_invocations: (alloc::vec::Vec::new())
                    });
                    fields.insert(::std::borrow::ToOwned::to_owned("type"),async_graphql::registry::MetaField {
                        name: ::std::borrow::ToOwned::to_owned("type"),description: ::std::option::Option::None,args:{
                            let mut args = async_graphql::indexmap::IndexMap::new();
                            args
                        },ty: <ActivityLogNodeType as async_graphql::OutputType> ::create_type_info(registry),deprecation:async_graphql::registry::Deprecation::NoDeprecated,cache_control:async_graphql::CacheControl {
                            public:true,max_age:0i32,
                        },external:false,provides: ::std::option::Option::None,requires: ::std::option::Option::None,shareable:false,inaccessible:false,tags: (alloc::vec::Vec::new()),override_from: ::std::option::Option::None,visible: ::std::option::Option::None,compute_complexity: ::std::option::Option::None,directive_invocations: (alloc::vec::Vec::new())
                    });
                    fields.insert(::std::borrow::ToOwned::to_owned("storeId"),async_graphql::registry::MetaField {
                        name: ::std::borrow::ToOwned::to_owned("storeId"),description: ::std::option::Option::None,args:{
                            let mut args = async_graphql::indexmap::IndexMap::new();
                            args
                        },ty: < &Option<String>as async_graphql::OutputType> ::create_type_info(registry),deprecation:async_graphql::registry::Deprecation::NoDeprecated,cache_control:async_graphql::CacheControl {
                            public:true,max_age:0i32,
                        },external:false,provides: ::std::option::Option::None,requires: ::std::option::Option::None,shareable:false,inaccessible:false,tags: (alloc::vec::Vec::new()),override_from: ::std::option::Option::None,visible: ::std::option::Option::None,compute_complexity: ::std::option::Option::None,directive_invocations: (alloc::vec::Vec::new())
                    });
                    fields.insert(::std::borrow::ToOwned::to_owned("recordId"),async_graphql::registry::MetaField {
                        name: ::std::borrow::ToOwned::to_owned("recordId"),description: ::std::option::Option::None,args:{
                            let mut args = async_graphql::indexmap::IndexMap::new();
                            args
                        },ty: < &Option<String>as async_graphql::OutputType> ::create_type_info(registry),deprecation:async_graphql::registry::Deprecation::NoDeprecated,cache_control:async_graphql::CacheControl {
                            public:true,max_age:0i32,
                        },external:false,provides: ::std::option::Option::None,requires: ::std::option::Option::None,shareable:false,inaccessible:false,tags: (alloc::vec::Vec::new()),override_from: ::std::option::Option::None,visible: ::std::option::Option::None,compute_complexity: ::std::option::Option::None,directive_invocations: (alloc::vec::Vec::new())
                    });
                    fields.insert(::std::borrow::ToOwned::to_owned("datetime"),async_graphql::registry::MetaField {
                        name: ::std::borrow::ToOwned::to_owned("datetime"),description: ::std::option::Option::None,args:{
                            let mut args = async_graphql::indexmap::IndexMap::new();
                            args
                        },ty: <DateTime<Utc>as async_graphql::OutputType> ::create_type_info(registry),deprecation:async_graphql::registry::Deprecation::NoDeprecated,cache_control:async_graphql::CacheControl {
                            public:true,max_age:0i32,
                        },external:false,provides: ::std::option::Option::None,requires: ::std::option::Option::None,shareable:false,inaccessible:false,tags: (alloc::vec::Vec::new()),override_from: ::std::option::Option::None,visible: ::std::option::Option::None,compute_complexity: ::std::option::Option::None,directive_invocations: (alloc::vec::Vec::new())
                    });
                    fields.insert(::std::borrow::ToOwned::to_owned("to"),async_graphql::registry::MetaField {
                        name: ::std::borrow::ToOwned::to_owned("to"),description: ::std::option::Option::None,args:{
                            let mut args = async_graphql::indexmap::IndexMap::new();
                            args
                        },ty: < &Option<String>as async_graphql::OutputType> ::create_type_info(registry),deprecation:async_graphql::registry::Deprecation::NoDeprecated,cache_control:async_graphql::CacheControl {
                            public:true,max_age:0i32,
                        },external:false,provides: ::std::option::Option::None,requires: ::std::option::Option::None,shareable:false,inaccessible:false,tags: (alloc::vec::Vec::new()),override_from: ::std::option::Option::None,visible: ::std::option::Option::None,compute_complexity: ::std::option::Option::None,directive_invocations: (alloc::vec::Vec::new())
                    });
                    fields.insert(::std::borrow::ToOwned::to_owned("from"),async_graphql::registry::MetaField {
                        name: ::std::borrow::ToOwned::to_owned("from"),description: ::std::option::Option::None,args:{
                            let mut args = async_graphql::indexmap::IndexMap::new();
                            args
                        },ty: < &Option<String>as async_graphql::OutputType> ::create_type_info(registry),deprecation:async_graphql::registry::Deprecation::NoDeprecated,cache_control:async_graphql::CacheControl {
                            public:true,max_age:0i32,
                        },external:false,provides: ::std::option::Option::None,requires: ::std::option::Option::None,shareable:false,inaccessible:false,tags: (alloc::vec::Vec::new()),override_from: ::std::option::Option::None,visible: ::std::option::Option::None,compute_complexity: ::std::option::Option::None,directive_invocations: (alloc::vec::Vec::new())
                    });
                    fields.insert(::std::borrow::ToOwned::to_owned("user"),async_graphql::registry::MetaField {
                        name: ::std::borrow::ToOwned::to_owned("user"),description: ::std::option::Option::None,args:{
                            let mut args = async_graphql::indexmap::IndexMap::new();
                            args
                        },ty: <Option<UserNode>as async_graphql::OutputType> ::create_type_info(registry),deprecation:async_graphql::registry::Deprecation::NoDeprecated,cache_control:async_graphql::CacheControl {
                            public:true,max_age:0i32,
                        },external:false,provides: ::std::option::Option::None,requires: ::std::option::Option::None,shareable:false,inaccessible:false,tags: (alloc::vec::Vec::new()),override_from: ::std::option::Option::None,visible: ::std::option::Option::None,compute_complexity: ::std::option::Option::None,directive_invocations: (alloc::vec::Vec::new())
                    });
                    fields.insert(::std::borrow::ToOwned::to_owned("store"),async_graphql::registry::MetaField {
                        name: ::std::borrow::ToOwned::to_owned("store"),description: ::std::option::Option::None,args:{
                            let mut args = async_graphql::indexmap::IndexMap::new();
                            args
                        },ty: <Option<StoreNode>as async_graphql::OutputType> ::create_type_info(registry),deprecation:async_graphql::registry::Deprecation::NoDeprecated,cache_control:async_graphql::CacheControl {
                            public:true,max_age:0i32,
                        },external:false,provides: ::std::option::Option::None,requires: ::std::option::Option::None,shareable:false,inaccessible:false,tags: (alloc::vec::Vec::new()),override_from: ::std::option::Option::None,visible: ::std::option::Option::None,compute_complexity: ::std::option::Option::None,directive_invocations: (alloc::vec::Vec::new())
                    });
                    fields
                },cache_control:async_graphql::CacheControl {
                    public:true,max_age:0i32,
                },extends:false,shareable:false,resolvable:true,inaccessible:false,interface_object:false,tags: (alloc::vec::Vec::new()),keys: ::std::option::Option::None,visible: ::std::option::Option::None,is_subscription:false,rust_typename: ::std::option::Option::Some(::std::any::type_name:: <Self>()),directive_invocations: (alloc::vec::Vec::new())
            });
            ty
        }
        async fn resolve(
            &self,
            ctx: &async_graphql::ContextSelectionSet<'_>,
            _field: &async_graphql::Positioned<async_graphql::parser::types::Field>,
        ) -> async_graphql::ServerResult<async_graphql::Value> {
            async_graphql::resolver_utils::resolve_container(ctx, self).await
        }
    }
    impl async_graphql::ObjectType for ActivityLogNode {}
};

impl ActivityLogNode {
    pub fn from_domain(activity_log: ActivityLog) -> Self {
        ActivityLogNode { activity_log }
    }

    pub fn row(&self) -> &ActivityLogRow {
        &self.activity_log.activity_log_row
    }
}

impl ActivityLogNodeType {
    pub fn from_domain(from: &ActivityLogType) -> ActivityLogNodeType {
        use ActivityLogNodeType as to;
        use ActivityLogType as from;

        match from {
            from::UserLoggedIn => to::UserLoggedIn,
            from::InvoiceCreated => to::InvoiceCreated,
            from::InvoiceDeleted => to::InvoiceDeleted,
            from::InvoiceStatusAllocated => to::InvoiceStatusAllocated,
            from::InvoiceStatusPicked => to::InvoiceStatusPicked,
            from::InvoiceStatusShipped => to::InvoiceStatusShipped,
            from::InvoiceStatusDelivered => to::InvoiceStatusDelivered,
            from::InvoiceStatusVerified => to::InvoiceStatusVerified,
            from::InventoryAdjustment => to::InventoryAdjustment,
            from::StocktakeCreated => to::StocktakeCreated,
            from::StocktakeDeleted => to::StocktakeDeleted,
            from::StocktakeStatusFinalised => to::StocktakeStatusFinalised,
            from::RequisitionCreated => to::RequisitionCreated,
            from::RequisitionDeleted => to::RequisitionDeleted,
            from::RequisitionApproved => to::RequisitionApproved,
            from::RequisitionStatusSent => to::RequisitionStatusSent,
            from::RequisitionStatusFinalised => to::RequisitionStatusFinalised,
            from::StockLocationChange => to::StockLocationChange,
            from::StockCostPriceChange => to::StockCostPriceChange,
            from::StockSellPriceChange => to::StockSellPriceChange,
            from::StockExpiryDateChange => to::StockExpiryDateChange,
            from::StockBatchChange => to::StockBatchChange,
            from::StockOnHold => to::StockOnHold,
            from::StockOffHold => to::StockOffHold,
            from::InvoiceNumberAllocated => to::InvoiceNumberAllocated,
            from::RequisitionNumberAllocated => to::RequisitionNumberAllocated,
            from::Repack => to::Repack,
            from::PrescriptionCreated => to::PrescriptionCreated,
            from::PrescriptionDeleted => to::PrescriptionDeleted,
            from::PrescriptionStatusPicked => to::PrescriptionStatusPicked,
            from::PrescriptionStatusVerified => to::PrescriptionStatusVerified,
            from::SensorLocationChanged => to::SensorLocationChanged,
            from::AssetCreated => to::AssetCreated,
            from::AssetUpdated => to::AssetUpdated,
            from::AssetDeleted => to::AssetDeleted,
            from::AssetLogCreated => to::AssetLogCreated,
            from::AssetCatalogueItemCreated => to::AssetCatalogueItemCreated,
            from::QuantityForLineHasBeenSetToZero => to::QuantityForLineHasBeenSetToZero,
            from::AssetCatalogueItemPropertyCreated => to::AssetCatalogueItemPropertyCreated,
            from::AssetLogReasonCreated => to::AssetLogReasonCreated,
            from::AssetLogReasonDeleted => to::AssetLogReasonDeleted,
            from::AssetPropertyCreated => to::AssetPropertyCreated,
            from::VaccineCourseCreated => to::VaccineCourseCreated,
            from::VaccineCourseUpdated => to::VaccineCourseUpdated,
            from::ProgramCreated => to::ProgramCreated,
            from::ProgramUpdated => to::ProgramUpdated,
            from::RnrFormCreated => to::RnrFormCreated,
            from::RnrFormUpdated => to::RnrFormUpdated,
            from::RnrFormFinalised => to::RnrFormFinalised,
            from::VaccinationCreated => to::VaccinationCreated,
            from::VaccinationUpdated => to::VaccinationUpdated,
            from::VaccinationDeleted => to::VaccinationDeleted,
            from::DemographicIndicatorCreated => to::DemographicIndicatorCreated,
            from::DemographicIndicatorUpdated => to::DemographicIndicatorUpdated,
            from::DemographicProjectionCreated => to::DemographicProjectionCreated,
            from::DemographicProjectionUpdated => to::DemographicProjectionUpdated,
        }
    }

    pub fn to_domain(self) -> ActivityLogType {
        use ActivityLogNodeType as from;
        use ActivityLogType as to;

        match self {
            from::UserLoggedIn => to::UserLoggedIn,
            from::InvoiceCreated => to::InvoiceCreated,
            from::InvoiceDeleted => to::InvoiceDeleted,
            from::InvoiceStatusAllocated => to::InvoiceStatusAllocated,
            from::InvoiceStatusPicked => to::InvoiceStatusPicked,
            from::InvoiceStatusShipped => to::InvoiceStatusShipped,
            from::InvoiceStatusDelivered => to::InvoiceStatusDelivered,
            from::InvoiceStatusVerified => to::InvoiceStatusVerified,
            from::InventoryAdjustment => to::InventoryAdjustment,
            from::StocktakeCreated => to::StocktakeCreated,
            from::StocktakeDeleted => to::StocktakeDeleted,
            from::StocktakeStatusFinalised => to::StocktakeStatusFinalised,
            from::RequisitionCreated => to::RequisitionCreated,
            from::RequisitionDeleted => to::RequisitionDeleted,
            from::RequisitionApproved => to::RequisitionApproved,
            from::RequisitionStatusSent => to::RequisitionStatusSent,
            from::RequisitionStatusFinalised => to::RequisitionStatusFinalised,
            from::StockLocationChange => to::StockLocationChange,
            from::StockCostPriceChange => to::StockCostPriceChange,
            from::StockSellPriceChange => to::StockSellPriceChange,
            from::StockExpiryDateChange => to::StockExpiryDateChange,
            from::StockBatchChange => to::StockBatchChange,
            from::StockOnHold => to::StockOnHold,
            from::StockOffHold => to::StockOffHold,
            from::InvoiceNumberAllocated => to::InvoiceNumberAllocated,
            from::RequisitionNumberAllocated => to::RequisitionNumberAllocated,
            from::Repack => to::Repack,
            from::PrescriptionCreated => to::PrescriptionCreated,
            from::PrescriptionDeleted => to::PrescriptionDeleted,
            from::PrescriptionStatusPicked => to::PrescriptionStatusPicked,
            from::PrescriptionStatusVerified => to::PrescriptionStatusVerified,
            from::SensorLocationChanged => to::SensorLocationChanged,
            from::AssetCreated => to::AssetCreated,
            from::AssetUpdated => to::AssetUpdated,
            from::AssetDeleted => to::AssetDeleted,
            from::AssetLogCreated => to::AssetLogCreated,
            from::AssetCatalogueItemCreated => to::AssetCatalogueItemCreated,
            from::QuantityForLineHasBeenSetToZero => to::QuantityForLineHasBeenSetToZero,
            from::AssetCatalogueItemPropertyCreated => to::AssetCatalogueItemPropertyCreated,
            from::AssetLogReasonCreated => to::AssetLogReasonCreated,
            from::AssetLogReasonDeleted => to::AssetLogReasonDeleted,
            from::AssetPropertyCreated => to::AssetPropertyCreated,
            from::VaccineCourseCreated => to::VaccineCourseCreated,
            from::VaccineCourseUpdated => to::VaccineCourseUpdated,
            from::ProgramCreated => to::ProgramCreated,
            from::ProgramUpdated => to::ProgramUpdated,
            from::RnrFormCreated => to::RnrFormCreated,
            from::RnrFormUpdated => to::RnrFormUpdated,
            from::RnrFormFinalised => to::RnrFormFinalised,
            from::VaccinationCreated => to::VaccinationCreated,
            from::VaccinationUpdated => to::VaccinationUpdated,
            from::VaccinationDeleted => to::VaccinationDeleted,
            from::DemographicIndicatorCreated => to::DemographicIndicatorCreated,
            from::DemographicIndicatorUpdated => to::DemographicIndicatorUpdated,
            from::DemographicProjectionCreated => to::DemographicProjectionCreated,
            from::DemographicProjectionUpdated => to::DemographicProjectionUpdated,
        }
    }
}

impl ActivityLogConnector {
    pub fn from_domain(activity_logs: ListResult<ActivityLog>) -> ActivityLogConnector {
        ActivityLogConnector {
            total_count: activity_logs.count,
            nodes: activity_logs
                .rows
                .into_iter()
                .map(ActivityLogNode::from_domain)
                .collect(),
        }
    }
}
