import type { NextPage } from 'next';
import { SignInPage, Layout } from '@/modules';

const SignIn: NextPage = () => {
    return (
        <Layout title={'Sign in'}>
            <SignInPage />
        </Layout>
    );
};

export default SignIn;
