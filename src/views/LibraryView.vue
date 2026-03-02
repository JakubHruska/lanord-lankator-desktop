<script setup>
const games = [
  { id: 1, title: "Hra Alpha", playtime: "12 h" },
  { id: 2, title: "Hra Beta", playtime: "5 h" },
  { id: 3, title: "Hra Gamma", playtime: "34 h" },
];
</script>

<template>
  <div class="view-container">
    <h1>Knihovna</h1>
    <div class="game-list">
      <div class="game-row" v-for="game in games" :key="game.id">
        <div class="game-icon" />
        <div class="game-info">
          <strong>{{ game.title }}</strong>
          <span>Odehráno: {{ game.playtime }}</span>
        </div>
        <button class="play-btn">Hrát</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.view-container {
  padding: 2rem;
}

.game-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  margin-top: 1rem;
}

.game-row {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.75rem 1rem;
  border: 1px solid #ddd;
  border-radius: 8px;
  background: #fff;
}

.game-icon {
  width: 56px;
  height: 56px;
  border-radius: 6px;
  background: linear-gradient(135deg, #f093fb, #f5576c);
  flex-shrink: 0;
}

.game-info {
  display: flex;
  flex-direction: column;
  flex: 1;
}

.game-info span {
  font-size: 0.85rem;
  color: #777;
}

.play-btn {
  background: #4caf50;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 0.4rem 1.2rem;
  cursor: pointer;
}

.play-btn:hover {
  background: #43a047;
}
</style>
