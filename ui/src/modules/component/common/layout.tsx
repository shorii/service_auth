import * as React from 'react';
import Head from 'next/head';

export interface LayoutProps {
    title: string;
}

export const Layout = (props: React.PropsWithChildren<LayoutProps>) => {
    const { title, children } = props;

    return (
        <>
            <Head>
                <title>{title}</title>
            </Head>
            {children}
        </>
    );
};
