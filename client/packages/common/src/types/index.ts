import { ObjectWithStringKeys } from './utility';

export * from './utility';

type RecordWithId = { id: string };

export interface DomainObject extends RecordWithId, ObjectWithStringKeys {}

export interface Item extends DomainObject {
  id: string;
  code: string;
  name: string;
  availableQuantity: number;
}

export interface StockLine extends DomainObject {
  id: string;
  expiry: string;
  name: string;
  availableNumberOfPacks: number;
  packSize: number;
  item: Item;
}

export interface InvoiceLine extends DomainObject {
  id: string;
  itemName: string;
  stockLineId: string;
  invoiceId: string;
  itemCode?: string;
  stockLine?: StockLine;
  item?: Item;
  quantity: number;
  batchName?: string;
  expiry: string;
}

export interface Transaction extends DomainObject {
  id: string;
  color: string;
  comment: string;
  status: string;
  type: string;
  entered: string;
  confirmed: string;
  invoiceNumber: string;
  total: string;
  name: string;
  lines: InvoiceLine[];
}

export type Test = {
  id: number;
  message: string;
};

export type User = {
  id: string;
  name: string;
};

export type Store = {
  id: string;
  name: string;
};

export interface Invoice extends DomainObject {
  id: string;
  color: string;
  comment: string;
  status: string;
  type: string;
  entered: string;
  confirmed: string;
  invoiceNumber: string;
  total: string;
  name: string;
  lines: InvoiceLine[];
}
