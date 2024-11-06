import React from 'react';

import {
  DataTable,
  useColumns,
  TooltipTextCell,
  TableProvider,
  createTableStore,
  FlatButton,
  useTranslation,
  PlusCircleIcon,
  useEditModal,
  IconButton,
  DeleteIcon,
  Typography,
  NumUtils,
} from '@openmsupply-client/common';
import {
  BundledItemFragment,
  ItemVariantFragment,
  useDeleteBundledItem,
} from '../../../api';
import { BundledItemModal } from './BundledItemModal';

export const BundledItemVariants = ({
  variant,
}: {
  variant: ItemVariantFragment;
}) => {
  const t = useTranslation();

  const { isOpen, onClose, onOpen, entity } =
    useEditModal<BundledItemFragment>();

  const isBundledOnOtherVariants = variant.bundlesWith.length > 0;

  return (
    <>
      {isOpen && (
        <BundledItemModal onClose={onClose} bundle={entity} variant={variant} />
      )}
      {!isBundledOnOtherVariants && (
        <BundledVariants variant={variant} onOpen={onOpen} />
      )}
      <FlatButton
        disabled={isBundledOnOtherVariants}
        label={t('label.add-bundled-item')}
        onClick={() => onOpen()}
        startIcon={<PlusCircleIcon />}
        color="primary"
      />

      {isBundledOnOtherVariants && (
        <>
          <Typography
            variant="caption"
            fontStyle="italic"
            color="textSecondary"
            marginBottom={2}
            display="block"
          >
            {t('messages.cannot-bundle')}
          </Typography>
          <BundledOn variant={variant} />
        </>
      )}
    </>
  );
};

const BundledVariants = ({
  variant,
  onOpen,
}: {
  variant: ItemVariantFragment;
  onOpen: (row?: BundledItemFragment) => void;
}) => {
  const t = useTranslation();

  const deleteBundledItem = useDeleteBundledItem({ itemId: variant.itemId });

  const columns = useColumns<BundledItemFragment>([
    {
      key: 'name',
      Cell: TooltipTextCell,
      label: 'label.item-variant',
      accessor: ({ rowData }) =>
        `${rowData.bundledItemVariant?.itemName} - ${rowData.bundledItemVariant?.name}`,
    },
    {
      key: 'ratio',
      Cell: TooltipTextCell,
      label: 'label.ratio',
      description: 'description.bundle-ratio',
    },
    {
      key: 'delete',
      width: 50,
      Cell: props => (
        <IconButton
          icon={<DeleteIcon fontSize="small" color="primary" />}
          label={t('label.delete')}
          onClick={e => {
            e.stopPropagation();
            deleteBundledItem(props.rowData.id);
          }}
        />
      ),
    },
  ]);

  return (
    <TableProvider createStore={createTableStore}>
      <Typography fontWeight="bold">{t('title.bundled-with')}</Typography>
      <DataTable
        id="bundled-item-variants"
        data={variant.bundledItemVariants}
        columns={columns}
        onRowClick={row => onOpen(row)}
        noDataMessage={t('messages.no-bundled-items')}
        dense
      />
    </TableProvider>
  );
};

const BundledOn = ({ variant }: { variant: ItemVariantFragment }) => {
  const t = useTranslation();

  const columns = useColumns<BundledItemFragment>([
    {
      key: 'name',
      Cell: TooltipTextCell,
      label: 'label.item-variant',
      accessor: ({ rowData }) =>
        `${rowData.principalItemVariant?.itemName} - ${rowData.principalItemVariant?.name}`,
    },
    {
      key: 'ratio',
      Cell: TooltipTextCell,
      label: 'label.ratio',
      description: 'description.bundled-item-ratio',
      accessor: ({ rowData }) => NumUtils.round(1 / rowData.ratio, 2),
    },
  ]);

  return (
    <TableProvider createStore={createTableStore}>
      <Typography fontWeight="bold">{t('title.bundled-on')}</Typography>
      <DataTable
        id="bundled-on"
        data={variant.bundlesWith}
        columns={columns}
        dense
      />
    </TableProvider>
  );
};
