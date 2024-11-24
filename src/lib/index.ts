// place files you want to import through the `$lib` alias in this folder.
const generateSpotifyCodeChallenge = async () => {
    const generateRandomString = (length: number) => {
        const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
        const values = crypto.getRandomValues(new Uint8Array(length));
        return values.reduce((acc, x) => acc + possible[x % possible.length], '');
    };
    const sha256 = async (plain: string) => {
        const encoder = new TextEncoder()
        const data = encoder.encode(plain)
        return window.crypto.subtle.digest('SHA-256', data)
    }
    const base64encode = (input: ArrayBuffer): string => {
        return btoa(String.fromCharCode(...new Uint8Array(input)))
            .replace(/=/g, '')
            .replace(/\+/g, '-')
            .replace(/\//g, '_');
    }
    const codeVerifier = generateRandomString(64);
    const hashed = await sha256(codeVerifier)
    const codeChallenge = base64encode(hashed);
    return { codeVerifier, codeChallenge };
}
import type { AccessToken } from '@spotify/web-api-ts-sdk';
import { open } from '@tauri-apps/plugin-shell';
const getSpotifyCode = async (clientId: string) => {
    const { codeVerifier, codeChallenge } = await generateSpotifyCodeChallenge();
    const scopes = ['user-read-currently-playing', 'playlist-modify-private', 'user-library-modify', 'user-library-read', 'user-read-playback-state'];
    const redirectUri = 'http://localhost:8889';

    const urlParams = new URLSearchParams({
        client_id: clientId,
        response_type: 'code',
        redirect_uri: redirectUri,
        code_challenge_method: 'S256',
        code_challenge: codeChallenge,
        scope: scopes.join(' ')
    });
    const url = new URL('https://accounts.spotify.com/authorize');
    url.search = urlParams.toString();
    await open(url.toString());
    return codeVerifier;
}
const getSpotifyToken = async (clientId: string, code: string, codeVerifier: string) => {

    const redirectUri = 'http://localhost:8889';
    const url = new URL('https://accounts.spotify.com/api/token');
    const body = new URLSearchParams({
        grant_type: 'authorization_code',
        code,
        redirect_uri: redirectUri,
        client_id: clientId,
        code_verifier: codeVerifier
    });
    const response = await fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: body
    });
    const data = await response.json();
    return data
}
/*  here I am, once again, writing custom code to interact with 
    the Spotify API because the official SDK is incorrect. 
    https://github.com/spotify/spotify-web-api-ts-sdk/pull/132 */
async function saveTracks(token: string, trackIds: string[]): Promise<void> {
    const url = 'https://api.spotify.com/v1/me/tracks';
    const response = await fetch(url, {
        method: 'PUT',
        headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ ids: trackIds })
    });

    if (!response.ok) {
        throw new Error('Failed to save tracks');
    }
}
async function removeSavedTracks(token: string, trackIds: string[]): Promise<void> {
    const url = 'https://api.spotify.com/v1/me/tracks';
    const response = await fetch(url, {
        method: 'DELETE',
        headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ ids: trackIds })
    });

    if (!response.ok) {
        throw new Error('Failed to remove tracks');
    }
}
type refreshReply = {
    token: AccessToken | null,
    success: boolean
}

async function tryRefreshToken(clientId: string, token: AccessToken): Promise<refreshReply> {
    const url = 'https://api.spotify.com/v1/me';
    if (!token || !token.refresh_token || !token.access_token) {
        return { token: null, success: false };
    }
    const response = await fetch(url, {
        method: 'GET',
        headers: {
            Authorization: `Bearer ${token.access_token}`
        }
    });
    if (response.status == 200) {
        return { token, success: true };
    }
    else if (response.status == 401) {
        const url = "https://accounts.spotify.com/api/token";
        const payload = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            body: new URLSearchParams({
                grant_type: 'refresh_token',
                refresh_token: token.refresh_token,
                client_id: clientId
            })
        }
        const body = await fetch(url, payload);
        const res = await body.json();
        const newToken = res.access_token;
        if (body.ok) {
            return { token: newToken, success: true };
        }
        else {
            return { token: null, success: false };
        }
    }
    return { token: null, success: false };
};
export { getSpotifyCode, getSpotifyToken, saveTracks, removeSavedTracks, tryRefreshToken };