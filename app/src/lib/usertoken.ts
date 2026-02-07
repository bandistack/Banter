import { writable } from "svelte/store";

export type UserToken = {
  access_token: string;
  refresh_token?: string | null;
  expires_in: number;
  scope: string[];
  token_type: string;
};

export const userToken = writable<UserToken | null>(null);
