export interface LoginRequest {
    username: string;
    password: string;
}

export interface RegisterRequest {
    username: string;
    password: string;
}

export interface LoginResponse {
    location: string;
}

export interface RegisterResponse {
    location: string;
}
