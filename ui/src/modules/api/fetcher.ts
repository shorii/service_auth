import { LoginRequest, LoginResponse, RegisterRequest, RegisterResponse } from './model';

class APIClient {
    constructor(private url: string, private prefix: string) {}

    async _fetch(path: string, method: string, payload?: any) {
        return await fetch(`http://${this.url}${this.prefix}${path}`, {
            method: method,
            headers: {
                'Content-Type': 'application/json',
            },
            redirect: 'follow',
            mode: 'same-origin',
            body: payload ? JSON.stringify(payload) : undefined,
        });
    }

    async login(payload: LoginRequest): Promise<LoginResponse> {
        const resp = await this._fetch('/user/login', 'POST', payload);
        return resp.json();
    }

    async register(payload: RegisterRequest): Promise<RegisterResponse> {
        const resp = await this._fetch('/user/register', 'POST', payload);
        return resp.json();
    }
}

export const apiClient = new APIClient(process.env.API_SERVER!, process.env.API_PREFIX!);
