import React, { FC } from 'react';
import {
  Box,
  FlatButton,
  PaperPopoverSection,
  usePaperClickPopover,
  useTranslation,
  useNavigate,
} from '@openmsupply-client/common';
import { useIntlUtils, SupportedLocales, useUserName } from '@common/intl';

import { LanguageType, PropsWithChildrenOnly } from '@common/types';

export const LanguageSelector: FC<PropsWithChildrenOnly> = ({ children }) => {
  const navigate = useNavigate();
  const { hide, PaperClickPopover } = usePaperClickPopover();
  const t = useTranslation('app');
  const username = useUserName();

  const { i18n, languageOptions, changeLanguage, setUserLocale } =
    useIntlUtils();

  const languageButtons = languageOptions.map(l => (
    <FlatButton
      label={l.label}
      disabled={l.value === i18n.language}
      onClick={() => {
        changeLanguage(l.value as LanguageType);
        setUserLocale(username, l.value as SupportedLocales);
        hide();
        navigate(0);
      }}
      key={l.value}
      sx={{
        whiteSpace: 'nowrap',
        overflowX: 'hidden',
        overflowY: 'visible',
        textOverflow: 'ellipsis',
        display: 'block',
        textAlign: 'left',
      }}
    />
  ));
  return (
    <PaperClickPopover
      placement="top"
      width={300}
      Content={
        <PaperPopoverSection label={t('select-language')}>
          <Box
            style={{
              overflowY: 'auto',
              maxHeight: 300,
            }}
          >
            {languageButtons}
          </Box>
        </PaperPopoverSection>
      }
    >
      {children}
    </PaperClickPopover>
  );
};
