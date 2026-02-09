import { invoke } from "@tauri-apps/api/core";
import { UserTokenPersistence } from "$lib/tokenpersistence";
import type { UserToken } from "$lib/usertoken";

export async function ensureTokenLife(token: UserToken | null): Promise<void> {
  if(!token) return;
  if (token.expires_in < Date.now() / 1000 + 60) {
    const newToken: UserToken = await invoke("refresh_token");
    UserTokenPersistence.save(newToken);
  } 
}
