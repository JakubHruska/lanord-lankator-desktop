<script setup>
</script>

<template>
  <div class="view-container">
    <h1>Obchod</h1>
    <div class="card-grid">
      <div class="card" v-for="i in 6" :key="i">
        <div class="card-image" />
        <div class="card-body">
          <h3>Hra {{ i }}</h3>
          <p>Popis hry {{ i }}. Lorem ipsum dolor sit amet.</p>
          <button class="buy-btn">Koupit — {{ i * 100 }} Kč</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.view-container {
  padding: 2rem;
}

.card-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 1.5rem;
  margin-top: 1rem;
}

.card {
  border: 1px solid #ddd;
  border-radius: 8px;
  overflow: hidden;
  background: #fff;
  box-shadow: 0 2px 6px rgba(0,0,0,0.08);
}

.card-image {
  width: 100%;
  height: 130px;
  background: linear-gradient(135deg, #667eea, #764ba2);
}

.card-body {
  padding: 1rem;
}

.card-body h3 {
  margin: 0 0 0.5rem;
}

.card-body p {
  font-size: 0.875rem;
  color: #555;
  margin: 0 0 1rem;
}

.buy-btn {
  background: #667eea;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 0.4rem 1rem;
  cursor: pointer;
  width: 100%;
}

.buy-btn:hover {
  background: #5a6fd6;
}
</style>
