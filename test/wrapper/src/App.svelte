<script>
	import Test from './components/Test.svelte'
	import { ZeroFrame } from './lib/ZeroFrame.js'
	export let name
	window.f = new ZeroFrame()

	const tests_api = [
		{
			title: "Announcer Info",
			fn: () => window.f.cmdp("announcerInfo", {}),
		},
		{
			title: "Cert Add",
			fn: () => window.f.cmdp("certAdd", {}),
		},
		{
			title: "Cert Select",
			fn: () => window.f.cmdp("certSelect", {}),
		},
		{
			title: "Channel Join",
			fn: () => window.f.cmdp("channelJoin", {}),
		},
		{
			title: "DB Query",
			fn: () => window.f.cmdp("dbQuery", {}),
		},
		{
			title: "Dir List",
			fn: () => window.f.cmdp("dirList", {}),
		},
		{
			title: "File Delete",
			fn: () => window.f.cmdp("fileDelete", {}),
		},
		{
			title: "File Get",
			fn: () => window.f.cmdp("fileGet", {}),
		},
		{
			title: "File List",
			fn: () => window.f.cmdp("fileList", {}),
		},
		{
			title: "File Need",
			fn: () => window.f.cmdp("fileNeed", {}),
		},
		{
			title: "File Query",
			fn: () => window.f.cmdp("fileQuery", {}),
		},
		{
			title: "File Rules",
			fn: () => window.f.cmdp("fileRules", {}),
		},
		{
			title: "File Write",
			fn: () => window.f.cmdp("fileWrite", {}),
		},
		{
			title: "Ping",
			fn: () => window.f.cmdp("ping", {}),
		},
		{
			title: "Server Info",
			fn: () => window.f.cmdp("serverInfo", {}),
		},
		{
			title: "Site Info",
			fn: () => window.f.cmdp("siteInfo", {}),
		},
		{
			title: "Site Publish",
			fn: () => window.f.cmdp("sitePublish", {}),
		},
		{
			title: "Site Reload",
			fn: () => window.f.cmdp("siteReload", {}),
		},
		{
			title: "User Get Global Settings",
			fn: () => window.f.cmdp("userGetGlobalSettings", {}),
		},
		{
			title: "User Set Global Settings",
			fn: () => window.f.cmdp("userSetGlobalSettings", {}),
		},
	]
	let tests_bigfile = [
		{
			title: "Big File Upload Init",
			fn: () => window.f.cmdp("bigFileUploadInit", {}),
		},
	]
</script>

<main>
	<h1>Hello {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
	{#each tests_api as test}
		<Test title={test.title} test={test.fn}></Test>
	{/each}
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>