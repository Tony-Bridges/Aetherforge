<template>
    <div>
      <h2>Puzzle Interface</h2>
      <button @click="solvePuzzle">Solve Puzzle</button>
    </div>
  </template>
  
  <script>
  export default {
    methods: {
      solvePuzzle() {
        alert("Puzzle solved!");
      },
    },
  };
  </script>