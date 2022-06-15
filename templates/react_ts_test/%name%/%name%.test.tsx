import { render } from '@testing-library/react';
import React from 'react';
import %name%, { %name%Props } from './%name%';

describe('%name%', () => {
    const defaultProps: %name%Props = {};

    it('should render', () => {
        const props = { ...defaultProps };
        const { asFragment, queryByText } = render(<%name% {...props} />);

        expect(asFragment()).toMatchSnapshot();
        expect(queryByText('%name%')).toBeTruthy();
    });
});
