<script>
import { onMount } from 'svelte'
export let title = ''
let status = 'Pending'
let body = ''
export let test
let open
const colors = {
	Pending: 'info',
	Error: 'danger',
	Success: 'success',
}

let onSuccess = (res) => {
		status = 'Success'
		body = JSON.stringify(res, null, 2)
}

let onFailure = (err) => {
	status = 'Error'
	body = JSON.stringify(err, null, 2)
}

let onTimeout = () => {
	if (status === 'Pending') {
		status = 'Error'
		body = 'Timeout'
	}
}

onMount(() => {
	test().then(onSuccess, onFailure)
	setTimeout(onTimeout, 30*1000)
})
</script>

<div class="message is-{colors[status]}">
	<div class="message-header">
		<p>{title}</p>
    <button class="delete" aria-label="delete" on:click={() => open=!open}></button>
	</div>
	{#if open}
		<div class="message-body has-text-left is-size-7">
			<pre>
				{@html body}
			</pre>
		</div>
	{/if}
</div>