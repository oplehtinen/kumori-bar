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
	import MediaControlButton from './MediaControlButton.svelte';
	import LikeIcon from './Icons/LikeIcon.svelte';
	import NotLikeIcon from './Icons/NotLikeIcon.svelte';
	import { saveTracks, removeSavedTracks } from '$lib';
	import { PUBLIC_SPOTIFY_CLIENT_ID } from '$env/static/public';
	const clientId = PUBLIC_SPOTIFY_CLIENT_ID;
	let sdk: SpotifyApi;
	let token: AccessToken;
	let nowPlaying: string;
	let isSaved: boolean;
	async function startOAuthFlow() {
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
				token = await getSpotifyToken(clientId, code, challenge);
				console.log('access token from oauth:' + token);
				sdk = SpotifyApi.withAccessToken(clientId, token);
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
	onMount(async () => {
		startOAuthFlow();
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
			if (isSaved) {
				await removeSavedTracks(token.access_token, [nowPlaying]);
			} else {
				await saveTracks(token.access_token, [nowPlaying]);
			}
			isSaved = !isSaved;
		}
	};
	updateSignal.subscribe(async () => {
		if (token) {
			const refreshData = await tryRefreshToken(clientId, token);
			if (!refreshData.success) {
				console.error('Token refresh failed, going through OAuth flow');
				await startOAuthFlow();
			} else if (refreshData.token) {
				token = refreshData.token;
				sdk = SpotifyApi.withAccessToken(clientId, token);
			}
		}
		if (sdk) {
			const state = await sdk.player.getCurrentlyPlayingTrack();
			nowPlaying = state?.item?.id ?? null;
			console.log(state);
			const hasSaved = await sdk.currentUser.tracks.hasSavedTracks([nowPlaying]);
			isSaved = hasSaved[0];
		}
	});
</script>

{#if sdk}
	<MediaControlButton
		icon={isSaved ? LikeIcon : NotLikeIcon}
		color="success"
		onClick={() => likeSong()}
	/>
{/if}
