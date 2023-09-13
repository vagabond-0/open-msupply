import React, { useMemo } from 'react';
import {
  AlertIcon,
  Box,
  Tooltip,
  Typography,
  ValueBar,
} from '@openmsupply-client/common';
import { useFormatNumber, useTranslation } from '@common/intl';
import { useRequest } from '../../../api';

export interface StockDistributionProps {
  availableStockOnHand?: number;
  averageMonthlyConsumption?: number;
  suggestedQuantity?: number;
}

const MIN_MC_WIDTH_TO_SHOW_TEXT = 5;

const MonthlyConsumption = ({
  month,
  flexBasis,
  averageMonthlyConsumption,
  showText,
}: {
  month: number;
  flexBasis: string;
  averageMonthlyConsumption: number;
  showText: boolean;
}) => {
  const t = useTranslation('common');
  const formatNumber = useFormatNumber();
  const text = ` (${month} ${t('label.months', {
    count: month,
  })})`;
  const label = `${formatNumber.round(averageMonthlyConsumption * month)}${
    showText ? text : ''
  }`;

  return <MonthlyBar flexBasis={flexBasis} label={label} />;
};

const MonthlyBar = ({
  label,
  left,
  flexBasis,
}: {
  label: string;
  left?: boolean;
  flexBasis?: string;
}) => {
  const directionStyle = left
    ? { borderLeftWidth: 1, paddingLeft: 8 }
    : { paddingLeft: 3, paddingRight: 8, borderRightWidth: 1 };
  return (
    <Box
      sx={{
        borderWidth: 0,
        borderBottomWidth: 1,
        borderColor: 'gray.dark',
        borderStyle: 'solid',
        height: '20px',
        overflow: 'hidden',
      }}
      style={{ ...directionStyle, textAlign: left ? undefined : 'right' }}
      flexBasis={flexBasis}
    >
      <Tooltip title={label} placement="top">
        <Typography
          variant="body1"
          fontSize={12}
          style={{ color: 'gray.dark' }}
        >
          {label}
        </Typography>
      </Tooltip>
    </Box>
  );
};

const CalculationError = ({
  isAmcZero,
  isSohAndQtyZero,
}: {
  isAmcZero?: boolean;
  isSohAndQtyZero?: boolean;
}) => {
  const t = useTranslation('replenishment');
  const detail = isAmcZero
    ? `: ${t('error.amc-is-zero')}`
    : isSohAndQtyZero
    ? `: ${t('error.soh-and-suggested-quantity-are-zero')}`
    : '';
  const message = `${t('error.unable-to-calculate')}${detail}`;

  return (
    <Box display="flex" padding={1} gap={1}>
      <AlertIcon color="primary" fontSize="small" />
      <Typography variant="body1" fontSize={12} sx={{ color: 'error.main' }}>
        {message}
      </Typography>
    </Box>
  );
};

const StockDistributionContent: React.FC<StockDistributionProps> = ({
  availableStockOnHand = 0,
  averageMonthlyConsumption = 0,
  suggestedQuantity = 0,
}) => {
  if (averageMonthlyConsumption === 0) return <CalculationError isAmcZero />;

  const { maxMonthsOfStock } = useRequest.document.fields('maxMonthsOfStock');
  const targetQuantity = maxMonthsOfStock * averageMonthlyConsumption;
  const t = useTranslation('replenishment');

  if (suggestedQuantity === 0 && availableStockOnHand === 0)
    return <CalculationError isSohAndQtyZero />;

  const targetQuantityWidth =
    availableStockOnHand > targetQuantity
      ? Math.round((100 * targetQuantity) / availableStockOnHand)
      : 100;
  const barWidth =
    availableStockOnHand + suggestedQuantity < targetQuantity
      ? `${Math.round(
          (100 * (availableStockOnHand + suggestedQuantity)) / targetQuantity
        )}%`
      : '100%';

  return useMemo(
    () => (
      <>
        <Typography variant="body1" fontWeight={700} fontSize={12}>
          {t('heading.target-quantity')}
        </Typography>
        <Box
          display="flex"
          alignItems="flex-start"
          width={`${targetQuantityWidth}%`}
          style={{ paddingBottom: 7 }}
        >
          <MonthlyBar
            flexBasis="1px"
            label={targetQuantityWidth > MIN_MC_WIDTH_TO_SHOW_TEXT ? '0' : ''}
            left={true}
          />

          {Array.from({ length: maxMonthsOfStock }, (_, i) => (
            <MonthlyConsumption
              key={i}
              month={i + 1}
              flexBasis={`${100 / maxMonthsOfStock}%`}
              averageMonthlyConsumption={averageMonthlyConsumption}
              showText={targetQuantityWidth > MIN_MC_WIDTH_TO_SHOW_TEXT}
            />
          ))}
        </Box>

        <Box display="flex" alignItems="flex-start" width={barWidth}>
          <ValueBar
            value={availableStockOnHand}
            total={targetQuantity}
            label={t('label.stock-on-hand')}
            colour="gray.main"
            startDivider
          />
          <ValueBar
            value={suggestedQuantity}
            total={targetQuantity}
            label={t('label.suggested-order-quantity')}
            colour="primary.light"
          />
        </Box>
      </>
    ),
    [availableStockOnHand, averageMonthlyConsumption, suggestedQuantity]
  );
};

export const StockDistribution: React.FC<StockDistributionProps> = ({
  availableStockOnHand = 0,
  averageMonthlyConsumption = 0,
  suggestedQuantity = 0,
}) => {
  return (
    <Box
      sx={{
        paddingLeft: 4,
        paddingRight: 4,
        paddingTop: 4,
        paddingBottom: 2,
      }}
    >
      <StockDistributionContent
        availableStockOnHand={availableStockOnHand}
        averageMonthlyConsumption={averageMonthlyConsumption}
        suggestedQuantity={suggestedQuantity}
      />
    </Box>
  );
};
