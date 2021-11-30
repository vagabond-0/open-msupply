import React from 'react';
import { Route } from 'react-router';

import { ComponentStory, ComponentMeta } from '@storybook/react';
import { StoryProvider, TestingRouter } from '@openmsupply-client/common';

const handlers: any[] = [];

import { DetailView } from './DetailView';

export default {
  title: 'Page/OutboundShipmentDetailView',
  component: DetailView,
  argTypes: {
    backgroundColor: { control: 'color' },
  },
} as ComponentMeta<typeof DetailView>;

const Template: ComponentStory<typeof DetailView> = args => (
  <StoryProvider>
    <TestingRouter initialEntries={['/distribution/outbound-shipment/3']}>
      <Route path="/distribution/outbound-shipment">
        <Route path=":id" element={<DetailView {...args} />} />
      </Route>
    </TestingRouter>
  </StoryProvider>
);

export const Primary = Template.bind({});
Primary.parameters = {
  msw: handlers,
};
