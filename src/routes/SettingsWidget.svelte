<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { getAllWebviewWindows, WebviewWindow } from '@tauri-apps/api/webviewWindow';
	import { Window } from '@tauri-apps/api/window';
	import { Webview } from '@tauri-apps/api/webview';
	const openSettings = async () => {
		invoke('open_settings_window');
		/* 				const webview = new WebviewWindow('kumori-settings75467458', {
			url: 'https://github.com/tauri-apps/tauri'
		});
		
		webview.once('tauri://error', function (e) {
			console.error(e);
		}); */
		/* 	const window = new Window('kumori-settings', {});
		await new Promise((resolve) => setTimeout(resolve, 500));
		const webview = new Webview(window, 'kumori-settings', {
			dragDropEnabled: false,
			devtools: false,
			incognito: true,
			useHttpsScheme: false,
			url: '/settings',
			x: 100,
			y: 100,
			width: 600,
			height: 400
		}); */
		const windows = await getAllWebviewWindows();
		console.log(...windows);
	};
</script>

<svelte:window on:contextmenu|preventDefault={openSettings} />
