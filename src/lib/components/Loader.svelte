
<div class="loader-container">
    <div class="loader"></div> <!-- Display loader when loading -->
</div>

<style>
    .loader-container{
        padding: 10px 0;
        display: flex;
        justify-content: center;
        align-items: center;}
    .loader {
        border: 4px solid #f3f3f3; /* Light grey */
        border-top: 4px solid #D4AF37; 
        border-radius: 50%;
        width: 30px;
        height: 30px;
        animation: spin 2s linear infinite;
    }

    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
</style>