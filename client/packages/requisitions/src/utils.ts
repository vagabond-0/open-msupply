import { RequisitionNodeStatus } from '@openmsupply-client/common';
import { Requisition, RequisitionRow } from './types';

export const isRequisitionEditable = (requisition: Requisition): boolean => {
  return (
    requisition.status === RequisitionNodeStatus.Draft ||
    requisition.status === RequisitionNodeStatus.New
  );
};

// TODO: When supplier requisition statuses are finalised, this function should be passed
// `t` and should properly translate the status.
export const getSupplierRequisitionTranslator =
  () =>
  (currentStatus: RequisitionNodeStatus): string =>
    currentStatus;

// TODO: When supplier requisition statuses are finalised, this function should
// return the next status rather than just returning the current status
export const getNextSupplierRequisitionStatus = (
  currentStatus: RequisitionNodeStatus
): RequisitionNodeStatus => {
  const statuses = getSupplierRequisitionStatuses();
  const currentIdx = statuses.findIndex(
    requisitionStatus => requisitionStatus === currentStatus
  );

  if (currentIdx < 0) {
    throw new Error('Cannot find index of current supplier requisition idx');
  }

  const nextStatus = statuses[currentIdx + 1];

  if (!nextStatus) {
    throw new Error('Cannot find next supplier requisition status');
  }

  return nextStatus;
};

export const getSupplierRequisitionStatuses = (): RequisitionNodeStatus[] => [
  RequisitionNodeStatus.Draft,
  RequisitionNodeStatus.New,
  RequisitionNodeStatus.Sent,
];

export const canDeleteRequisition = (requisitionRow: RequisitionRow): boolean =>
  requisitionRow.status === RequisitionNodeStatus.Draft;

export const getNextStatusText = (status: RequisitionNodeStatus): string => {
  const nextStatus = getNextSupplierRequisitionStatus(status);
  const translation = getSupplierRequisitionTranslator()(nextStatus);
  return translation;
};

export const createStatusLog = (status: RequisitionNodeStatus) => {
  if (status === 'DRAFT') {
    return {
      DRAFT: new Date().toISOString(),
      NEW: null,
      FINALISED: null,
      SENT: null,
    };
  }

  if (status === 'FINALISED') {
    return {
      DRAFT: new Date().toISOString(),
      NEW: new Date().toISOString(),
      FINALISED: new Date().toISOString(),
      SENT: null,
    };
  }

  return {
    DRAFT: new Date().toISOString(),
    NEW: new Date().toISOString(),
    FINALISED: new Date().toISOString(),
    SENT: new Date().toISOString(),
  };
};
