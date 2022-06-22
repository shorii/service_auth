import * as React from 'react';
import { useSignUp } from '@/modules/service';
import { AuthForm, AuthFormValues } from '@/modules/component/common';
import ArrowLeft from '@mui/icons-material/ArrowLeft';
import Button from '@mui/material/Button';
import Link from 'next/link';

export interface SignUpContainerProps {}

export const SignUpContainer: React.FC<SignUpContainerProps> = (props) => {
    const { signUp } = useSignUp();

    const handleClick = React.useCallback(
        async (values: AuthFormValues) => {
            await signUp({ ...values });
        },
        [signUp],
    );

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
    return <AuthForm label="Sign up" footer={footer} onClick={handleClick} />;
};
