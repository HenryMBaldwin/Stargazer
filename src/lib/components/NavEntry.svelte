<script>
    import { page } from '$app/stores';
    export let path = "/";
    export let name = "default";
    export let top = false; // Default size if not provided
</script>

<a href={path} class={top ? 'top-nav-element' : 'nav-element'} class:active={$page.url.pathname == path}>{name}</a>

<style>
    .nav-element {   
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: 100%;
        padding-bottom: 10px;
    }

    a:hover{
        background-color: #D4AF37;
        color: white;}

    .nav-element::before {
        padding-top: 10px;
        content: '';
        position: relative; /* This will center the pseudo-element */
        top: 0;
        height: 1px;
        width: 80%; /* The width of the border */
        border-top: 1px solid #D4AF37 /* The color of the border */
    }

    .top-nav-element {
        padding-top: 10px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: 100%;
        padding-bottom: 10px;
    }

    .top-nav-element:hover{
        background-color: #D4AF37;
        color: white;
    }

    a {
        text-decoration: none;
        color: #050505;
        font-size: 16px;
        font-family: "Aleo", sans-serif;
    }

    a.active {
        background-color: #D4AF37;
        color: #ffffff;
    }
</style>