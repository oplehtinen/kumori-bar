<script lang="ts">
	import { onMount } from 'svelte';
	import { SpotifyApi, type AccessToken } from '@spotify/web-api-ts-sdk';
	import {
		start,
		cancel,
		onUrl,
		type OauthConfig,
		onInvalidUrl
	} from '@fabianlars/tauri-plugin-oauth';
	import { getSpotifyCode, getSpotifyToken, tryRefreshToken } from '$lib';
	import { updateSignal } from '../store/update';
	import MediaControlButton from './ClickButton.svelte';
	import LikeIcon from './Icons/LikeIcon.svelte';
	import NotLikeIcon from './Icons/NotLikeIcon.svelte';
	import { saveTracks, removeSavedTracks } from '$lib';
	import { PUBLIC_SPOTIFY_CLIENT_ID } from '$env/static/public';
	import { load, Store } from '@tauri-apps/plugin-store';
	const clientId = PUBLIC_SPOTIFY_CLIENT_ID;
	let sdk: SpotifyApi;
	let token: AccessToken;
	let nowPlaying: string;
	let isSaved: boolean;
	let store: Store;
	async function startOAuthFlow(store: Store) {
		try {
			const config: OauthConfig = {
				ports: [8889],
				response: ''
			};
			const port = await start(config);
			console.log(`OAuth server started on port ${port}`);
			const challenge = await getSpotifyCode(clientId);
			// Set up listeners for OAuth results
			await onUrl(async (url) => {
				const uri = new URL(url);
				const code = uri.searchParams.get('code');
				if (!code) {
					console.error('No code found in URL:', url);
					return;
				}
				const oauthToken = await getSpotifyToken(clientId, code, challenge);
				console.log('validating token..');
				tryInitSdk(clientId, oauthToken, store);
				const state = await sdk.player.getCurrentlyPlayingTrack();
				nowPlaying = state?.item?.id ?? null;
				stopOAuthServer();
			});
		} catch (error) {
			stopOAuthServer();
			console.error('Error starting OAuth server:', error);
		}
	}

	// Don't forget to stop the server when you're done
	async function stopOAuthServer() {
		try {
			await cancel(8889);
			console.log('OAuth server stopped');
		} catch (error) {
			console.error('Error stopping OAuth server:', error);
		}
	}
	async function tryInitSdk(clientId: string, currentToken: AccessToken, store: Store) {
		if (currentToken) {
			tryInitToken(clientId, currentToken, store);
		}
		if (sdk) {
			const state = await sdk.player.getCurrentlyPlayingTrack();
			nowPlaying = state?.item?.id ?? null;
			console.log(state);
			const hasSaved = await sdk.currentUser.tracks.hasSavedTracks([nowPlaying]);
			isSaved = hasSaved[0];
		}
	}
	async function tryInitToken(clientId: string, currentToken: AccessToken, store: Store) {
		const refreshData = await tryRefreshToken(clientId, currentToken);
		if (!refreshData.success) {
			console.error('Token refresh failed, going through OAuth flow');
			await startOAuthFlow(store);
		} else if (refreshData.token) {
			token = refreshData.token;
			console.log('token ok');
			console.log(token);
			store.set('token', token);
			sdk = SpotifyApi.withAccessToken(clientId, token);
		}
	}
	onMount(async () => {
		store = await load('spotify.json', { autoSave: true });
		const storeToken = await store.get<AccessToken>('token');
		if (storeToken) {
			console.log('token found from store, validating');
			tryInitSdk(clientId, storeToken, store);
		} else {
			console.log('no token stored, go to oauth flow');
			startOAuthFlow(store);
		}
	});
	const likeSong = async () => {
		if (!sdk) {
			console.error('SDK not initialized');
			return;
		}
		if (!nowPlaying) {
			console.error('No song playing');
			return;
		}
		if (sdk && nowPlaying) {
			console.log(token);
			if (isSaved) {
				await removeSavedTracks(token.access_token, [nowPlaying]);
			} else {
				await saveTracks(token.access_token, [nowPlaying]);
			}
			isSaved = !isSaved;
		}
	};
	updateSignal.subscribe(async () => {
		tryInitSdk(clientId, token, store);
	});
</script>

{#if sdk}
	<MediaControlButton
		icon={isSaved ? LikeIcon : NotLikeIcon}
		color="success"
		onClick={() => likeSong()}
	/>
{/if}
