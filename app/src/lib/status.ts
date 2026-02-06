import { writable } from "svelte/store";

export type StatusMessage = {
  type: "success" | "error";
  text: string;
};

export const statusMessage = writable<StatusMessage | null>(null);
