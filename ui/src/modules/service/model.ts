export interface SignInRequest {
    username: string;
    password: string;
}

export const createSignInRequest = (username: string, password: string): SignInRequest => {
    return {
        username,
        password,
    };
};

export interface SignUpRequest {
    username: string;
    password: string;
}

export const createSignUpRequest = (username: string, password: string): SignUpRequest => {
    return {
        username,
        password,
    };
};
