export enum AppRoute {
  Android = 'android',

  Initialise = 'initialise',
  Login = 'login',
  Distribution = 'distribution',
  OutboundShipment = 'outbound-shipment',
  CustomerRequisition = 'customer-requisition',
  Customer = 'customers',
  Dispensary = 'dispensary',
  Patients = 'patients',
  Encounter = 'encounter',
  ContactTrace = 'contact-trace',
  VaccineCard = 'vaccine-card',
  Prescription = 'prescription',
  CustomerReturn = 'customer-return',

  Coldchain = 'cold-chain',
  Sensors = 'sensors',
  Monitoring = 'monitoring',
  Equipment = 'equipment',

  Discovery = 'discovery',

  Dashboard = 'dashboard',

  Replenishment = 'replenishment',
  InboundShipment = 'inbound-shipment',
  InternalOrder = 'internal-order',
  Suppliers = 'suppliers',
  SupplierReturn = 'supplier-return',

  Inventory = 'inventory',
  Stock = 'stock',
  Stocktakes = 'stocktakes',
  Locations = 'locations',
  MasterLists = 'master-lists',
  IndicatorsDemographics = 'indicators-demographics',

  Manage = 'manage',
  Programs = 'programs',
  Facilities = 'facilities',

  Tools = 'tools',

  Reports = 'reports',

  Messages = 'messages',

  Sync = 'sync',

  Settings = 'settings',

  Logout = 'logout',

  Catalogue = 'catalogue',
  Items = 'items',
  Assets = 'assets',
  LogReasons = 'log-reasons',
  ImmunisationPrograms = 'immunisations',

  RnRForms = 'r-and-r-forms',

  PageNotFound = 'page-not-found',
}

export enum ExternalURL {
  // PublicDocs = 'https://docs.msupply.foundation/docs',
  PublicDocs,
}

export interface LocalisedExternalUrlProps {
  url: ExternalURL;
  locale: String | undefined;
}

export const localisedExternalUrl = ({
  url,
  locale,
}: LocalisedExternalUrlProps) => {
  switch (url) {
    case ExternalURL.PublicDocs: {
      const localeUrlInsert = locale == 'en' ? '' : `${locale}/`;
      return `https://docs.msupply.foundation/${localeUrlInsert}docs`;
    }
  }
};
