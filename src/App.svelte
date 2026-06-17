<script lang="ts">
  import { onMount } from 'svelte';

  let count = $state(0);

  function increment() {
    count += 1;
  }
</script>

<main class="flex h-screen items-center justify-center bg-slate-50 dark:bg-slate-950">
  <div class="text-center">
    <h1 class="text-4xl font-bold text-brand-forest dark:text-brand-spring">
      MongoDesk
    </h1>
    <p class="mt-4 text-base text-slate-600 dark:text-slate-400">
      Ultra-fast, lightweight MongoDB desktop client
    </p>
    <button
      class="mt-8 rounded-sm bg-brand-evergreen px-6 py-3 text-sm font-medium text-white hover:bg-brand-spring-dim"
      onclick={increment}
    >
      Count: {count}
    </button>
  </div>
</main>
