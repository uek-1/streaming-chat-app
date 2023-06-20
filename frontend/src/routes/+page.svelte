<script language = "typescript">
  import {onMount} from "svelte";
  let ws;
  
  $: inputContent = "";
  $: content = null;
  $: receivedMessages = [];
  
  onMount (() => {
    console.log(window.location.host.split(":"));
    ws = new WebSocket(`ws://${window.location.host.split(':')[0]}:3000/ws`)
    ws.addEventListener("message", handleMessage);
    console.log("setup");
    return () => {
      ws.close();
      console.log("closed");
    }
  });


  function handleMessage(msg) {
    console.log("test");
    let data = JSON.parse(msg.data);
    console.log(data);
    content = data.content;
    console.log(content);
    receivedMessages.push(`${content}`);
    receivedMessages = receivedMessages;
  }

  async function onClick(e) {
    console.log("SENT", inputContent);
    ws.send(inputContent);
    inputContent = "";
  }

</script>

<h1>CHAT:</h1>
<table class = "chat-table"> 
  {#each receivedMessages as item}
    <tr>{item}</tr>
  {/each}
</table>
<form on:submit={onClick}>
  <input type="text" bind:value={inputContent}>
  <button>"Chat"</button>
</form>
