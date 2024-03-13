import React, { useMemo, useState, useEffect } from 'react';
import {
  ArrowRightIcon,
  useTranslation,
  useNotification,
  InvoiceNodeStatus,
  SplitButton,
  SplitButtonOption,
  useConfirmationModal,
} from '@openmsupply-client/common';
import { getNextInboundStatus, getStatusTranslation } from '../../../utils';
import { useReturns } from '../../api';

const getStatusOptions = (
  currentStatus: InvoiceNodeStatus,
  getButtonLabel: (status: InvoiceNodeStatus) => string
): SplitButtonOption<InvoiceNodeStatus>[] => {
  const options: [
    SplitButtonOption<InvoiceNodeStatus>,
    SplitButtonOption<InvoiceNodeStatus>,
    SplitButtonOption<InvoiceNodeStatus>,
    SplitButtonOption<InvoiceNodeStatus>,
    SplitButtonOption<InvoiceNodeStatus>,
  ] = [
    {
      value: InvoiceNodeStatus.New,
      label: getButtonLabel(InvoiceNodeStatus.New),
      isDisabled: true,
    },
    {
      value: InvoiceNodeStatus.Picked,
      label: getButtonLabel(InvoiceNodeStatus.Picked),
      isDisabled: true,
    },
    {
      value: InvoiceNodeStatus.Shipped,
      label: getButtonLabel(InvoiceNodeStatus.Shipped),
      isDisabled: true,
    },
    {
      value: InvoiceNodeStatus.Delivered,
      label: getButtonLabel(InvoiceNodeStatus.Delivered),
      isDisabled: true,
    },
    {
      value: InvoiceNodeStatus.Verified,
      label: getButtonLabel(InvoiceNodeStatus.Verified),
      isDisabled: true,
    },
  ];

  if (currentStatus === InvoiceNodeStatus.New) {
    // When new, can change to delivered or verified
    options[3].isDisabled = false;
    options[4].isDisabled = false;
  }

  if (currentStatus === InvoiceNodeStatus.Delivered) {
    // When delivered, can change to verified
    options[4].isDisabled = false;
  }

  return options;
};

const getManualStatusOptions = (
  currentStatus: InvoiceNodeStatus,
  getButtonLabel: (status: InvoiceNodeStatus) => string
): SplitButtonOption<InvoiceNodeStatus>[] => {
  const options: [
    SplitButtonOption<InvoiceNodeStatus>,
    SplitButtonOption<InvoiceNodeStatus>,
    SplitButtonOption<InvoiceNodeStatus>,
  ] = [
    {
      value: InvoiceNodeStatus.New,
      label: getButtonLabel(InvoiceNodeStatus.New),
      isDisabled: true,
    },
    {
      value: InvoiceNodeStatus.Delivered,
      label: getButtonLabel(InvoiceNodeStatus.Delivered),
      isDisabled: true,
    },
    {
      value: InvoiceNodeStatus.Verified,
      label: getButtonLabel(InvoiceNodeStatus.Verified),
      isDisabled: true,
    },
  ];

  if (currentStatus === InvoiceNodeStatus.New) {
    // When the status is new, delivered and verified are available to
    // select.
    options[1].isDisabled = false;
  }

  // When the status is delivered, only verified is available to select.
  if (currentStatus === InvoiceNodeStatus.Delivered) {
    options[2].isDisabled = false;
  }

  return options;
};

const getNextStatusOption = (
  status: InvoiceNodeStatus,
  options: SplitButtonOption<InvoiceNodeStatus>[]
): SplitButtonOption<InvoiceNodeStatus> | null => {
  if (!status) return options[0] ?? null;

  const nextStatus = getNextInboundStatus(status);
  const nextStatusOption = options.find(o => o.value === nextStatus);
  return nextStatusOption || null;
};

const getButtonLabel =
  (t: ReturnType<typeof useTranslation>) =>
  (invoiceStatus: InvoiceNodeStatus): string => {
    return t('button.save-and-confirm-status', {
      status: t(getStatusTranslation(invoiceStatus)),
    });
  };

const useStatusChangeButton = () => {
  const { success, error } = useNotification();
  const t = useTranslation('distribution');
  const { mutateAsync } = useReturns.document.updateInboundReturn();

  const {
    data: { status, lines, onHold, id } = {
      status: InvoiceNodeStatus.New,
      lines: { totalCount: 0 },
      onHold: false,
    },
  } = useReturns.document.inboundReturn();

  const lineCount = lines.totalCount;

  // TODO manual vs automatic returns
  // const isManuallyCreated = !linkedShipment?.id;
  const isManuallyCreated = true;

  const options = useMemo(
    () =>
      isManuallyCreated
        ? getManualStatusOptions(status, getButtonLabel(t))
        : getStatusOptions(status, getButtonLabel(t)),
    [status, getButtonLabel]
  );

  const [selectedOption, setSelectedOption] =
    useState<SplitButtonOption<InvoiceNodeStatus> | null>(() =>
      getNextStatusOption(status, options)
    );

  const onConfirmStatusChange = async () => {
    if (!selectedOption || !id) return null;
    try {
      await mutateAsync({ id, status: selectedOption.value });

      success(t('messages.return-saved'))();
    } catch (e) {
      error(t('messages.error-saving-return'))();
    }
  };

  const getConfirmation = useConfirmationModal({
    title: t('heading.are-you-sure'),
    message: t('messages.confirm-status-as', {
      status: selectedOption?.value
        ? getStatusTranslation(selectedOption?.value)
        : '',
    }),
    onConfirm: onConfirmStatusChange,
  });

  // When the status of the invoice changes (after an update), set the selected
  // option to the next status. It would be set to the current status, which is
  // now a disabled option.
  useEffect(() => {
    setSelectedOption(() => getNextStatusOption(status, options));
  }, [status, options]);

  return {
    options,
    selectedOption,
    setSelectedOption,
    getConfirmation,
    onHold,
    lineCount,
  };
};

export const StatusChangeButton = () => {
  const {
    options,
    selectedOption,
    setSelectedOption,
    getConfirmation,
    onHold,
    lineCount,
  } = useStatusChangeButton();
  const isDisabled = useReturns.utils.inboundIsDisabled();
  const t = useTranslation();
  const noLines = lineCount === 0;

  if (!selectedOption) return null;
  if (isDisabled) return null;

  const onStatusClick = () => {
    return getConfirmation();
  };

  return (
    <SplitButton
      label={noLines ? t('messages.no-lines') : ''}
      isDisabled={noLines || onHold}
      options={options}
      selectedOption={selectedOption}
      onSelectOption={setSelectedOption}
      Icon={<ArrowRightIcon />}
      onClick={onStatusClick}
    />
  );
};
