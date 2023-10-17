import * as Types from '@openmsupply-client/common';

import { GraphQLClient } from 'graphql-request';
import { GraphQLClientRequestHeaders } from 'graphql-request/build/cjs/types';
import gql from 'graphql-tag';
import { SensorFragmentDoc } from '../../../Sensor/api/operations.generated';
import { LocationRowFragmentDoc } from '../../../../../system/src/Location/api/operations.generated';
import { graphql, ResponseResolver, GraphQLRequest, GraphQLContext } from 'msw'
export type TemperatureBreachFragment = { __typename: 'TemperatureBreachNode', id: string, acknowledged: boolean, duration: number, endDatetime?: string | null, startDatetime: string, type: Types.TemperatureBreachNodeType, maxOrMinTemperature?: number | null, sensor?: { __typename: 'SensorNode', id: string, isActive: boolean, name: string, serial: string, batteryLevel?: number | null, breach?: Types.TemperatureBreachNodeType | null, type: Types.SensorNodeType, location?: { __typename: 'LocationNode', id: string, name: string, onHold: boolean, code: string } | null, latestTemperatureLog?: { __typename: 'TemperatureLogConnector', totalCount: number, nodes: Array<{ __typename: 'TemperatureLogNode', temperature: number, datetime: string }> } | null } | null, location?: { __typename: 'LocationNode', id: string, name: string, onHold: boolean, code: string } | null };

export type Temperature_BreachesQueryVariables = Types.Exact<{
  page?: Types.InputMaybe<Types.PaginationInput>;
  sort?: Types.InputMaybe<Array<Types.TemperatureBreachSortInput> | Types.TemperatureBreachSortInput>;
  filter?: Types.InputMaybe<Types.TemperatureBreachFilterInput>;
  storeId: Types.Scalars['String']['input'];
}>;


export type Temperature_BreachesQuery = { __typename: 'Queries', temperatureBreaches: { __typename: 'TemperatureBreachConnector', totalCount: number, nodes: Array<{ __typename: 'TemperatureBreachNode', id: string, acknowledged: boolean, duration: number, endDatetime?: string | null, startDatetime: string, type: Types.TemperatureBreachNodeType, maxOrMinTemperature?: number | null, sensor?: { __typename: 'SensorNode', id: string, isActive: boolean, name: string, serial: string, batteryLevel?: number | null, breach?: Types.TemperatureBreachNodeType | null, type: Types.SensorNodeType, location?: { __typename: 'LocationNode', id: string, name: string, onHold: boolean, code: string } | null, latestTemperatureLog?: { __typename: 'TemperatureLogConnector', totalCount: number, nodes: Array<{ __typename: 'TemperatureLogNode', temperature: number, datetime: string }> } | null } | null, location?: { __typename: 'LocationNode', id: string, name: string, onHold: boolean, code: string } | null }> } };

export const TemperatureBreachFragmentDoc = gql`
    fragment TemperatureBreach on TemperatureBreachNode {
  __typename
  id
  acknowledged
  duration
  endDatetime
  startDatetime
  type
  maxOrMinTemperature
  sensor {
    ...Sensor
  }
  location {
    ...LocationRow
  }
}
    ${SensorFragmentDoc}
${LocationRowFragmentDoc}`;
export const Temperature_BreachesDocument = gql`
    query temperature_breaches($page: PaginationInput, $sort: [TemperatureBreachSortInput!], $filter: TemperatureBreachFilterInput, $storeId: String!) {
  temperatureBreaches(
    page: $page
    sort: $sort
    filter: $filter
    storeId: $storeId
  ) {
    ... on TemperatureBreachConnector {
      totalCount
      nodes {
        ...TemperatureBreach
      }
    }
  }
}
    ${TemperatureBreachFragmentDoc}`;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    temperature_breaches(variables: Temperature_BreachesQueryVariables, requestHeaders?: GraphQLClientRequestHeaders): Promise<Temperature_BreachesQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<Temperature_BreachesQuery>(Temperature_BreachesDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'temperature_breaches', 'query');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockTemperatureBreachesQuery((req, res, ctx) => {
 *   const { page, sort, filter, storeId } = req.variables;
 *   return res(
 *     ctx.data({ temperatureBreaches })
 *   )
 * })
 */
export const mockTemperatureBreachesQuery = (resolver: ResponseResolver<GraphQLRequest<Temperature_BreachesQueryVariables>, GraphQLContext<Temperature_BreachesQuery>, any>) =>
  graphql.query<Temperature_BreachesQuery, Temperature_BreachesQueryVariables>(
    'temperature_breaches',
    resolver
  )