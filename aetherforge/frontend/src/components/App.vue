<template>
    <div id="app">
      <PuzzleInterface />
      <MiningStats />
      <ValidatorDashboard />
    </div>
  </template>
  
  <script>
  import PuzzleInterface from "./components/PuzzleInterface.vue";
  import MiningStats from "./components/MiningStats.vue";
  import ValidatorDashboard from "./components/ValidatorDashboard.vue";
  
  export default {
    components: {
      PuzzleInterface,
      MiningStats,
      ValidatorDashboard,
    },
  };
  </script>