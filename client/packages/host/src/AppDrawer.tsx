import React from 'react';
import {
  Box,
  Dashboard,
  Divider,
  Drawer,
  List,
  MSupplyGuy,
  Messages,
  Power,
  Radio,
  Reports,
  Settings,
  Stock,
  Suppliers,
  Theme,
  Tools,
  UnstyledIconButton,
  styled,
  useDrawer,
  useMediaQuery,
  useTheme,
  useTranslation,
} from '@openmsupply-client/common';
import { AppRoute } from '@openmsupply-client/config';
import { AppNavLink } from '@openmsupply-client/common/src/ui/components/NavLink';
import { Property } from 'csstype';

const CustomersNav = React.lazy(
  () => import('@openmsupply-client/customers/src/Nav')
);

const ToolbarIconContainer = styled(Box)(({ theme }) => ({
  display: 'flex',
  height: 90,
  justifyContent: 'center',
  alignItems: 'center',
  padding: '0 8px',
  ...theme.mixins.toolbar,
}));

const commonListContainerStyles = {
  display: 'flex',
  flexDirection: 'column' as Property.FlexDirection,
  justifyContent: 'space-between',
  alignItems: 'center',
};

const LowerListContainer = styled(Box)({
  ...commonListContainerStyles,
  height: 151,
  position: 'static',
  bottom: 0,
});

const UpperListContainer = styled(Box)({
  ...commonListContainerStyles,
  flex: 1,
  height: '100%',
  msOverflowStyle: 'none',
  overflow: 'scroll',
  scrollbarWidth: 'none',
  '&::-webkit-scrollbar': {
    display: 'none',
  },
});

const StyledDivider = styled(Divider)({
  backgroundColor: '#555770',
  marginLeft: 8,
  width: 152,
});

const drawerWidth = 200;

const getDrawerCommonStyles = (theme: Theme) => ({
  backgroundColor: theme.palette.background.menu,
  overflow: 'hidden',
});

const openedMixin = (theme: Theme) => ({
  ...getDrawerCommonStyles(theme),
  width: drawerWidth,

  transition: theme.transitions.create('width', {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.enteringScreen,
  }),
});

const closedMixin = (theme: Theme) => ({
  ...getDrawerCommonStyles(theme),

  transition: theme.transitions.create('width', {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.leavingScreen,
  }),
  [theme.breakpoints.up('sm')]: {
    width: theme.spacing(10),
  },
});

const StyledDrawer = styled(Drawer)(({ open, theme }) => {
  return {
    position: 'relative',
    whiteSpace: 'nowrap',
    borderRadius: 8,
    overflow: 'hidden',
    boxShadow: theme.shadows[7],
    ...(open && {
      ...openedMixin(theme),
      '& .MuiDrawer-paper': openedMixin(theme),
    }),
    ...(!open && {
      ...closedMixin(theme),
      '& .MuiDrawer-paper': closedMixin(theme),
    }),
  };
});

const AppDrawer: React.FC = () => {
  const theme = useTheme();
  const t = useTranslation();
  const isSmallScreen = useMediaQuery(theme.breakpoints.down('lg'));
  const drawer = useDrawer();

  React.useEffect(() => {
    if (drawer.hasUserSet) return;
    if (isSmallScreen && drawer.isOpen) drawer.close();
    if (!isSmallScreen && !drawer.isOpen) drawer.open();
  }, [isSmallScreen]);

  return (
    <StyledDrawer
      data-testid="drawer"
      variant="permanent"
      aria-expanded={drawer.isOpen}
      open={drawer.isOpen}
    >
      <ToolbarIconContainer>
        <UnstyledIconButton
          titleKey={
            drawer.isOpen ? 'button.close-the-menu' : 'button.open-the-menu'
          }
          onClick={drawer.toggle}
          icon={<MSupplyGuy size={drawer.isOpen ? 'large' : 'medium'} />}
        />
      </ToolbarIconContainer>
      <UpperListContainer>
        <List>
          <AppNavLink
            to={AppRoute.Dashboard}
            icon={<Dashboard fontSize="small" />}
            text={t('app.dashboard')}
          />
          <React.Suspense fallback={null}>
            <CustomersNav />
          </React.Suspense>
          <AppNavLink
            to={AppRoute.Suppliers}
            icon={<Suppliers fontSize="small" />}
            text={t('app.suppliers')}
          />
          <AppNavLink
            to={AppRoute.Stock}
            icon={<Stock fontSize="small" />}
            text={t('app.stock')}
          />
          <AppNavLink
            to={AppRoute.Tools}
            icon={<Tools fontSize="small" />}
            text={t('app.tools')}
          />
          <AppNavLink
            to={AppRoute.Reports}
            icon={<Reports fontSize="small" />}
            text={t('app.reports')}
          />
          <AppNavLink
            to={AppRoute.Messages}
            icon={<Messages fontSize="small" />}
            text={t('app.messages')}
          />
        </List>
      </UpperListContainer>
      <LowerListContainer>
        <List>
          {drawer.isOpen && <StyledDivider />}
          <AppNavLink
            to={AppRoute.Sync}
            icon={<Radio />}
            text={t('app.sync')}
          />
          <AppNavLink
            to={AppRoute.Admin}
            icon={<Settings />}
            text={t('app.admin')}
          />
          <AppNavLink
            to={AppRoute.Logout}
            icon={<Power />}
            text={t('app.logout')}
          />
        </List>
      </LowerListContainer>
    </StyledDrawer>
  );
};

export default AppDrawer;
