import { gql } from 'apollo-server';

export default gql`
  schema {
    query: Queries
    mutation: Mutations
  }

  # Directs the executor to query only when the field exists.
  directive @ifdef on FIELD

  type BatchIsReserved implements DeleteSupplierInvoiceLineErrorInterface & UpdateSupplierInvoiceLineErrorInterface {
    description: String!
  }

  type CanOnlyEditInvoicesInLoggedInStoreError implements UpdateCustomerInvoiceErrorInterface {
    description: String!
  }

  type CannotChangeInvoiceBackToDraft implements UpdateSupplierInvoiceErrorInterface {
    description: String!
  }

  type CannotChangeStatusBackToDraftError implements UpdateCustomerInvoiceErrorInterface {
    description: String!
  }

  type CannotDeleteInvoiceWithLines implements DeleteCustomerInvoiceErrorInterface & DeleteSupplierInvoiceErrorInterface {
    description: String!
    lines: InvoiceLineConnector!
  }

  type CannotEditFinalisedInvoice implements UpdateSupplierInvoiceLineErrorInterface & InsertCustomerInvoiceLineErrorInterface & UpdateCustomerInvoiceLineErrorInterface & DeleteCustomerInvoiceLineErrorInterface & DeleteSupplierInvoiceLineErrorInterface & UpdateSupplierInvoiceErrorInterface & DeleteCustomerInvoiceErrorInterface & DeleteSupplierInvoiceErrorInterface & InsertSupplierInvoiceLineErrorInterface {
    description: String!
  }

  # Generic Error Wrapper
  type ConnectorError {
    error: ConnectorErrorInterface!
  }

  interface ConnectorErrorInterface {
    description: String!
  }

  type DatabaseError implements UpdateCustomerInvoiceLineErrorInterface & InsertSupplierInvoiceLineErrorInterface & NodeErrorInterface & DeleteSupplierInvoiceLineErrorInterface & DeleteCustomerInvoiceErrorInterface & InsertCustomerInvoiceErrorInterface & UpdateCustomerInvoiceErrorInterface & InsertCustomerInvoiceLineErrorInterface & DeleteCustomerInvoiceLineErrorInterface & InsertSupplierInvoiceErrorInterface & UpdateSupplierInvoiceLineErrorInterface & ConnectorErrorInterface & UpdateSupplierInvoiceErrorInterface & DeleteSupplierInvoiceErrorInterface {
    description: String!
    fullError: String!
  }

  # Implement the DateTime<Utc> scalar
  #
  # The input/output is a string in RFC3339 format.
  scalar DateTime

  input DatetimeFilterInput {
    equalTo: DateTime
    beforeOrEqualTo: DateTime
    afterOrEqualTo: DateTime
  }

  # Generic Error Wrapper
  type DeleteCustomerInvoiceError {
    error: DeleteCustomerInvoiceErrorInterface!
  }

  interface DeleteCustomerInvoiceErrorInterface {
    description: String!
  }

  # Generic Error Wrapper
  type DeleteCustomerInvoiceLineError {
    error: DeleteCustomerInvoiceLineErrorInterface!
  }

  interface DeleteCustomerInvoiceLineErrorInterface {
    description: String!
  }

  input DeleteCustomerInvoiceLineInput {
    id: String!
    invoiceId: String!
  }

  union DeleteCustomerInvoiceLineResponse =
      DeleteCustomerInvoiceLineError
    | DeleteResponse

  union DeleteCustomerInvoiceResponse =
      DeleteCustomerInvoiceError
    | DeleteResponse

  type DeleteResponse {
    id: String!
  }

  # Generic Error Wrapper
  type DeleteSupplierInvoiceError {
    error: DeleteSupplierInvoiceErrorInterface!
  }

  interface DeleteSupplierInvoiceErrorInterface {
    description: String!
  }

  input DeleteSupplierInvoiceInput {
    id: String!
  }

  # Generic Error Wrapper
  type DeleteSupplierInvoiceLineError {
    error: DeleteSupplierInvoiceLineErrorInterface!
  }

  interface DeleteSupplierInvoiceLineErrorInterface {
    description: String!
  }

  input DeleteSupplierInvoiceLineInput {
    id: String!
    invoiceId: String!
  }

  union DeleteSupplierInvoiceLineResponse =
      DeleteSupplierInvoiceLineError
    | DeleteResponse

  union DeleteSupplierInvoiceResponse =
      DeleteSupplierInvoiceError
    | DeleteResponse

  input EqualFilterBoolInput {
    equalTo: Boolean
  }

  input EqualFilterInvoiceStatusInput {
    equalTo: InvoiceNodeStatus
  }

  input EqualFilterInvoiceTypeInput {
    equalTo: InvoiceNodeType
  }

  input EqualFilterStringInput {
    equalTo: String
  }

  type FinalisedInvoiceIsNotEditableError implements UpdateCustomerInvoiceErrorInterface {
    description: String!
  }

  enum ForeignKey {
    OTHER_PARTY_ID
    ITEM_ID
    INVOICE_ID
    STOCK_LINE_ID
  }

  type ForeignKeyError implements InsertCustomerInvoiceErrorInterface & UpdateCustomerInvoiceErrorInterface & InsertSupplierInvoiceErrorInterface & UpdateSupplierInvoiceErrorInterface & UpdateCustomerInvoiceLineErrorInterface & UpdateSupplierInvoiceLineErrorInterface & InsertCustomerInvoiceLineErrorInterface & DeleteCustomerInvoiceLineErrorInterface & DeleteSupplierInvoiceLineErrorInterface & InsertSupplierInvoiceLineErrorInterface {
    description: String!
    key: ForeignKey!
  }

  # Generic Error Wrapper
  type InsertCustomerInvoiceError {
    error: InsertCustomerInvoiceErrorInterface!
  }

  interface InsertCustomerInvoiceErrorInterface {
    description: String!
  }

  input InsertCustomerInvoiceInput {
    # The new invoice id provided by the client
    id: String!

    # The other party must be an customer of the current store
    otherPartyId: String!
    status: InvoiceNodeStatus
    comment: String
    theirReference: String
  }

  # Generic Error Wrapper
  type InsertCustomerInvoiceLineError {
    error: InsertCustomerInvoiceLineErrorInterface!
  }

  interface InsertCustomerInvoiceLineErrorInterface {
    description: String!
  }

  input InsertCustomerInvoiceLineInput {
    id: String!
    invoiceId: String!
    itemId: String!
    stockLineId: String!
    numberOfPacks: Int!
  }

  union InsertCustomerInvoiceLineResponse =
      InsertCustomerInvoiceLineError
    | NodeError
    | InvoiceLineNode

  union InsertCustomerInvoiceResponse =
      InsertCustomerInvoiceError
    | NodeError
    | InvoiceNode

  # Generic Error Wrapper
  type InsertSupplierInvoiceError {
    error: InsertSupplierInvoiceErrorInterface!
  }

  interface InsertSupplierInvoiceErrorInterface {
    description: String!
  }

  input InsertSupplierInvoiceInput {
    id: String!
    otherPartyId: String!
    status: InvoiceNodeStatus!
    comment: String
    theirReference: String
  }

  # Generic Error Wrapper
  type InsertSupplierInvoiceLineError {
    error: InsertSupplierInvoiceLineErrorInterface!
  }

  interface InsertSupplierInvoiceLineErrorInterface {
    description: String!
  }

  input InsertSupplierInvoiceLineInput {
    id: String!
    invoiceId: String!
    itemId: String!
    packSize: Int!
    batch: String
    costPricePerPack: Float!
    sellPricePerPack: Float!
    expiryDate: NaiveDate
    numberOfPacks: Int!
  }

  union InsertSupplierInvoiceLineResponse =
      InsertSupplierInvoiceLineError
    | NodeError
    | InvoiceLineNode

  union InsertSupplierInvoiceResponse =
      InsertSupplierInvoiceError
    | NodeError
    | InvoiceNode

  # Generic Connector
  type InvoiceConnector {
    totalCount: Int!
    nodes: [InvoiceNode!]!
  }

  type InvoiceDoesNotBelongToCurrentStore implements InsertCustomerInvoiceLineErrorInterface & DeleteCustomerInvoiceLineErrorInterface & UpdateCustomerInvoiceLineErrorInterface & UpdateSupplierInvoiceLineErrorInterface & DeleteSupplierInvoiceLineErrorInterface & DeleteSupplierInvoiceErrorInterface & UpdateSupplierInvoiceErrorInterface & DeleteCustomerInvoiceErrorInterface & InsertSupplierInvoiceLineErrorInterface {
    description: String!
  }

  input InvoiceFilterInput {
    nameId: EqualFilterStringInput
    storeId: EqualFilterStringInput
    type: EqualFilterInvoiceTypeInput
    status: EqualFilterInvoiceStatusInput
    comment: SimpleStringFilterInput
    theirReference: EqualFilterStringInput
    entryDatetime: DatetimeFilterInput
    confirmDatetime: DatetimeFilterInput
    finalisedDatetime: DatetimeFilterInput
  }

  type InvoiceLineBelongsToAnotherInvoice implements UpdateCustomerInvoiceLineErrorInterface & DeleteSupplierInvoiceLineErrorInterface & DeleteCustomerInvoiceLineErrorInterface & UpdateSupplierInvoiceLineErrorInterface {
    description: String!
    invoice: InvoiceResponse!
  }

  # Generic Connector
  type InvoiceLineConnector {
    totalCount: Int!
    nodes: [InvoiceLineNode!]!
  }

  type InvoiceLineHasNoStockLineError implements UpdateCustomerInvoiceErrorInterface {
    description: String!
    invoiceLineId: String!
  }

  type InvoiceLineNode {
    id: String!
    itemId: String!
    itemName: String!
    itemCode: String!
    itemUnit: String!
    packSize: Int!
    numberOfPacks: Int!
    costPricePerPack: Float!
    sellPricePerPack: Float!
    batch: String
    expiryDate: NaiveDate
    stockLine: StockLineResponse
    location: String
  }

  union InvoiceLineResponse = NodeError | InvoiceLineNode

  union InvoiceLinesResponse = ConnectorError | InvoiceLineConnector

  type InvoiceNode {
    id: String!
    otherPartyName: String!
    otherPartyId: String!
    type: InvoiceNodeType!
    status: InvoiceNodeStatus!
    invoiceNumber: Int!
    theirReference: String
    comment: String
    entryDatetime: DateTime!
    confirmedDatetime: DateTime
    finalisedDatetime: DateTime
    lines: InvoiceLinesResponse!
    pricing: InvoicePriceResponse!
  }

  enum InvoiceNodeStatus {
    # For customer invoices: In DRAFT mode only the available_number_of_packs in a stock line gets
    # updated when items are added to the invoice.
    DRAFT

    # For customer invoices: When an invoice is CONFIRMED available_number_of_packs and
    # total_number_of_packs get updated when items are added to the invoice.
    CONFIRMED

    # A FINALISED invoice can't be edited nor deleted.
    FINALISED
  }

  enum InvoiceNodeType {
    CUSTOMER_INVOICE
    SUPPLIER_INVOICE
  }

  union InvoicePriceResponse = NodeError | InvoicePricingNode

  type InvoicePricingNode {
    totalAfterTax: Float!
  }

  union InvoiceResponse = NodeError | InvoiceNode

  enum InvoiceSortFieldInput {
    TYPE
    STATUS
    ENTRY_DATETIME
    CONFIRM_DATETIME
    FINALISED_DATE_TIME
  }

  input InvoiceSortInput {
    key: InvoiceSortFieldInput!
    desc: Boolean
  }

  union InvoicesResponse = ConnectorError | InvoiceConnector

  # Generic Connector
  type ItemConnector {
    totalCount: Int!
    nodes: [ItemNode!]!
  }

  type ItemDoesNotMatchStockLine implements InsertCustomerInvoiceLineErrorInterface & UpdateCustomerInvoiceLineErrorInterface {
    description: String!
  }

  input ItemFilterInput {
    name: SimpleStringFilterInput
    code: SimpleStringFilterInput
    isVisible: EqualFilterBoolInput
  }

  type ItemNode {
    id: String!
    name: String!
    code: String!
    isVisible: Boolean!
    availableBatches: StockLinesResponse!
  }

  enum ItemSortFieldInput {
    NAME
    CODE
  }

  input ItemSortInput {
    key: ItemSortFieldInput!
    desc: Boolean
  }

  union ItemsResponse = ConnectorError | ItemConnector

  type LineDoesNotReferenceStockLine implements UpdateCustomerInvoiceLineErrorInterface {
    description: String!
  }

  type Mutations {
    insertCustomerInvoice(
      input: InsertCustomerInvoiceInput!
    ): InsertCustomerInvoiceResponse!
    updateCustomerInvoice(
      input: UpdateCustomerInvoiceInput!
    ): UpdateCustomerInvoiceResponse!
    deleteCustomerInvoice(id: String!): DeleteCustomerInvoiceResponse!
    insertCustomerInvoiceLine(
      input: InsertCustomerInvoiceLineInput!
    ): InsertCustomerInvoiceLineResponse!
    updateCustomerInvoiceLine(
      input: UpdateCustomerInvoiceLineInput!
    ): UpdateCustomerInvoiceLineResponse!
    deleteCustomerInvoiceLine(
      input: DeleteCustomerInvoiceLineInput!
    ): DeleteCustomerInvoiceLineResponse!
    insertSupplierInvoice(
      input: InsertSupplierInvoiceInput!
    ): InsertSupplierInvoiceResponse!
    updateSupplierInvoice(
      input: UpdateSupplierInvoiceInput!
    ): UpdateSupplierInvoiceResponse!
    deleteSupplierInvoice(
      input: DeleteSupplierInvoiceInput!
    ): DeleteSupplierInvoiceResponse!
    insertSupplierInvoiceLine(
      input: InsertSupplierInvoiceLineInput!
    ): InsertSupplierInvoiceLineResponse!
    updateSupplierInvoiceLine(
      input: UpdateSupplierInvoiceLineInput!
    ): UpdateSupplierInvoiceLineResponse!
    deleteSupplierInvoiceLine(
      input: DeleteSupplierInvoiceLineInput!
    ): DeleteSupplierInvoiceLineResponse!
  }

  scalar NaiveDate

  # Generic Connector
  type NameConnector {
    totalCount: Int!
    nodes: [NameNode!]!
  }

  input NameFilterInput {
    name: SimpleStringFilterInput
    code: SimpleStringFilterInput
    isCustomer: Boolean
    isSupplier: Boolean
  }

  type NameNode {
    id: String!
    name: String!
    code: String!
    isCustomer: Boolean!
    isSupplier: Boolean!
  }

  enum NameSortFieldInput {
    NAME
    CODE
  }

  input NameSortInput {
    key: NameSortFieldInput!
    desc: Boolean
  }

  union NamesResponse = ConnectorError | NameConnector

  # Generic Error Wrapper
  type NodeError {
    error: NodeErrorInterface!
  }

  interface NodeErrorInterface {
    description: String!
  }

  type NotACustomerInvoice implements InsertCustomerInvoiceLineErrorInterface & UpdateCustomerInvoiceLineErrorInterface & DeleteCustomerInvoiceLineErrorInterface & DeleteCustomerInvoiceErrorInterface {
    description: String!
  }

  type NotACustomerInvoiceError implements UpdateCustomerInvoiceErrorInterface {
    description: String!
  }

  type NotASupplierInvoice implements UpdateSupplierInvoiceLineErrorInterface & DeleteSupplierInvoiceErrorInterface & UpdateSupplierInvoiceErrorInterface & DeleteSupplierInvoiceLineErrorInterface & InsertSupplierInvoiceLineErrorInterface {
    description: String!
  }

  type NotEnoughStockForReduction implements InsertCustomerInvoiceLineErrorInterface & UpdateCustomerInvoiceLineErrorInterface {
    description: String!
    line: InvoiceLineResponse
    batch: StockLineResponse!
  }

  type OtherPartyCannotBeThisStoreError implements InsertCustomerInvoiceErrorInterface & UpdateCustomerInvoiceErrorInterface {
    description: String!
  }

  type OtherPartyNotACustomerError implements UpdateCustomerInvoiceErrorInterface & InsertCustomerInvoiceErrorInterface {
    description: String!
    otherParty: NameNode!
  }

  type OtherPartyNotASupplier implements InsertSupplierInvoiceErrorInterface & UpdateSupplierInvoiceErrorInterface {
    description: String!
    otherParty: NameNode!
  }

  type PaginationError implements ConnectorErrorInterface {
    description: String!
    rangeError: RangeError!
  }

  # Generic Pagination Input
  input PaginationInput {
    first: Int
    offset: Int
  }

  type Queries {
    # apiVersion: String!
    names(
      # pagination (first and offset)
      page: PaginationInput

      # filters option
      filter: NameFilterInput

      # sort options (only first sort input is evaluated for this endpoint)
      sort: [NameSortInput!]
    ): NamesResponse!
    items(
      # pagination (first and offset)
      page: PaginationInput

      # filters option
      filter: ItemFilterInput

      # sort options (only first sort input is evaluated for this endpoint)
      sort: [ItemSortInput!]
    ): ItemsResponse!
    invoice(
      # id of the invoice
      id: String!
    ): InvoiceResponse!
    invoices(
      # pagination (first and offset)
      page: PaginationInput

      # filters option
      filter: InvoiceFilterInput

      # sort options (only first sort input is evaluated for this endpoint)
      sort: [InvoiceSortInput!]
    ): InvoicesResponse!
  }

  type RangeError implements UpdateCustomerInvoiceLineErrorInterface & UpdateSupplierInvoiceLineErrorInterface & InsertCustomerInvoiceLineErrorInterface & InsertSupplierInvoiceLineErrorInterface {
    description: String!
    field: RangeField!
    max: Int
    min: Int
  }

  enum RangeField {
    FIRST
    NUMBER_OF_PACKS
    PACK_SIZE
  }

  type RecordAlreadyExist implements InsertSupplierInvoiceLineErrorInterface & InsertCustomerInvoiceErrorInterface & InsertCustomerInvoiceLineErrorInterface & InsertSupplierInvoiceErrorInterface {
    description: String!
  }

  type RecordNotFound implements DeleteSupplierInvoiceLineErrorInterface & UpdateCustomerInvoiceErrorInterface & DeleteCustomerInvoiceLineErrorInterface & UpdateSupplierInvoiceErrorInterface & DeleteSupplierInvoiceErrorInterface & UpdateCustomerInvoiceLineErrorInterface & UpdateSupplierInvoiceLineErrorInterface & NodeErrorInterface & DeleteCustomerInvoiceErrorInterface {
    description: String!
  }

  input SimpleStringFilterInput {
    equalTo: String
    like: String
  }

  type StockLineAlreadyExistsInInvoice implements InsertCustomerInvoiceLineErrorInterface & UpdateCustomerInvoiceLineErrorInterface {
    description: String!
    line: InvoiceLineResponse!
  }

  # Generic Connector
  type StockLineConnector {
    totalCount: Int!
    nodes: [StockLineNode!]!
  }

  type StockLineDoesNotBelongToCurrentStore implements UpdateCustomerInvoiceLineErrorInterface & InsertCustomerInvoiceLineErrorInterface {
    description: String!
  }

  type StockLineNode {
    id: String!
    itemId: String!
    storeId: String!
    batch: String
    packSize: Int!
    costPricePerPack: Float!
    sellPricePerPack: Float!
    availableNumberOfPacks: Int!
    totalNumberOfPacks: Int!
    expiryDate: NaiveDate
  }

  union StockLineResponse = NodeError | StockLineNode

  union StockLinesResponse = ConnectorError | StockLineConnector

  # Generic Error Wrapper
  type UpdateCustomerInvoiceError {
    error: UpdateCustomerInvoiceErrorInterface!
  }

  interface UpdateCustomerInvoiceErrorInterface {
    description: String!
  }

  input UpdateCustomerInvoiceInput {
    # The new invoice id provided by the client
    id: String!

    # The other party must be a customer of the current store.
    # This field can be used to change the other_party of an invoice
    otherPartyId: String

    # When changing the status from DRAFT to CONFIRMED or FINALISED the total_number_of_packs for
    # existing invoice items gets updated.
    status: InvoiceNodeStatus
    comment: String

    # External invoice reference, e.g. purchase or shipment number
    theirReference: String
  }

  # Generic Error Wrapper
  type UpdateCustomerInvoiceLineError {
    error: UpdateCustomerInvoiceLineErrorInterface!
  }

  interface UpdateCustomerInvoiceLineErrorInterface {
    description: String!
  }

  input UpdateCustomerInvoiceLineInput {
    id: String!
    invoiceId: String!
    itemId: String
    stockLineId: String
    numberOfPacks: Int
  }

  union UpdateCustomerInvoiceLineResponse =
      UpdateCustomerInvoiceLineError
    | NodeError
    | InvoiceLineNode

  union UpdateCustomerInvoiceResponse =
      UpdateCustomerInvoiceError
    | NodeError
    | InvoiceNode

  # Generic Error Wrapper
  type UpdateSupplierInvoiceError {
    error: UpdateSupplierInvoiceErrorInterface!
  }

  interface UpdateSupplierInvoiceErrorInterface {
    description: String!
  }

  input UpdateSupplierInvoiceInput {
    id: String!
    otherPartyId: String
    status: InvoiceNodeStatus
    comment: String
    theirReference: String
  }

  # Generic Error Wrapper
  type UpdateSupplierInvoiceLineError {
    error: UpdateSupplierInvoiceLineErrorInterface!
  }

  interface UpdateSupplierInvoiceLineErrorInterface {
    description: String!
  }

  input UpdateSupplierInvoiceLineInput {
    id: String!
    invoiceId: String!
    itemId: String
    packSize: Int
    batch: String
    costPricePerPack: Float
    sellPricePerPack: Float
    expiryDate: NaiveDate
    numberOfPacks: Int
  }

  union UpdateSupplierInvoiceLineResponse =
      UpdateSupplierInvoiceLineError
    | NodeError
    | InvoiceLineNode

  union UpdateSupplierInvoiceResponse =
      UpdateSupplierInvoiceError
    | NodeError
    | InvoiceNode
`;
