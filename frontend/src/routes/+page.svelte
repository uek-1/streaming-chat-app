<script language = "typescript">
  import {onMount} from "svelte";
  import Login from './Login.svelte'
  let ws;
  
  $: inputContent = "";
  $: content = null;
  $: receivedMessages = [];
  $: userColors = {"test" : "blue"};
  let thisUsername = "";
  let time = new Date();
  const validUsernameColors = ["blue", "red", "yellow", "green", "orange", "violet" , "indigo"];

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
    console.log(msg);
    let data = JSON.parse(msg.data);
    console.log(data);
    receivedMessages.push({
      time : data.time.replaceAll("\"", ""),  
      username: data.username.replaceAll("\"", ""), 
      message: data.message.replaceAll("\"", "")
    });
    receivedMessages = receivedMessages;
  }

  function getUsernameColor(username) {
    if (userColors[username] != undefined) {
      return userColors[username];
    }
    userColors[username] = validUsernameColors[Math.floor(Math.random() * validUsernameColors.length)];
    return userColors[username];
  }

  async function onSubmit(e) {
    let hourString = (time.getHours() % 12).toString();
    if (hourString.length < 2) {
      hourString = `0${hourString}`;
    }
  
    let minuteString = time.getMinutes().toString();
    if (minuteString.length < 2) {
      minuteString = `0${minuteString}`
    }
    const chat_message = {
      time : `${hourString}:${minuteString}`,
      username : thisUsername,
      message : inputContent
    };

    console.log(chat_message);
    ws.send(JSON.stringify(chat_message));
    inputContent = "";
  }

  function createChat(username) {
    console.log(username.detail.username);
    thisUsername = username.detail.username;
  }

</script>

<style>
  .chat-box {
    width : 30vw; 
    height : 80vh;
    outline-style : solid;
    outline-color : black;
    overflow-y: scroll;
  }

  .chat-table {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
  }

  .chat-table-time {
    color : grey;
    align-self : left;
  }
  
  .chat-table-username {
    font-weight : bold;
    align-self : left;

  }

  .chat-table-message {
    color: black;
  }

  .chat-form {
    align-self : bottom;
  }
</style>

<center><Login on:username={createChat}/>
{#if thisUsername != ""}
  <h1>CHAT:</h1>
  <div class = "chat-box">
    <table class = "chat-table"> 
      {#each receivedMessages as item}
        <tr>
          <td class = "chat-table-time">{item.time}</td>
          <td style = {`color : ${getUsernameColor(item.username)}`} class = "chat-table-username">
            {item.username}
          </td>
          <td class = "chat-table-message">{item.message}</td>
        </tr>
      {/each}
    </table>
    <form on:submit={onSubmit} class = "chat-form">
      <input type="text" bind:value={inputContent} autofocus>
      <button>"Chat"</button>
    </form>
  </div>
{/if}
</center>
