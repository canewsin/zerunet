<script>
	import Test from './components/Test.svelte'
	import { ZeroFrame } from './lib/ZeroFrame.js'
	export let name
	window.f = new ZeroFrame()

	const tests_api = {
		title: "ZeroNet API",
		tests: [
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
		],
	}
	let tests_bigfile = {
		title: "Bigfile API",
		tests: [
			{
				title: "Big File Upload Init",
				fn: () => window.f.cmdp("bigFileUploadInit", {}),
			},
		],
	}
	let tests_cors = {
		title: "Cors API",
		tests: [
			{
				title: "Cors Permission",
				fn: () => window.f.cmdp("corsPermission", {}),
			},
		],
	}
	let tests_multiuser = {
		title: "MultiUser API",
		tests: [
			{
				title: "User Login Form",
				fn: () => window.f.cmdp("userLoginForm", {}),
			},
			{
				title: "User Show Master Seed",
				fn: () => window.f.cmdp("userShowMasterSeed", {}),
			},
		],
	}
	let tests_cryptmessage = {
		title: "CryptMessage API",
		tests: [
			{
				title: "User Public Key",
				fn: () => window.f.cmdp("userPublickey", {}),
			},
			{
				title: "Ecies Encrypt",
				fn: () => window.f.cmdp("eciesEncrypt", {}),
			},
			{
				title: "Ecies Decrypt",
				fn: () => window.f.cmdp("eciesDecrypt", {}),
			},
			{
				title: "Aes Encrypt",
				fn: () => window.f.cmdp("aesEncrypt", {}),
			},
			{
				title: "Aes Decrypt",
				fn: () => window.f.cmdp("aesDecrypt", {}),
			},
		],
	}
	let tests_newsfeed = {
		title: "Newsfeed API",
		tests: [
			{
				title: "Feed Follow",
				fn: () => window.f.cmdp("feedFollow", {}),
			},
			{
				title: "Feed List Follow",
				fn: () => window.f.cmdp("feedListFollow", {}),
			},
			{
				title: "Feed Query",
				fn: () => window.f.cmdp("feedQuery", {}),
			},
		],
	}
	let tests_mergersite = {
		title: "Mergersite API",
		tests: [
			{
				title: "Merger Site Add",
				fn: () => window.f.cmdp("mergerSiteAdd", {}),
			},
			{
				title: "Merger Site Delete",
				fn: () => window.f.cmdp("mergerSiteDelete", {}),
			},
			{
				title: "Merger Site List",
				fn: () => window.f.cmdp("mergerSiteList", {}),
			},
		],
	}
	let tests_mute = {
		title: "Mute API",
		tests: [
			{
				title: "Mute Add",
				fn: () => window.f.cmdp("muteAdd", {}),
			},
			{
				title: "Mute Remove",
				fn: () => window.f.cmdp("muteRemove", {}),
			},
			{
				title: "Mute List",
				fn: () => window.f.cmdp("muteList", {}),
			},
		],
	}
	let tests_optionalmanager = {
		title: "Optional Manager API",
		tests: [
			{
				title: "Optional File List",
				fn: () => window.f.cmdp("optionalFileList", {}),
			},
			{
				title: "Optional File Info",
				fn: () => window.f.cmdp("optionalFileInfo", {}),
			},
			{
				title: "Optional File Pin",
				fn: () => window.f.cmdp("optionalFilePin", {}),
			},
			{
				title: "Optional File Unpin",
				fn: () => window.f.cmdp("optionalFileUnpin", {}),
			},
			{
				title: "OptionalFileDelete",
				fn: () => window.f.cmdp("optionalFileDelete", {}),
			},
			{
				title: "OptionalLimitStats",
				fn: () => window.f.cmdp("optionalLimitStats", {}),
			},
			{
				title: "Optional Limit Set",
				fn: () => window.f.cmdp("optionalLimitSet", {}),
			},
			{
				title: "Optional Help List",
				fn: () => window.f.cmdp("optionalHelpList", {}),
			},
			{
				title: "Merger Help",
				fn: () => window.f.cmdp("optionalHelp", {}),
			},
			{
				title: "Merger Help Remove",
				fn: () => window.f.cmdp("optionalHelpRemove", {}),
			},
			{
				title: "Optional Help All",
				fn: () => window.f.cmdp("optionalHelpAll", {}),
			},
		],
	}
	let tests_admin = {
		title: "Admin API",
		tests: [
			{
				title: "Execute command as other site",
				fn: () => window.f.cmdp("as", {}),
			},
			{
				title: "Cert List",
				fn: () => window.f.cmdp("certList", {}),
			},
			{
				title: "Cert Set",
				fn: () => window.f.cmdp("certSet", {}),
			},
			{
				title: "Channel Join All Site",
				fn: () => window.f.cmdp("channelJoinAllsite", {}),
			},
			{
				title: "ConfigSet",
				fn: () => window.f.cmdp("configSet", {}),
			},
			{
				title: "Server Portcheck",
				fn: () => window.f.cmdp("serverPortcheck", {}),
			},
			{
				title: "Server shutdown",
				fn: () => window.f.cmdp("serverShutdown", {}),
			},
			{
				title: "Server update",
				fn: () => window.f.cmdp("serverUpdate", {}),
			},
			{
				title: "Site clone",
				fn: () => window.f.cmdp("siteClone", {}),
			},
			{
				title: "Site list",
				fn: () => window.f.cmdp("siteList", {}),
			},
			{
				title: "Site Pause",
				fn: () => window.f.cmdp("sitePause", {}),
			},
			{
				title: "Site Resume",
				fn: () => window.f.cmdp("siteResume", {}),
			},
		],
	}
	let test_suites = [
		tests_api,
		tests_bigfile,
		tests_cors,
		tests_multiuser,
		tests_cryptmessage,
		tests_newsfeed,
		tests_mergersite,
		tests_mute,
		tests_optionalmanager,
		tests_admin,
	]
</script>

<main>
	<h1>Hello {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
	{#each test_suites as suite}
	<section class="section">
		<div class="container">
			<h2 class="title">{suite.title}</h2>
			{#each suite.tests as test}
				<Test title={test.title} test={test.fn}></Test>
			{/each}
		</div>
	</section>
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