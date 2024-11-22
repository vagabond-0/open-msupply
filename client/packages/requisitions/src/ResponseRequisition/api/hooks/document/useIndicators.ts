import { useQuery } from '@openmsupply-client/common';
import { useResponseApi } from '../utils/useResponseApi';

export const useIndicators = (customerNameLinkId: string, periodId: string, enabled: boolean = true) => {
  const api = useResponseApi();
  return useQuery(
    api.keys.indicators(),
    () => api.getIndicators(customerNameLinkId, periodId),
    {
      refetchOnMount: false,
      cacheTime: 0,
      enabled,
    }
  );
};
