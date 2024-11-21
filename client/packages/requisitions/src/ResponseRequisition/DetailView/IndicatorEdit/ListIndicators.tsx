import React from 'react';
import {
  Tooltip,
  ListOptions,
  RouteBuilder,
  useNavigate,
} from '@openmsupply-client/common';
import { IndicatorLineRowFragment } from '../../api';

interface ListIndicatorLineProps {
  currentIndicatorLineId?: string | null;
  lines: IndicatorLineRowFragment[];
  route: RouteBuilder;
}

export const ListIndicatorLines = ({
  currentIndicatorLineId,
  lines,
  route,
}: ListIndicatorLineProps) => {
  const navigate = useNavigate();
  const value = lines?.find(({ id }) => id === currentIndicatorLineId) ?? null;
  const sortedLines = lines.sort((a, b) => a.lineNumber - b.lineNumber);

  return (
    <Tooltip title={value?.code}>
      <ListOptions
        currentId={value?.id}
        onClick={id => {
          navigate(route.addPart('indicator').addPart(id).build(), {
            replace: true,
          });
        }}
        options={
          sortedLines?.map(({ id, code }) => ({
            id,
            value: code,
          })) ?? []
        }
      />
    </Tooltip>
  );
};
