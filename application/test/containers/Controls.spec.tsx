import React from 'react';
import Enzyme, { mount, ReactWrapper } from 'enzyme';
import { testStore } from '../../app/store';
import Adapter from 'enzyme-adapter-react-16';
import Root from '../../app/containers/Root';
import axios from 'axios';
import MockAdapter from 'axios-mock-adapter';
import { act } from 'react-dom/test-utils';
import { OuterSpaceWrapper } from '../helpers/wrappers';
import { flushPromises } from '../helpers/tools';

Enzyme.configure({ adapter: new Adapter() });

// @ts-ignore
process.resourcesPath = 'test';
jest.mock('electron', () => ({
  require: jest.fn(),
  match: jest.fn(),
  app: jest.fn(),
  remote: {
    app: {
      getPath: jest.fn(() => 'extraResources'),
      isPackaged: jest.fn(),
      getVersion: jest.fn(() => 'test'),
    },
  },
  dialog: jest.fn(),
}));

describe('Play', () => {
  test('click #play: ParseError', async () => {
    const mock = new MockAdapter(axios);
    const response = {
      ParseError: { message: 'Unexpected Token', line: 14, column: 15 },
    };
    mock.onPost().reply(200, response);
    // const component = mount(<OuterSpaceWrapper />);
    let component: ReactWrapper;
    await act(async () => {
      component = mount(<OuterSpaceWrapper />);
      await flushPromises();
    });
    await act(async () => {
      component.find('#playButton').at(0).simulate('click');
      await flushPromises();
      component.update();
      const errorDescription = component.find('#errorDescription');
      expect(errorDescription.exists()).toBe(true);
      expect(errorDescription.at(0).text()).toBe(
        'Unexpected Token: Line: 14 | Column 15'
      );
    });
  });
  test('click play: RenderSuccess', async () => {
    const mock = new MockAdapter(axios);
    const response = {
      RenderSuccess: 'success',
    };
    mock.onPost().reply(200, response);

    const component = mount(<OuterSpaceWrapper />);

    await act(async () => {
      component.find('#playButton').at(0).simulate('click');
      await flushPromises();
    });
    component.update();

    const errorDescription = component.find('#errorDescription');
    expect(errorDescription.exists()).toBe(false);
  });
  test('click play: IdError', async () => {
    const mock = new MockAdapter(axios);
    const response = {
      IdError: { id: 'thing' },
    };
    mock.onPost().reply(200, response);

    const component = mount(<OuterSpaceWrapper />);
    await act(async () => {
      component.find('#playButton').at(0).simulate('click');
      await flushPromises();
    });
    component.update();

    const errorDescription = component.find('#errorDescription');
    expect(errorDescription.exists()).toBe(true);
    expect(errorDescription.at(0).text()).toBe('Name Not Found: thing');
  });
  test('click #play: IndexError', async () => {
    const mock = new MockAdapter(axios);
    const response = {
      IndexError: {
        len_list: 7,
        index: 8,
        message: 'index 8 is greater than length of list 7',
      },
    };
    mock.onPost().reply(200, response);

    const component = mount(<OuterSpaceWrapper />);
    await act(async () => {
      component.find('#playButton').at(0).simulate('click');
      await flushPromises();
    });
    component.update();

    const errorDescription = component.find('#errorDescription');
    expect(errorDescription.exists()).toBe(true);
    expect(errorDescription.at(0).text()).toBe(
      'index 8 is greater than length of list 7'
    );
  });
  test('click #play: MsgError', async () => {
    const mock = new MockAdapter(axios);
    const response = {
      Msg: 'I am a message',
    };
    mock.onPost().reply(200, response);

    const component = mount(<OuterSpaceWrapper />);
    await act(async () => {
      component.find('#playButton').at(0).simulate('click');
      await flushPromises();
    });
    component.update();

    const errorDescription = component.find('#errorDescription');
    expect(errorDescription.exists()).toBe(true);
    expect(errorDescription.at(0).text()).toBe('Error: I am a message');
  });
});

describe('Controls', () => {
  it('play button exists', async () => {
    await act(async () => {
      const component = mount(<Root initialStore={testStore} />);
      await flushPromises();
      expect(component.find('#playButton').exists()).toBe(true);
    });
  });
  it('render button exists', async () => {
    await act(async () => {
      const component = mount(<Root initialStore={testStore} />);
      await flushPromises();
      expect(component.find('#printButton').exists()).toBe(true);
    });
  });
  it('stop button exists', async () => {
    await act(async () => {
      const component = mount(<Root initialStore={testStore} />);
      await flushPromises();
      expect(component.find('#stopButton').exists()).toBe(true);
    });
  });
  it('load button exists', async () => {
    await act(async () => {
      const component = mount(<Root initialStore={testStore} />);
      await flushPromises();
      expect(component.find('#loadButton').exists()).toBe(true);
    });
  });
  it('save button exists', async () => {
    await act(async () => {
      const component = mount(<Root initialStore={testStore} />);
      await flushPromises();
      expect(component.find('#saveButton').exists()).toBe(true);
    });
  });
  it('reset button exists', async () => {
    await act(async () => {
      const component = mount(<Root initialStore={testStore} />);
      await flushPromises();
      expect(component.find('#resetButton').exists()).toBe(true);
    });
  });
  it('editor button exists', async () => {
    await act(async () => {
      const component = mount(<Root initialStore={testStore} />);
      await flushPromises();
      expect(component.find('#settingsButton').exists()).toBe(true);
    });
  });
});
