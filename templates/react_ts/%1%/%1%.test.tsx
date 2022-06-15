import { render } from '@testing-library/react';
import React from 'react';
import %1%, { %1%Props } from './%1%';

describe('%1%', () => {
    const defaultProps: %1%Props = {};

    it('should render', () => {
        const props = { ...defaultProps };
        const { asFragment, queryByText } = render(<%1% {...props} />);

        expect(asFragment()).toMatchSnapshot();
        expect(queryByText('%1%')).toBeTruthy();
    });
});
