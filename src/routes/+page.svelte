<script>
    import { goto } from '$app/navigation';
    import {invoke} from '@tauri-apps/api/tauri';
    import {StatusCodes} from 'http-status-codes';
    let username = '';
    let password = '';


    async function loginFunction() {
        console.log('Login function called');
        let stat = await invoke('login', {username: username, password: password});
         console.log('Login status: ' + stat);
        if (stat == StatusCodes.OK) {
            goto('/dashboard');
        } else {
            console.log('Login failed');
        }
    }

</script>
  
<div class="login-container">
<div class="image-container">
    <!-- svelte-ignore a11y-img-redundant-alt -->
    <img src="src\lib\assets\stargazer_black_vert_transparent.png" alt="Login Image">
</div>
<div class="form-container">
    <input type="text" bind:value={username} placeholder="Username" class="login-input">
    <input type="password" bind:value={password} placeholder="Password" class="login-input">
    <button on:click={loginFunction}>Login</button>
</div>
</div>
  
<style>
    .login-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 20px;
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
    padding: 8px 20px;
    margin: 10px 0;
    border: none;
    border-radius: 4px;
    background-color: #D4AF37;
    color: rgb(255, 255, 255);

    font-size: large;
    cursor: pointer;
    
    }

    button:hover {
    background-color: #ac8b1f;
    }
</style>