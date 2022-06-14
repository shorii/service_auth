import type { NextPage } from 'next';
import { SignUpPage, Layout } from '@/modules';

const SignUp: NextPage = () => {
    return (
        <Layout title={'Sign up'}>
            <SignUpPage />
        </Layout>
    );
};

export default SignUp;
