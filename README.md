# Banter
Cross-platform App to get Twitch Chat (on real time) 
🔑 Orden recomendado de módulos
Auth (services/auth)

Implementar el OAuth2 con Twitch (Authorization Code Flow).

Aquí se maneja el login, el intercambio de código por token, y el refresco de credenciales.

Resultado: un AccessToken y RefreshToken que se guardan en storage.

Storage

Encapsular cómo se guardan y leen las credenciales (ej. en archivo local, SQLite, o secure storage).

Así services/auth no depende de detalles de persistencia.

Services/twitch

Una vez tienes credenciales, este módulo se encarga de:

Conexión IRC con el token.

Opcional: llamadas REST a la API de Twitch (ej. obtener info del canal, usuarios).

Este módulo nunca pide login directamente, solo consume tokens de storage.

UI

Se conecta a services/auth para disparar el login.

Muestra estado de sesión (logueado/no logueado).

Consume eventos de services/twitch para renderizar mensajes.

⚙️ Flujo modular
UI → Auth Service → Twitch OAuth → Storage

UI → Twitch Service → IRC/REST → Storage (tokens)

De esta forma:

El login está desacoplado de la lógica de chat.

storage es el punto común para credenciales y datos persistentes.

services se dividen en auth y twitch, cada uno con responsabilidades claras.