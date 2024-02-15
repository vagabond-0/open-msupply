import React, { FC, useCallback } from 'react';
import {
  TableProvider,
  createTableStore,
  // useEditModal,
  DetailViewSkeleton,
  AlertModal,
  useNavigate,
  RouteBuilder,
  useTranslation,
  createQueryParamsStore,
  // DetailTabs,
  // ModalMode,
} from '@openmsupply-client/common';
// import { toItemRow, ActivityLogList } from '@openmsupply-client/system';
// import { ContentArea } from './ContentArea';
import { StockOutItem } from '../../types';
// import { Toolbar } from './Toolbar';
// import { Footer } from './Footer';
// import { AppBarButtons } from './AppBarButtons';
// import { SidePanel } from './SidePanel';
import { useReturns } from '../api';
import { AppRoute } from '@openmsupply-client/config';
// import { Draft } from '../..';
import { StockOutLineFragment } from '../../StockOut';
// import { OutboundLineEdit } from './OutboundLineEdit';

export const DetailView: FC = () => {
  // const isDisabled = useReturn.utils.isDisabled();
  // const { entity, mode, onOpen, onClose, isOpen, setMode } =
  //   useEditModal<Draft>();
  const { data, isLoading } = useReturns.document.invoiceByNumber();
  const t = useTranslation('distribution');
  const navigate = useNavigate();
  // const onRowClick = useCallback(
  //   (item: StockOutLineFragment | StockOutItem) => {
  //     onOpen({ item: toItemRow(item) });
  //   },
  //   [toItemRow, onOpen]
  // );
  // const onAddItem = (draft?: Draft) => {
  //   onOpen(draft);
  //   setMode(ModalMode.Create);
  // };

  if (isLoading) return <DetailViewSkeleton hasGroupBy={true} hasHold={true} />;

  // const tabs = [
  //   {
  //     Component: (
  //       <ContentArea
  //         // onRowClick={!isDisabled ? onRowClick : null}
  //         onAddItem={onAddItem}
  //       />
  //     ),
  //     value: 'Details',
  //   },
  //   {
  //     Component: <ActivityLogList recordId={data?.id ?? ''} />,
  //     value: 'Log',
  //   },
  // ];

  return (
    <React.Suspense
      fallback={<DetailViewSkeleton hasGroupBy={true} hasHold={true} />}
    >
      {data ? (
        <TableProvider
          createStore={createTableStore}
          queryParamsStore={createQueryParamsStore<
            StockOutLineFragment | StockOutItem
          >({
            initialSortBy: {
              key: 'itemName',
            },
          })}
        >
          {JSON.stringify(data)}
          {/* <AppBarButtons onAddItem={onAddItem} /> */}
          {/* {isOpen && (
            <OutboundLineEdit
              draft={entity}
              mode={mode}
              isOpen={isOpen}
              onClose={onClose}
            />
          )} */}

          {/* <Toolbar /> */}
          {/* <DetailTabs tabs={tabs} /> */}
          {/* <Footer /> */}
          {/* <SidePanel /> */}
        </TableProvider>
      ) : (
        <AlertModal
          open={true}
          onOk={() =>
            navigate(
              RouteBuilder.create(AppRoute.Distribution)
                .addPart(AppRoute.OutboundReturn)
                .build()
            )
          }
          title={t('error.shipment-not-found')}
          message={t('messages.click-to-return-to-shipments')}
        />
      )}
    </React.Suspense>
  );
};
