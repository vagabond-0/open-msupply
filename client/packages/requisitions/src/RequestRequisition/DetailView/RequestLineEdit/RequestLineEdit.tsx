import React from 'react';
import { useTranslation } from '@common/intl';
import {
  ItemRowFragment,
  ReasonOptionsSearchInput,
} from '@openmsupply-client/system';
import {
  BarIcon,
  Box,
  InputWithLabelRow,
  NumericTextInput,
  NumUtils,
  Popover,
  ReasonOptionNodeType,
  TextArea,
  useAuthContext,
  useToggle,
} from '@openmsupply-client/common';
import { DraftRequestLine } from './hooks';
import { Footer } from './Footer';
import { RequestStats } from './ItemCharts/RequestStats';

const INPUT_WIDTH = 100;
const LABEL_WIDTH = '150px';

interface RequestLineEditProps {
  item?: ItemRowFragment | null;
  draft?: DraftRequestLine | null;
  update: (patch: Partial<DraftRequestLine>) => void;
  save?: () => void;
  hasNext: boolean;
  next: ItemRowFragment | null;
  hasPrevious: boolean;
  previous: ItemRowFragment | null;
  isProgram: boolean;
}

export const RequestLineEdit = ({
  draft,
  update,
  save,
  hasNext,
  next,
  hasPrevious,
  previous,
  isProgram,
}: RequestLineEditProps) => {
  const t = useTranslation();
  const { isOn, toggle } = useToggle();
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const { store } = useAuthContext();
  const useConsumptionData =
    store?.preferences?.useConsumptionAndStockFromCustomersForInternalOrders;

  return (
    <Box>
      <Box display="flex" justifyContent="space-between">
        <Box paddingLeft={4} paddingRight={7}>
          {/* Left column content */}
          <InputWithLabelRow
            Input={
              <NumericTextInput
                width={INPUT_WIDTH}
                value={draft?.itemStats.availableStockOnHand}
                disabled
                autoFocus
              />
            }
            labelWidth={LABEL_WIDTH}
            label={t('label.stock-on-hand')}
            sx={{ marginBottom: 1 }}
          />
          {isProgram && useConsumptionData && (
            <>
              <InputWithLabelRow
                Input={
                  <NumericTextInput
                    width={INPUT_WIDTH}
                    value={draft?.incomingUnits}
                    disabled
                  />
                }
                labelWidth={LABEL_WIDTH}
                label={t('label.incoming-stock')}
                sx={{ marginBottom: 1 }}
              />
              <InputWithLabelRow
                Input={
                  <NumericTextInput
                    width={INPUT_WIDTH}
                    value={draft?.outgoingUnits}
                    disabled
                  />
                }
                labelWidth={LABEL_WIDTH}
                label={t('label.outgoing')}
                sx={{ marginBottom: 1 }}
              />
              <InputWithLabelRow
                Input={
                  <NumericTextInput
                    width={INPUT_WIDTH}
                    value={draft?.lossInUnits}
                    disabled
                  />
                }
                labelWidth={LABEL_WIDTH}
                label={t('label.losses')}
                sx={{ marginBottom: 1 }}
              />
              <InputWithLabelRow
                Input={
                  <NumericTextInput
                    width={INPUT_WIDTH}
                    value={draft?.additionInUnits}
                    disabled
                  />
                }
                labelWidth={LABEL_WIDTH}
                label={t('label.additions')}
                sx={{ marginBottom: 1 }}
              />
              <InputWithLabelRow
                Input={
                  <NumericTextInput
                    width={INPUT_WIDTH}
                    value={draft?.expiringUnits}
                    disabled
                  />
                }
                labelWidth={LABEL_WIDTH}
                label={t('label.short-expiry')}
                sx={{ marginBottom: 1 }}
              />
              <InputWithLabelRow
                Input={
                  <NumericTextInput
                    width={INPUT_WIDTH}
                    value={draft?.daysOutOfStock}
                    disabled
                  />
                }
                labelWidth={LABEL_WIDTH}
                label={t('label.days-out-of-stock')}
                sx={{ marginBottom: 1 }}
              />
            </>
          )}
          <InputWithLabelRow
            Input={
              <NumericTextInput
                width={INPUT_WIDTH}
                value={NumUtils.round(
                  draft?.itemStats.averageMonthlyConsumption ?? 0,
                  2
                )}
                decimalLimit={2}
                disabled
              />
            }
            labelWidth={LABEL_WIDTH}
            label={t('label.amc')}
            sx={{ marginBottom: 1 }}
          />
          {isProgram && useConsumptionData && (
            <InputWithLabelRow
              Input={
                <NumericTextInput
                  width={INPUT_WIDTH}
                  value={draft?.itemStats.availableMonthsOfStockOnHand ?? 0}
                  disabled
                  decimalLimit={2}
                  sx={{ marginBottom: 1 }}
                />
              }
              labelWidth={LABEL_WIDTH}
              label={t('label.months-of-stock')}
            />
          )}
        </Box>
        <Box>
          {/* Right column content */}
          <Box display="flex" flexDirection="row">
            <InputWithLabelRow
              Input={
                <NumericTextInput
                  width={INPUT_WIDTH}
                  value={draft?.requestedQuantity}
                  onChange={value => {
                    if (draft?.suggestedQuantity === value) {
                      update({
                        requestedQuantity: value,
                        reason: null,
                      });
                    } else {
                      update({ requestedQuantity: value });
                    }
                  }}
                  onBlur={save}
                />
              }
              labelWidth={LABEL_WIDTH}
              label={t('label.requested-quantity')}
              sx={{ marginBottom: 1 }}
            />
            <Box
              paddingLeft={1}
              paddingTop={0.5}
              onClick={e => {
                toggle();
                setAnchorEl(e?.currentTarget);
              }}
              sx={{ cursor: 'pointer' }}
            >
              <BarIcon
                sx={{
                  color: 'primary.main',
                  backgroundColor: 'background.drawer',
                  borderRadius: '30%',
                  padding: '2px',
                }}
              />
              {isOn && (
                <Popover
                  anchorOrigin={{ vertical: 'center', horizontal: 'left' }}
                  anchorEl={anchorEl}
                  open={isOn}
                >
                  <RequestStats draft={draft} />
                </Popover>
              )}
            </Box>
          </Box>
          <InputWithLabelRow
            Input={
              <NumericTextInput
                width={INPUT_WIDTH}
                value={draft?.suggestedQuantity}
                disabled
              />
            }
            labelWidth={LABEL_WIDTH}
            label={t('label.suggested-quantity')}
            sx={{ marginBottom: 1 }}
          />
          {isProgram && useConsumptionData && (
            <InputWithLabelRow
              Input={
                <ReasonOptionsSearchInput
                  value={draft?.reason}
                  onChange={value => {
                    update({ reason: value });
                  }}
                  width={200}
                  type={ReasonOptionNodeType.RequisitionLineVariance}
                  isDisabled={
                    draft?.requestedQuantity === draft?.suggestedQuantity
                  }
                  onBlur={save}
                />
              }
              labelWidth={'66px'}
              label={t('label.reason')}
              sx={{ marginBottom: 1 }}
            />
          )}
          <InputWithLabelRow
            Input={
              <TextArea
                value={draft?.comment ?? ''}
                onChange={e => update({ comment: e.target.value })}
                InputProps={{
                  sx: {
                    backgroundColor: theme => theme.palette.background.menu,
                  },
                }}
                onBlur={save}
              />
            }
            sx={{ width: 275 }}
            labelWidth={'75px'}
            label={t('label.comment')}
          />
        </Box>
      </Box>
      <Box>
        <Footer
          hasNext={hasNext}
          next={next}
          hasPrevious={hasPrevious}
          previous={previous}
          requisitionNumber={draft?.requisitionNumber}
        />
      </Box>
    </Box>
  );
};
