<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {documentDir} from "@tauri-apps/api/path";

async function fetchData() {
  const item = document.getElementById('item').value;

  if (item == "") {
    document.getElementById('price').innerText = 'Please enter the item name!';
    return;
  }

  try {
    const response = await fetch('https://api.tarkov.dev/graphql', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
      body: JSON.stringify({
        query: `{
                    items(name: "${item}") {
                        name
                        avg24hPrice
                    }
                }`
      })
    });

    const data = await response.json();

    if (data && data.data.items.length > 0) {
      document.getElementById('price').innerText = "NAME: " + data.data.items[0].name + "\nPRICE: " + data.data.items[0].avg24hPrice;
      await saveItems();
    } else {
      document.getElementById('price').innerText = 'No data available';
    }
  } catch (error) {
    document.getElementById('price').innerText = 'Error fetching data';
    await saveItems();
  }
}

const saveItems = async() => {
  let dir = await documentDir();
  let appDir = dir + "\TarkovBuddy\\";
  await invoke("storePrice", {path: appDir, contents: document.getElementById('price').innerText});

}

</script>

<template>
  <form class="row" @submit.prevent="fetchData">
    <label for="item">Item name</label>
    <input type="text" id="item" name="item">
    <button type="submit">Search</button>
  </form>
  <p id="price"></p>
</template>

<style scoped>

</style>