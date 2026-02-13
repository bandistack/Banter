import { writable } from "svelte/store";

export const badgesEnabled = writable<boolean>(true);

// caché local de badges: "moderator" → { url_1x, url_2x }
export interface BadgeUrls {
  url_1x: string;
  url_2x: string;
}
export const badgeCache = writable<Record<string, BadgeUrls>>({});