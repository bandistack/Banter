import type { UserToken } from "$lib/usertoken";
import { userToken } from "$lib/usertoken";

export class UserTokenPersistence {
  private static STORAGE_KEY = "userToken";
    private static persist(token: UserToken | null): void {
        if (token) {
            localStorage.setItem(this.STORAGE_KEY, JSON.stringify(token));
        } else {
            localStorage.removeItem(this.STORAGE_KEY);
        }
    }
    private static update(token: UserToken | null): void {
        userToken.set(token);
    }
    static save(token: UserToken | null): void {
        this.persist(token);
        this.update(token);
    }
    static load(): UserToken | null { 
        const stored = localStorage.getItem(this.STORAGE_KEY); 
        if (stored) { 
            const parsed = JSON.parse(stored) as UserToken; 
            this.update(parsed); return parsed; 
        } 
        return null; 
    }
    static clear(): void {
        this.persist(null);
        this.update(null);
    }
}
