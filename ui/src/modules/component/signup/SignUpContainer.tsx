import * as React from 'react';
import { AuthForm } from '@/modules/component/common';
import ArrowLeft from '@mui/icons-material/ArrowLeft';
import Button from '@mui/material/Button';
import Link from 'next/link';

export interface SignUpContainerProps {}

export const SignUpContainer: React.FC<SignUpContainerProps> = (props) => {
    const footer = (
        <Link href="/">
            <Button
                size="small"
                variant="text"
                startIcon={<ArrowLeft />}
                style={{ float: 'right' }}
            >
                Sign in
            </Button>
        </Link>
    );
    return <AuthForm label="Sign up" footer={footer} />;
};
