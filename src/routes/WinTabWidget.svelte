<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import ClickButton from './ClickButton.svelte';
	import MetaIcon from './Icons/MetaIcon.svelte';
	const winTabPress = () => {
		invoke('simulate_windows_tab');
	};
</script>

<ClickButton onClick={() => winTabPress()} icon={MetaIcon} color="primary"></ClickButton>
