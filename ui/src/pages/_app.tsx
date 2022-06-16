import { FunctionComponent } from 'react';
import '../../style/global.style.css';
import type { AppProps } from 'next/app';
import { ThemeProvider, createTheme } from '@mui/material';

const App: FunctionComponent<AppProps> = ({ Component, pageProps }) => {
    const darkTheme = createTheme({
        palette: {
            mode: 'dark',
        },
    });
    return (
        <ThemeProvider theme={darkTheme}>
            <Component {...pageProps} />
        </ThemeProvider>
    );
};

export default App;
