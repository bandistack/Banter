import { invoke } from "@tauri-apps/api/core";
import { UserTokenPersistence } from "$lib/tokenpersistence";
import type { UserToken } from "$lib/usertoken";

export async function ensureTokenLife(): Promise<UserToken | null> {
  const token = UserTokenPersistence.load();
  const now = Date.now() / 1000;
  if (!token) {
    return null;
  }
  if (!(token.expires_in && token.expires_in < now + 60)) {
    return token;
  }
  try {
    const newToken: UserToken = await invoke("refresh_token");
    if (newToken?.access_token) {
      UserTokenPersistence.save(newToken);
      return newToken;
    }
  } catch (err) {
    console.error("Error al refrescar token:", err);
    return null;
  }
  return null;
}

