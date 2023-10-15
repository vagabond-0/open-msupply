import React, { FC, useEffect, useState } from 'react';

import {
  AppFooterPortal,
  Box,
  DetailPanel,
  AppFooter,
  Routes,
  Route,
  RouteBuilder,
  Navigate,
  useLocation,
  useHostContext,
  useGetPageTitle,
  useAuthContext,
  useNotification,
  useTranslation,
  SnackbarProvider,
  BarcodeScannerProvider,
  AlarmIcon,
  Typography,
  useGql,
  AlarmVariant,
  ChevronDownIcon,
} from '@openmsupply-client/common';
import { AppDrawer, AppBar, Footer, NotFound } from './components';
import { CommandK } from './CommandK';
import { AppRoute } from '@openmsupply-client/config';
import { Settings } from './Admin/Settings';
import {
  DashboardRouter,
  CatalogueRouter,
  DistributionRouter,
  ReplenishmentRouter,
  InventoryRouter,
  DispensaryRouter,
  ColdChainRouter,
} from './routers';
import { RequireAuthentication } from './components/Navigation/RequireAuthentication';
import { QueryErrorHandler } from './QueryErrorHandler';
import { Sync } from './components/Sync';
import {
  AlarmFragment,
  getSdk,
} from './api/operations.generated';

const NotifyOnLogin = () => {
  const { success } = useNotification();
  const { store, storeId } = useAuthContext();
  const { name } = store || {};
  const t = useTranslation('app');
  useEffect(() => {
    if (!!name) success(t('login.store-changed', { store: name }))();
  }, [storeId]);

  return <></>;
};

export const Site: FC = () => {
  const location = useLocation();
  const getPageTitle = useGetPageTitle();
  const { setPageTitle } = useHostContext();
  const { client } = useGql();
  const sdk = getSdk(client);
  const { error } = useNotification();
  const [alarms, setAlarms] = useState<AlarmFragment[]>([]);
  const [showAllAlarms, setShowAllAlarms] = useState(false);

  const onLoad = async () => {
    setTimeout(async () => {
      try {
        setAlarms((await sdk.getColdChainAlarms()).coldChainAlarms);
      } catch (e) {
        error(String(e))();
      }
    }, 10000);
  };

  useEffect(() => {
    onLoad();
  }, []);

  useEffect(() => {
    setPageTitle(getPageTitle(location.pathname));
  }, [location]);

  return (
    <RequireAuthentication>
      <CommandK>
        <SnackbarProvider maxSnack={3}>
          <BarcodeScannerProvider>
            <AppDrawer />
            <Box
              flex={1}
              display="flex"
              flexDirection="column"
              overflow="hidden"
            >
              {alarms.length > 0 && (
                <div
                  style={{
                    background: '#FEF5F2',
                    borderBottom: '3px solid #E95C30',
                    display: 'flex',
                    flexDirection: 'column',
                  }}
                  onClick={() => setShowAllAlarms(!showAllAlarms)}
                >
                  {alarms.flatMap((alarm, index) => {
                    if ((index > 1 && showAllAlarms) || index == 0) {
                      return (
                        <div
                          style={{
                            height: '50px',

                            display: 'flex',
                            flexDirection: 'row',
                            alignItems: 'center',
                          }}
                        >
                          <div
                            style={{
                              paddingLeft: '10px',
                              paddingRight: '10px',
                            }}
                          >
                            <AlarmIcon />
                          </div>
                          <Typography
                            sx={{ fontWeight: 'bold' }}
                            variant="subtitle2"
                          >
                            {alarm.alarmVariant ==
                            AlarmVariant.TemperatureExcursion
                              ? 'Temperature excursion detected!'
                              : alarm.alarmVariant == AlarmVariant.DoorOpen
                              ? 'Open door detected'
                              : 'Firdge powered off'}
                          </Typography>
                          {alarm.temperature && (
                            <>
                              <Typography
                                style={{
                                  paddingLeft: '10px',
                                  paddingRight: '10px',
                                }}
                                variant="subtitle2"
                              >
                                |
                              </Typography>
                              <Typography variant="subtitle2">
                                Temperature
                              </Typography>
                              <Typography
                                style={{
                                  paddingLeft: '2x',
                                  paddingRight: '2px',
                                }}
                                variant="subtitle2"
                              >
                                :
                              </Typography>
                              <Typography
                                sx={{ fontWeight: 'bold' }}
                                variant="subtitle2"
                              >
                                {`${alarm.temperature} °C`}
                              </Typography>
                            </>
                          )}
                          <Typography
                            style={{
                              paddingLeft: '10px',
                              paddingRight: '10px',
                            }}
                            variant="subtitle2"
                          >
                            |
                          </Typography>
                          <Typography variant="subtitle2">Device</Typography>
                          <Typography
                            style={{
                              paddingLeft: '2px',
                              paddingRight: '2px',
                            }}
                            variant="subtitle2"
                          >
                            :
                          </Typography>
                          <Typography
                            sx={{ fontWeight: 'bold' }}
                            variant="subtitle2"
                          >
                            {alarm.sensor.name}
                          </Typography>
                          {alarm.sensor.location && (
                            <>
                              <Typography
                                style={{
                                  paddingLeft: '10px',
                                  paddingRight: '10px',
                                }}
                                variant="subtitle2"
                              >
                                |
                              </Typography>
                              <Typography variant="subtitle2">
                                Location
                              </Typography>
                              <Typography
                                style={{
                                  paddingLeft: '2x',
                                  paddingRight: '2px',
                                }}
                                variant="subtitle2"
                              >
                                :
                              </Typography>
                              <Typography
                                sx={{ fontWeight: 'bold' }}
                                variant="subtitle2"
                              >
                                {alarm.sensor.location.name}
                              </Typography>
                            </>
                          )}

                          {alarms.length > 1 &&
                            index === 0 &&
                            !showAllAlarms && (
                              <div
                                style={{
                                  flexGrow: 1,
                                  display: 'flex',
                                  flexDirection: 'row',
                                  justifyContent: 'flex-end',
                                  paddingRight: '5px',
                                }}
                              >
                                <Typography
                                  sx={{
                                    fontWeight: 'bold',
                                  }}
                                  variant="subtitle2"
                                >
                                  {`${alarms.length - 1} more`}
                                </Typography>
                                <ChevronDownIcon />
                              </div>
                            )}

                          {alarms.length > 1 &&
                            index === 0 &&
                            showAllAlarms && (
                              <div
                                style={{
                                  flexGrow: 1,
                                  display: 'flex',
                                  flexDirection: 'row',
                                  justifyContent: 'flex-end',
                                  paddingRight: '5px',
                                }}
                              >
                                <Typography
                                  sx={{
                                    fontWeight: 'bold',
                                  }}
                                  variant="subtitle2"
                                >
                                  Show less
                                </Typography>
                                <div
                                  style={{
                                    transform: 'rotate(180deg) scaleX(-1)',
                                  }}
                                >
                                  <ChevronDownIcon />
                                </div>
                              </div>
                            )}
                        </div>
                      );
                    }
                  })}
                </div>
              )}

              <AppBar />
              <NotifyOnLogin />
              <Box display="flex" flex={1} overflow="auto">
                <Routes>
                  <Route
                    path={RouteBuilder.create(AppRoute.Dashboard)
                      .addWildCard()
                      .build()}
                    element={<DashboardRouter />}
                  />
                  <Route
                    path={RouteBuilder.create(AppRoute.Catalogue)
                      .addWildCard()
                      .build()}
                    element={<CatalogueRouter />}
                  />
                  <Route
                    path={RouteBuilder.create(AppRoute.Distribution)
                      .addWildCard()
                      .build()}
                    element={<DistributionRouter />}
                  />
                  <Route
                    path={RouteBuilder.create(AppRoute.Replenishment)
                      .addWildCard()
                      .build()}
                    element={<ReplenishmentRouter />}
                  />
                  <Route
                    path={RouteBuilder.create(AppRoute.Inventory)
                      .addWildCard()
                      .build()}
                    element={<InventoryRouter />}
                  />
                  <Route
                    path={RouteBuilder.create(AppRoute.Dispensary)
                      .addWildCard()
                      .build()}
                    element={<DispensaryRouter />}
                  />
                  <Route
                    path={RouteBuilder.create(AppRoute.Coldchain)
                      .addWildCard()
                      .build()}
                    element={<ColdChainRouter />}
                  />
                  <Route
                    path={RouteBuilder.create(AppRoute.Admin)
                      .addWildCard()
                      .build()}
                    element={<Settings />}
                  />
                  <Route
                    path={RouteBuilder.create(AppRoute.Sync)
                      .addWildCard()
                      .build()}
                    element={<Sync />}
                  />
                  <Route
                    path="/"
                    element={
                      <Navigate
                        replace
                        to={RouteBuilder.create(AppRoute.Dashboard).build()}
                      />
                    }
                  />
                  <Route path="*" element={<NotFound />} />
                </Routes>
              </Box>
              <AppFooter />
              <AppFooterPortal SessionDetails={<Footer />} />
            </Box>
            <DetailPanel />
            <QueryErrorHandler />
          </BarcodeScannerProvider>
        </SnackbarProvider>
      </CommandK>
    </RequireAuthentication>
  );
};
