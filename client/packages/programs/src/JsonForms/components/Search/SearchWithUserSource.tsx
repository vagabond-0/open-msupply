import React, { useEffect } from 'react';
import { ControlProps, UISchemaElement } from '@jsonforms/core';
import {
  useTranslation,
  Box,
  Typography,
  DetailInputWithLabelRow,
  FilterBy,
  Select,
  Button,
} from '@openmsupply-client/common';
import {
  FORM_INPUT_COLUMN_WIDTH,
  DefaultFormRowSx,
  FORM_LABEL_WIDTH,
} from '../../common/styleConstants';
import { useSearchQueries } from './useSearchQueries';
import { UserOptions } from './Search';
import { JsonFormsDispatch } from '@jsonforms/react';

export const SearchWithUserSource = (
  props: ControlProps & { options: UserOptions }
) => {
  const {
    data,
    path,
    handleChange,
    label,
    visible,
    options,
    schema,
    renderers,
  } = props;
  const t = useTranslation('programs');

  console.log('Data', data);

  const isPatientSelected = !!data?.id;

  const {
    runQuery,
    getOptionLabel,
    saveFields,
    error: queryError,
    results,
    resetResults,
  } = useSearchQueries(options ?? {});

  useEffect(() => {
    if (!data) {
      handleChange(path, {});
      return;
    }

    // Only run the database query if a specific patient hasn't yet been
    // selected
    if (isPatientSelected) return;

    const searchFilter: FilterBy = {};
    options.searchFields.forEach(field => {
      const match =
        field in queryMatchTypes
          ? queryMatchTypes?.[field as keyof typeof queryMatchTypes]
          : 'like';
      if (data[field]) searchFilter[field] = { [match]: data[field] };
    });
    if (Object.keys(searchFilter).length > 0) runQuery(searchFilter);
  }, [data]);

  const handlePatientSelect = (patientId: string) => {
    const patient = results.find(p => (p.id = patientId));
    if (!patient) return;
    if (!saveFields) {
      handleChange(path, patient);
      return;
    }
    const newData = Object.fromEntries(
      Object.entries(patient).filter(
        ([key]) => (saveFields as string[])?.includes(key)
      )
    );
    handleChange(path, newData);
  };

  const error = props.errors ?? queryError ?? null;

  if (!visible) return null;

  return (
    <Box>
      <Typography
        variant="subtitle1"
        width={'100%'}
        textAlign="left"
        marginBottom={1}
        paddingBottom={1}
        paddingTop={3}
      >
        <strong>{label}</strong>
      </Typography>
      <JsonFormsDispatch
        schema={schema}
        uischema={
          {
            type: 'VerticalLayout',
            elements: options.elements,
          } as UISchemaElement
        }
        path={path}
        renderers={renderers}
        enabled={!isPatientSelected}
      />
      {(isPatientSelected || results.length > 0) && (
        <DetailInputWithLabelRow
          sx={DefaultFormRowSx}
          label=""
          labelWidthPercentage={FORM_LABEL_WIDTH}
          inputAlignment={'start'}
          Input={
            !isPatientSelected ? (
              <Box>
                <Typography variant="body2" mt={1} mb={1}>
                  <em>{t('control.search.matching-patients')}</em>
                </Typography>
                <Select
                  options={results.map(res => ({
                    label: getOptionLabel(res) ?? '',
                    value: res.id,
                  }))}
                  onChange={e => handlePatientSelect(e.target.value)}
                  fullWidth
                />
              </Box>
            ) : (
              <Box
                display="flex"
                alignItems="center"
                justifyContent="space-between"
                flexBasis="100%"
                sx={{ width: FORM_INPUT_COLUMN_WIDTH }}
              >
                {!error ? (
                  <Button
                    onClick={() => {
                      handleChange(path, {});
                      resetResults();
                    }}
                    variant="outlined"
                    size="small"
                    sx={{ mt: 1 }}
                  >
                    {t('control.search.reset-button')}
                  </Button>
                ) : (
                  <Typography color="error">{error}</Typography>
                )}
              </Box>
            )
          }
        />
      )}
    </Box>
  );
};

// Most search fields will be matched using partial string matching, using
// "like". However, this is not available/reasonable for some fields, so their
// match type is referenced here
const queryMatchTypes = {
  // Add more as required
  gender: 'equalTo',
  dateOfBirth: 'equalTo',
};
