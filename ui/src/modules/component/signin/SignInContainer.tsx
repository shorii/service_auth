import * as React from 'react';
import { useSignIn } from '@/modules/service';
import { AuthForm, AuthFormValues } from '@/modules/component/common';
import Typography from '@mui/material/Typography';
import OpenInNew from '@mui/icons-material/OpenInNew';
import Button from '@mui/material/Button';
import Link from 'next/link';

export interface SignInContainerProps {}

export const SignInContainer: React.FC<SignInContainerProps> = (props) => {
    const { signIn } = useSignIn();

    const handleClick = React.useCallback(
        async (values: AuthFormValues) => {
            await signIn({ ...values });
        },
        [signIn],
    );

    const footer = (
        <>
            <Typography variant="caption">Do you have an account?</Typography>
            <Link href="/signup">
                <Button
                    size="small"
                    variant="text"
                    startIcon={<OpenInNew />}
                    style={{ float: 'right' }}
                >
                    Sign up
                </Button>
            </Link>
        </>
    );
    return <AuthForm label="Sign in" footer={footer} onClick={handleClick} />;
};
