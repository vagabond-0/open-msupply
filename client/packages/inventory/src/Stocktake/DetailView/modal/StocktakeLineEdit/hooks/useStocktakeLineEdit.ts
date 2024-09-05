import {
  RecordPatch,
  ArrayUtils,
  getErrorMessage,
  useAuthContext,
} from '@openmsupply-client/common';
import { ItemRowFragment, usePackVariant } from '@openmsupply-client/system';
import { StocktakeLineFragment, useStocktake } from './../../../../api';
import { DraftStocktakeLine, DraftLine } from '../utils';
import { useNextItem } from './useNextItem';
import { useDraftStocktakeLines } from './useDraftStocktakeLines';
import { useGetDefaultPrice } from 'packages/inventory/src/Stocktake/api/hooks/utils';
interface useStocktakeLineEditController {
  draftLines: DraftStocktakeLine[];
  update: (patch: RecordPatch<StocktakeLineFragment>) => void;
  addLine: () => void;
  save: () => Promise<{ errorMessages?: string[] }>;
  isSaving: boolean;
  nextItem: ItemRowFragment | null;
}

export const useStocktakeLineEdit = (
  item: ItemRowFragment | null
): useStocktakeLineEditController => {
  const { storeId } = useAuthContext();
  const { id } = useStocktake.document.fields('id');
  const { items } = useStocktake.line.rows();
  const filteredItems = items.filter(item => item.item?.id === item?.id);
  const nextItem = useNextItem(filteredItems, item?.id);
  const [draftLines, setDraftLines] = useDraftStocktakeLines(item);
  const { variantsControl } = usePackVariant(String(item?.id), null);
  const { saveAndMapStructuredErrors: upsertLines, isLoading: isSaving } =
    useStocktake.line.save();
  const { defaultPrice, isFetching } = useGetDefaultPrice({
    storeId,
    itemId: item?.id || '',
  });

  const defaultPackSize = variantsControl?.activeVariant?.packSize || 1;

  const update = (patch: RecordPatch<DraftStocktakeLine>) =>
    setDraftLines(lines =>
      ArrayUtils.immutablePatch(lines, {
        ...patch,
        isUpdated: !patch.isCreated,
      })
    );

  const save = async () => {
    try {
      return await upsertLines(draftLines);
    } catch (e) {
      return { errorMessages: [getErrorMessage(e)] };
    }
  };

  const addLine = () => {
    if (item && !isFetching) {
      setDraftLines(lines => [
        DraftLine.fromItem(id, item, defaultPackSize, defaultPrice),
        ...lines,
      ]);
    }
  };

  return {
    draftLines,
    update,
    addLine,
    save,
    isSaving,
    nextItem,
  };
};
