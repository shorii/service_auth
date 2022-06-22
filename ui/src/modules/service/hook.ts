import * as React from 'react';
import { apiClient } from '../api';
import { useRouter } from 'next/router';
import { SignInRequest, SignUpRequest } from './model';

const useAutoTransition = () => {
    const [location, setLocation] = React.useState<string | undefined>(undefined);
    const router = useRouter();
    React.useEffect(() => {
        if (!location) {
            return;
        }
        router.replace(location);
    }, [location, router]);

    return {
        setLocation,
    };
};

export const useSignIn = () => {
    const { setLocation } = useAutoTransition();
    const signIn = React.useCallback(async (request: SignInRequest) => {
        const resp = await apiClient.login({ ...request });
        setLocation(resp.location);
    }, []);

    return { signIn };
};

export const useSignUp = () => {
    const { setLocation } = useAutoTransition();
    const signUp = React.useCallback(async (request: SignUpRequest) => {
        const resp = await apiClient.register({ ...request });
        setLocation(resp.location);
    }, []);

    return { signUp };
};
