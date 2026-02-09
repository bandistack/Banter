import type { UserToken } from "$lib/usertoken";
import { userToken } from "$lib/usertoken";

export class UserTokenPersistence {
    private static STORAGE_KEY = "userToken";

    static save(token: UserToken | null): void {
        if (token) {
            localStorage.setItem(this.STORAGE_KEY, JSON.stringify(token));
            userToken.set(token);
        } 
        else {
            localStorage.removeItem(this.STORAGE_KEY);
            userToken.set(null);
        }
    }
    static update(token: UserToken | null): void {
        userToken.set(token);
    }
    static load(): UserToken | null {
        const stored = localStorage.getItem(this.STORAGE_KEY);
        if (stored) {
            const parsed = JSON.parse(stored) as UserToken;
            userToken.set(parsed);
            return parsed;
        }
        userToken.set(null);
        return null;
    }
    static clear(): void {
        localStorage.removeItem(this.STORAGE_KEY);
        userToken.set(null);
    }
}
