use repository::{PaginationOption, RepositoryError, Store};
use repository::{StoreFilter, StoreRepository, StoreSort};

use service::{
    get_default_pagination, i64_to_u32, service_provider::GeneralServiceTrait,
    service_provider::ServiceContext, ListError, ListResult,
};

pub fn get_stores(
    ctx: &ServiceContext,
    pagination: Option<PaginationOption>,
    filter: Option<StoreFilter>,
    sort: Option<StoreSort>,
) -> Result<ListResult<Store>, ListError> {
    let pagination = get_default_pagination(pagination, u32::MAX, 1)?;
    let repository = StoreRepository::new(&ctx.connection);

    println!("Change me and see what recompiles");

    Ok(ListResult {
        rows: repository.query(pagination, filter.clone(), sort)?,
        count: i64_to_u32(repository.count(filter)?),
    })
}

pub fn get_store(
    ctx: &ServiceContext,
    filter: StoreFilter,
) -> Result<Option<Store>, RepositoryError> {
    let mut result = StoreRepository::new(&ctx.connection).query_by_filter(filter)?;

    Ok(result.pop())
}

pub struct GeneralService;

impl GeneralServiceTrait for GeneralService {
    fn get_stores(
        &self,
        ctx: &ServiceContext,
        pagination: Option<PaginationOption>,
        filter: Option<StoreFilter>,
        sort: Option<StoreSort>,
    ) -> Result<ListResult<Store>, ListError> {
        get_stores(ctx, pagination, filter, sort)
    }

    fn get_store(
        &self,
        ctx: &ServiceContext,
        filter: StoreFilter,
    ) -> Result<Option<Store>, RepositoryError> {
        get_store(ctx, filter)
    }
}
