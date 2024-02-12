
<script>
    export let size = '30px'; // Default size if not provided
</script>
<div class="loader-container">
    <div class="loader" style="width: {size}; height: {size}; border-width: {parseInt(size) / 7}px;"></div> <!-- Display loader when loading -->
</div>

<style>
    .loader-container{
        display: flex;
        justify-content: center;
        align-items: center;}
    .loader {
        border: 4px solid #f3f3f3; /* Light grey */
        border-top: 4px solid #D4AF37; 
        border-radius: 50%;
        animation: spin 2s linear infinite;
    }

    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
</style>