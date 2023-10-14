import React, { useEffect, useState } from 'react';
import { useTranslation } from '@common/intl';
import {
  BasicTextInput,
  ErrorWithDetails,
  ErrorWithDetailsProps,
  Grid,
  LoadingButton,
  NumericTextInput,
  SaveIcon,
  Typography,
  useAuthContext,
  useGql,
  useNotification,
} from '@openmsupply-client/common';
import { Setting } from './Setting';
import { getSdk } from '../api/operations.generated';

const useEmdSettings = () => {
  const { success } = useNotification();
  const { client } = useGql();
  const { storeId } = useAuthContext();
  const sdk = getSdk(client);
  const [ip, setIp] = useState('');
  const [intervalSeconds, setIntervalSeconds] = useState(10);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<undefined | ErrorWithDetailsProps>();

  const onLoad = async () => {
    setIsLoading(true);
    try {
      let { emdSettings } = await sdk.getEmdSettings();
      setIp(emdSettings.ip);
      setIntervalSeconds(emdSettings.intervalSeconds);
    } catch (e) {
      setError({ error: 'Failed to load emd settings', details: String(e) });
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    onLoad();
  }, []);

  const save = async () => {
    setError(undefined);
    setIsLoading(true);
    try {
      await sdk.updateEmdSettings({
        storeId,
        input: {
          ip,
          intervalSeconds,
        },
      });
      success('Settings saved')();
    } catch (e) {
      setError({ error: 'Failed to save emd settings', details: String(e) });
    } finally {
      setIsLoading(false);
    }
  };
  return {
    setIp,
    setIntervalSeconds,
    save,
    intervalSeconds,
    ip,
    isLoading,
    error,
  };
};

export const EmdSettings = ({}) => {
  const t = useTranslation('app');
  const {
    intervalSeconds,
    ip,
    isLoading,
    setIp,
    save,
    error,
    setIntervalSeconds,
  } = useEmdSettings();

  return (
    <Grid container>
      <Typography variant="h5" color="primary" style={{ paddingBottom: 25 }}>
        Emd Settings
      </Typography>

      <form
        style={{ width: '100%' }}
        onKeyDown={e => {
          if (e.key === 'Enter') {
            save();
          }
        }}
      >
        <Setting
          title={'IP'}
          component={
            <BasicTextInput value={ip} onChange={e => setIp(e.target.value)} />
          }
        />
        <Setting
          title={'Interval Seconds'}
          component={
            <NumericTextInput
              value={intervalSeconds}
              onChange={seconds => setIntervalSeconds(seconds || 0)}
            />
          }
        />

        <Grid item justifyContent="flex-end" width="100%" display="flex">
          <LoadingButton
            isLoading={isLoading}
            startIcon={<SaveIcon />}
            type="submit"
            variant="contained"
            sx={{ fontSize: '12px' }}
            onClick={save}
          >
            {t('button.save')}
          </LoadingButton>
        </Grid>
      </form>
      {error && <ErrorWithDetails {...error} />}
    </Grid>
  );
};
