<script>
    import { goto } from '$app/navigation';
    import {invoke} from '@tauri-apps/api/tauri';
    import {StatusCodes} from 'http-status-codes';
    import {onMount} from 'svelte';
    import Loader from '$lib/components/Loader.svelte';

    let username = '';
    let password = '';
    let errorMessage = '';
    /**
	 * @type {HTMLInputElement}
	 */
    let passwordInput;

    let isLoading = false;
    let authChecked = false;
    onMount(() => {
        invoke('check_auth').then((res) => {
            if (res) {
                goto('/dashboard/home');
            }
            else{
                authChecked = true;
            }
        });
        
    });
    async function loginFunction() {
        isLoading = true;
        errorMessage = '';
        try {
            if (username == '') {
                errorMessage = 'Error: Username cannot be empty.';
                return;
            }
            else if(password == '') {
                errorMessage = 'Error: Password cannot be empty.';
                return;
            }
            else {
                let stat = await invoke('login', {username: username, password: password});
                console.log('Login status: ' + stat);
                if (stat == StatusCodes.OK) {
                    goto('/dashboard/home');
                } else {
                    errorMessage = 'Error: Invalid username or password.';
                }
            }
        } finally {
            isLoading = false;
        }
    }

    /**
	 * @param {{ key: string; }} event
	 */
    function handleUsernameKeyDown(event) {
        if (event.key === 'Enter') {
            passwordInput.focus(); // Focus on the password input when Enter is pressed
        }
    }

    /**
	 * @param {{ key: string; }} event
	 */
    function handlePasswordKeyDown(event) {
        if (event.key === 'Enter') {
            loginFunction(); // Trigger the login function when Enter is pressed
        }
    }

</script>
<div class="page">
{#if authChecked}
<div class="login-container">
    <div class="image-container">
        <!-- svelte-ignore a11y-img-redundant-alt -->
        <img src="/assets/stargazer_black_vert_transparent.png" alt="Login Image">
    </div>
    <div class="form-container">
        
                <p class="error-message">{errorMessage}</p>
        
        <input type="text"
                bind:value={username}
                placeholder="Username"
                class="login-input"
                on:keydown={handleUsernameKeyDown}
                autofocus>

            <input type="password"
                bind:value={password}
                placeholder="Password"
                class="login-input"
                on:keydown={handlePasswordKeyDown}
                bind:this={passwordInput}>
            {#if isLoading}
                <div class="loader-container">
                    <Loader />
                </div>  
            {:else}
                <button on:click={loginFunction}>Login</button> <!-- Display button otherwise -->
            {/if}
    </div>
</div>
{:else}
    <Loader />
{/if}
</div>
  
<style>
    .page {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 20px;
        height: 100vh;
        background-color: #f5f5f5;
    }
    .login-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        background-color: white;
        padding: 25px;
        border-radius: 10px;
        width: 350px;
        height: 450px;
        box-shadow: 0 0 10px #ddd;
    }

    .image-container img {
    width: 300px;
    height: 200px;
    }

    .form-container {
        display: flex;
        flex-direction: column;
        width: 300px;
    }

    .login-input {
    margin: 10px 0;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 4px;
    }

    button {
        padding: 8.5px 20px;
        margin: 10px 0;
        border: none;
        border-radius: 4px;
        background-color: #D4AF37;
        color: rgb(255, 255, 255);
        font-size: large;
        cursor: pointer;
        border: 1px solid;
    }

    .loader-container {
        display: flex;
        justify-content: center;
        align-items: center;
        padding: 18px;

    }
    button:hover {
    background-color: #ac8b1f;
    }

    .error-message {
        color: red;
        font-size: medium;
        margin: 0px 0;
        height:24px;
    }
</style>