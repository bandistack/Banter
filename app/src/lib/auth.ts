export function getAuthUrl(): string {
  const clientId = import.meta.env.VITE_TWITCH_CLIENT_ID;
  const redirectUri = import.meta.env.VITE_TWITCH_REDIRECT_URI;
  const scopes = import.meta.env.VITE_TWITCH_SCOPES;

  if (!clientId || !redirectUri || !scopes) {
    throw new Error("Variables de entorno VITE_TWITCH_* no están definidas correctamente");
  }
  const params = new URLSearchParams({
    response_type: "code",
    client_id: clientId,
    redirect_uri: redirectUri,
    scope: scopes,
  });

  return `https://id.twitch.tv/oauth2/authorize?${params.toString()}`;
}
