import {
  useColumns,
  ColumnAlign,
  Column,
  ColumnDescription,
  SortBy,
} from '@openmsupply-client/common';
import { useFormatDateTime } from '@common/intl';
import {
  ProgramEventFragment,
  EncounterRowFragment,
  useDocumentRegistry,
  DocumentRegistryFragment,
} from '@openmsupply-client/programs';

export const encounterEventCellValue = (events: ProgramEventFragment[]) => {
  // just take the name of the first event
  return events[0]?.data ?? '';
};

interface useEncounterListColumnsProps {
  onChangeSortBy: (column: Column<any>) => void;
  sortBy: SortBy<EncounterRowFragment>;
  includePatient?: boolean;
}

export const useEncounterListColumns = ({
  onChangeSortBy,
  sortBy,
  includePatient = false,
}: useEncounterListColumnsProps) => {
  const { localisedDate, localisedTime } = useFormatDateTime();
  const { data: documentRegistries } =
    useDocumentRegistry.get.documentRegistries();
  includePatient;
  // document type -> parent
  const documentRegistryParentMap =
    documentRegistries?.nodes?.reduce((prev, cur) => {
      if (!cur.parentId) return prev;
      const parent = documentRegistries?.nodes.find(
        it => it.id === cur.parentId
      );
      if (parent) prev.set(cur.documentType, parent);
      return prev;
    }, new Map<string, DocumentRegistryFragment>()) ??
    new Map<string, DocumentRegistryFragment>();

  const columnList: ColumnDescription<EncounterRowFragment>[] = [
    {
      key: 'encounter-type',
      label: 'label.encounter-type',
      sortable: false,
      accessor: ({ rowData }) => rowData?.document.documentRegistry?.name,
    },
    {
      key: 'program',
      label: 'label.program',
      accessor: ({ rowData }) =>
        documentRegistryParentMap.get(rowData.type)?.name,
    },
    {
      key: 'startDatetime',
      label: 'label.date',
      accessor: ({ rowData }) => rowData?.startDatetime,
      formatter: dateString =>
        dateString ? localisedDate((dateString as string) || '') : '',
    },
    {
      key: 'startTime',
      label: 'label.encounter-start',
      sortable: false,
      accessor: ({ rowData }) => rowData?.startDatetime,
      formatter: dateString =>
        dateString ? localisedTime((dateString as string) || '') : '',
    },
    {
      key: 'endDatetime',
      label: 'label.encounter-end',
      formatter: dateString =>
        dateString ? localisedTime((dateString as string) || '') : '',
    },
  ];
  if (includePatient)
    columnList.push({
      key: 'patientId',
      label: 'label.patient',
      accessor: ({ rowData }) => rowData?.patient?.name,
    });
  columnList.push({
    key: 'events',
    label: 'label.additional-info',
    sortable: false,
    formatter: events =>
      encounterEventCellValue((events as ProgramEventFragment[]) ?? []),
  });
  columnList.push({
    key: 'effectiveStatus',
    label: 'label.status',
    sortable: false,
    align: ColumnAlign.Right,
    width: 175,
  });

  const columns = useColumns<EncounterRowFragment>(
    columnList,
    { onChangeSortBy, sortBy },
    [sortBy]
  );

  return columns;
};
