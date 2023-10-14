import * as Types from '@openmsupply-client/common';

import { GraphQLClient } from 'graphql-request';
import { GraphQLClientRequestHeaders } from 'graphql-request/build/cjs/types';
import gql from 'graphql-tag';
import { graphql, ResponseResolver, GraphQLRequest, GraphQLContext } from 'msw'
export type DisplaySettingsQueryVariables = Types.Exact<{
  input: Types.DisplaySettingsHash;
}>;


export type DisplaySettingsQuery = { __typename: 'Queries', displaySettings: { __typename: 'DisplaySettingsNode', customTheme?: { __typename: 'DisplaySettingNode', value: string, hash: string } | null, customLogo?: { __typename: 'DisplaySettingNode', value: string, hash: string } | null } };

export type UpdateDisplaySettingsMutationVariables = Types.Exact<{
  displaySettings: Types.DisplaySettingsInput;
}>;


export type UpdateDisplaySettingsMutation = { __typename: 'Mutations', updateDisplaySettings: { __typename: 'UpdateDisplaySettingsError', error: string } | { __typename: 'UpdateResult', theme?: string | null, logo?: string | null } };

export type GetEmdSettingsQueryVariables = Types.Exact<{ [key: string]: never; }>;


export type GetEmdSettingsQuery = { __typename: 'Queries', emdSettings: { __typename: 'EmdSettingsNode', ip: string, intervalSeconds: number } };

export type UpdateEmdSettingsMutationVariables = Types.Exact<{
  input?: Types.InputMaybe<Types.EmdSettingsInput>;
  storeId: Types.Scalars['String']['input'];
}>;


export type UpdateEmdSettingsMutation = { __typename: 'Mutations', updateEmdSettings: { __typename: 'EmdSettingsNode', ip: string, intervalSeconds: number } };


export const DisplaySettingsDocument = gql`
    query displaySettings($input: DisplaySettingsHash!) {
  displaySettings(input: $input) {
    customTheme {
      value
      hash
    }
    customLogo {
      value
      hash
    }
  }
}
    `;
export const UpdateDisplaySettingsDocument = gql`
    mutation updateDisplaySettings($displaySettings: DisplaySettingsInput!) {
  updateDisplaySettings(input: $displaySettings) {
    __typename
    ... on UpdateResult {
      __typename
      theme
      logo
    }
    ... on UpdateDisplaySettingsError {
      __typename
      error
    }
  }
}
    `;
export const GetEmdSettingsDocument = gql`
    query getEmdSettings {
  emdSettings {
    ip
    intervalSeconds
  }
}
    `;
export const UpdateEmdSettingsDocument = gql`
    mutation updateEmdSettings($input: EmdSettingsInput, $storeId: String!) {
  updateEmdSettings(input: $input, storeId: $storeId) {
    ip
    intervalSeconds
  }
}
    `;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    displaySettings(variables: DisplaySettingsQueryVariables, requestHeaders?: GraphQLClientRequestHeaders): Promise<DisplaySettingsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<DisplaySettingsQuery>(DisplaySettingsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'displaySettings', 'query');
    },
    updateDisplaySettings(variables: UpdateDisplaySettingsMutationVariables, requestHeaders?: GraphQLClientRequestHeaders): Promise<UpdateDisplaySettingsMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<UpdateDisplaySettingsMutation>(UpdateDisplaySettingsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'updateDisplaySettings', 'mutation');
    },
    getEmdSettings(variables?: GetEmdSettingsQueryVariables, requestHeaders?: GraphQLClientRequestHeaders): Promise<GetEmdSettingsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<GetEmdSettingsQuery>(GetEmdSettingsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'getEmdSettings', 'query');
    },
    updateEmdSettings(variables: UpdateEmdSettingsMutationVariables, requestHeaders?: GraphQLClientRequestHeaders): Promise<UpdateEmdSettingsMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<UpdateEmdSettingsMutation>(UpdateEmdSettingsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'updateEmdSettings', 'mutation');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockDisplaySettingsQuery((req, res, ctx) => {
 *   const { input } = req.variables;
 *   return res(
 *     ctx.data({ displaySettings })
 *   )
 * })
 */
export const mockDisplaySettingsQuery = (resolver: ResponseResolver<GraphQLRequest<DisplaySettingsQueryVariables>, GraphQLContext<DisplaySettingsQuery>, any>) =>
  graphql.query<DisplaySettingsQuery, DisplaySettingsQueryVariables>(
    'displaySettings',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockUpdateDisplaySettingsMutation((req, res, ctx) => {
 *   const { displaySettings } = req.variables;
 *   return res(
 *     ctx.data({ updateDisplaySettings })
 *   )
 * })
 */
export const mockUpdateDisplaySettingsMutation = (resolver: ResponseResolver<GraphQLRequest<UpdateDisplaySettingsMutationVariables>, GraphQLContext<UpdateDisplaySettingsMutation>, any>) =>
  graphql.mutation<UpdateDisplaySettingsMutation, UpdateDisplaySettingsMutationVariables>(
    'updateDisplaySettings',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockGetEmdSettingsQuery((req, res, ctx) => {
 *   return res(
 *     ctx.data({ emdSettings })
 *   )
 * })
 */
export const mockGetEmdSettingsQuery = (resolver: ResponseResolver<GraphQLRequest<GetEmdSettingsQueryVariables>, GraphQLContext<GetEmdSettingsQuery>, any>) =>
  graphql.query<GetEmdSettingsQuery, GetEmdSettingsQueryVariables>(
    'getEmdSettings',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockUpdateEmdSettingsMutation((req, res, ctx) => {
 *   const { input, storeId } = req.variables;
 *   return res(
 *     ctx.data({ updateEmdSettings })
 *   )
 * })
 */
export const mockUpdateEmdSettingsMutation = (resolver: ResponseResolver<GraphQLRequest<UpdateEmdSettingsMutationVariables>, GraphQLContext<UpdateEmdSettingsMutation>, any>) =>
  graphql.mutation<UpdateEmdSettingsMutation, UpdateEmdSettingsMutationVariables>(
    'updateEmdSettings',
    resolver
  )
