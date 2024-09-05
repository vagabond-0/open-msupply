import { useGql, useAuthContext, useQuery } from '@openmsupply-client/common';
import { getSdk } from '../../operations.generated';

// Should only fire when nameId is not null (userQuery has parameter for that)
// Consumer to also handle error ?
export const useGetDefaultPrice = ({
  itemId,
}: {
  storeId: string;
  itemId: string;
}) => {
  const { client } = useGql();
  const sdk = getSdk(client);
  const { storeId } = useAuthContext();

  const result = useQuery(`defaultPrice${storeId}${itemId}`, () =>
    sdk.getDefaultPrice({ storeId, itemId })
  );

  return { ...result, defaultPrice: result.data?.defaultPrice || 0 };
};
