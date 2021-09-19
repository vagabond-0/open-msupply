import { SortRule, SortBy } from '@openmsupply-client/common';
import { KeyOf, ObjectWithStringKeys } from './../../types/utility';
import { useState } from 'react';
import { useSortBy } from '../useSortBy';

const parseValue = (object: any, key: string) => {
  const value = object[key];
  if (typeof value === 'string') {
    const valueAsNumber = Number.parseFloat(value);

    if (!Number.isNaN(valueAsNumber)) return valueAsNumber;
    return value.toUpperCase(); // ignore case
  }
  return value;
};

const getDataSorter = (sortKey: string, desc: boolean) => (a: any, b: any) => {
  const valueA = parseValue(a, sortKey);
  const valueB = parseValue(b, sortKey);

  if (valueA < valueB) {
    return desc ? 1 : -1;
  }
  if (valueA > valueB) {
    return desc ? -1 : 1;
  }

  return 0;
};

interface SortedDataState<T extends ObjectWithStringKeys> {
  sortedData: T[];
  sortBy: SortBy<T>;
  onChangeSortBy: (newSortBy: SortBy<T>) => void;
}

export const useSortedData = <T extends Record<string, unknown>>(
  data: T[],
  initialSortBy: SortRule<T>
): SortedDataState<T> => {
  const { sortBy, onChangeSortBy } = useSortBy(initialSortBy);
  const [sortedData, setSortedData] = useState(data);

  const wrapped = (newSortRule: SortRule<T>) => {
    onChangeSortBy(newSortRule);
    const sorter = getDataSorter(newSortRule.key, !!newSortRule.isDesc);
    setSortedData(data.sort(sorter));
  };

  return { sortedData, sortBy, onChangeSortBy: wrapped };
};
