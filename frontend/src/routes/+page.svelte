<script>
  export let test = ["test", "name"];
</script>

<h1>CHAT:</h1>
<table class = "chat-table"> 
  {#each test as item}
    <tr>{item}</tr>
  {/each}
</table>
