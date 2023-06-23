<script language = "typescript">
  import {onMount} from "svelte";
  import Login from './Login.svelte'
  let ws;
  
  $: inputContent = "";
  $: content = null;
  $: receivedMessages = [];
  let thisUsername = "";

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

  async function onSubmit(e) {
    console.log(thisUsername, "SENT", inputContent);
    ws.send(`${thisUsername} : ${inputContent}`);
    inputContent = "";
  }

  function createChat(username) {
    console.log(username.detail.username);
    thisUsername = username.detail.username;
  }

</script>

<style>
  .chat-table {
    color : purple
  }
</style>

<Login on:username={createChat}/>
{#if thisUsername != ""}
  <h1>CHAT:</h1>

  <table class = "chat-table"> 
    {#each receivedMessages as item}
      <tr>{item}</tr>
    {/each}
  </table>
  <form on:submit={onSubmit}>
    <input type="text" bind:value={inputContent} autofocus>
    <button>"Chat"</button>
  </form>
{/if}
