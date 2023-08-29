import {
  useColumns,
  ColumnAlign,
  Column,
  SortBy,
  ColumnDescription,
} from '@openmsupply-client/common';
import { useFormatDateTime } from '@common/intl';
import { ContactTraceRowFragment } from '@openmsupply-client/programs';

export interface ContactTraceListColumnsProps {
  onChangeSortBy: (column: Column<ContactTraceRowFragment>) => void;
  sortBy: SortBy<ContactTraceRowFragment>;
}

export const useContactTraceListColumns = ({
  onChangeSortBy,
  sortBy,
}: ContactTraceListColumnsProps) => {
  const { localisedDate } = useFormatDateTime();

  const columnList: ColumnDescription<ContactTraceRowFragment>[] = [
    {
      key: 'programName',
      label: 'label.program',
      accessor: ({ rowData }) => rowData.program.name,
      sortable: false,
    },
    {
      key: 'datetime',
      label: 'label.date-created',
      formatter: dateString =>
        dateString ? localisedDate((dateString as string) || '') : '',
    },
    {
      key: 'firstName',
      label: 'label.first-name',
    },
    {
      key: 'lastName',
      label: 'label.last-name',
    },
    {
      key: 'gender',
      label: 'label.gender',
    },
    {
      key: 'dateOfBirth',
      label: 'label.age',
      align: ColumnAlign.Right,
      width: 175,
      accessor: ({ rowData }) => rowData.age,
    },
  ];

  const columns = useColumns<ContactTraceRowFragment>(
    columnList,
    {
      sortBy,
      onChangeSortBy,
    },
    [sortBy, onChangeSortBy]
  );

  return columns;
};
