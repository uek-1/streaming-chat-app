<script>
    import { createEventDispatcher } from "svelte";
    import { destroy_each, get_current_component } from "svelte/internal";

  $: username = "";
  const thisComponent = get_current_component();
  const dispatch = createEventDispatcher();

  function pickedUsername(e) {  
    console.log(username);
    dispatch('username', {
        username : `${username}`,
      }); 
    destroySelf();
    }

  function destroySelf() {
      console.log("destroyed username!");
      thisComponent.$destroy();
    }
</script>

<h1>Choose Username</h1>
<form on:submit={pickedUsername}>
  <input type = "text" bind:value={username}>
  <button>"Proceed"</button>
</form>
