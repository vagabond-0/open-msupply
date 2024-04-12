import {
  SortBy,
  FilterByWithBoolean,
  AssetCatalogueItemSortFieldInput,
  AssetCategorySortFieldInput,
  AssetClassSortFieldInput,
  AssetTypeSortFieldInput,
  AssetCategoryFilterInput,
  AssetTypeFilterInput,
  InsertAssetLogReasonInput,
  AssetLogStatusInput,
} from '@openmsupply-client/common';
import { Sdk, AssetCatalogueItemFragment } from './operations.generated';

export type ListParams<T> = {
  first: number;
  offset: number;
  sortBy: SortBy<T>;
  filterBy?: FilterByWithBoolean | null;
};

const itemParsers = {
  toSortField: (sortBy: SortBy<AssetCatalogueItemFragment>) => {
    const fields: Record<string, AssetCatalogueItemSortFieldInput> = {
      catalogue: AssetCatalogueItemSortFieldInput.Catalogue,
      code: AssetCatalogueItemSortFieldInput.Code,
      make: AssetCatalogueItemSortFieldInput.Manufacturer,
      model: AssetCatalogueItemSortFieldInput.Model,
    };

    return fields[sortBy.key] ?? AssetCatalogueItemSortFieldInput.Manufacturer;
  },
};

const logReasonParsers = {
  toLogReasonInsert: (
    input: Partial<InsertAssetLogReasonInput>
  ): InsertAssetLogReasonInput => ({
    id: input.id ?? '',
    // default enum of NotInUse will never be used as it will fail the checkStatus check first
    // and throw an error.
    assetLogStatus: input.assetLogStatus ?? AssetLogStatusInput.NotInUse,
    reason: input.reason ?? '',
  }),
  checkStatus: (status: string): boolean => {
    switch (status) {
      case AssetLogStatusInput.Decommissioned:
        return true;
      case AssetLogStatusInput.Functioning:
        return true;
      case AssetLogStatusInput.FunctioningButNeedsAttention:
        return true;
      case AssetLogStatusInput.NotFunctioning:
        return true;
      case AssetLogStatusInput.NotInUse:
        return true;
      default:
        return false;
    }
  },
};

export const getAssetQueries = (sdk: Sdk) => ({
  get: {
    byId: async (assetCatalogueItemId: string) => {
      const result = await sdk.assetCatalogueItemById({
        assetCatalogueItemId,
      });
      const { assetCatalogueItems } = result;
      if (assetCatalogueItems.__typename === 'AssetCatalogueItemConnector') {
        if (assetCatalogueItems.nodes.length) {
          return assetCatalogueItems.nodes[0];
        }
      }

      throw new Error('Asset catalogue item not found');
    },
    list: async ({
      first,
      offset,
      sortBy,
      filterBy,
    }: ListParams<AssetCatalogueItemFragment>) => {
      const result = await sdk.assetCatalogueItems({
        first,
        offset,
        key: itemParsers.toSortField(sortBy),
        desc: sortBy.isDesc,
        filter: filterBy,
      });

      const items = result?.assetCatalogueItems;

      return items;
    },
    listAll: async ({ sortBy }: ListParams<AssetCatalogueItemFragment>) => {
      const result = await sdk.assetCatalogueItems({
        key: itemParsers.toSortField(sortBy),
        desc: sortBy.isDesc,
      });

      const items = result?.assetCatalogueItems;

      return items;
    },
    categories: async (filter: AssetCategoryFilterInput | undefined) => {
      const result = await sdk.assetCategories({
        filter,
        sort: { key: AssetCategorySortFieldInput.Name, desc: false },
      });
      const categories = result?.assetCategories;

      return categories;
    },
    classes: async () => {
      const result = await sdk.assetClasses({
        sort: { key: AssetClassSortFieldInput.Name, desc: false },
      });
      const classes = result?.assetClasses;

      return classes;
    },
    types: async (filter: AssetTypeFilterInput | undefined) => {
      const result = await sdk.assetTypes({
        filter,
        sort: { key: AssetTypeSortFieldInput.Name, desc: false },
      });
      const types = result?.assetTypes;

      return types;
    },
    logReasons: async (storeId: string) => {
      const result = await sdk.assetLogReasons({
        storeId,
        // TODO functioning filter - can add later (currently not sure if query params will use this)
        filter: {
          assetLogStatus: undefined,
          id: undefined,
          reason: undefined,
        },
      });
      return result?.assetLogReasons;
    },
  },
  insertLogReason: async (input: Partial<InsertAssetLogReasonInput>) => {
    if (!logReasonParsers.checkStatus(input.assetLogStatus ?? '')) {
      throw new Error('Cannot parse status');
    }
    const result = await sdk.insertAssetLogReason({
      input: logReasonParsers.toLogReasonInsert(input),
    });
    if (
      result.centralServer.logReason.insertAssetLogReason.__typename ===
      'AssetLogReasonNode'
    ) {
      return result.centralServer.logReason.insertAssetLogReason;
    }

    throw new Error('Could not insert reason');
  },
  deleteLogReason: async (reasonId: string) => {
    const result = await sdk.deleteLogReason({ reasonId });
    if (
      result.centralServer.logReason.deleteLogReason.__typename ===
      'DeleteResponse'
    ) {
      return result.centralServer.logReason;
    }
    throw new Error('Could not delete reason');
  },
});
