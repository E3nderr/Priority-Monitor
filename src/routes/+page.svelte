<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  type Rule = { process_name: string; priority: string; enabled: boolean };
  const levels = ["Idle", "BelowNormal", "Normal", "AboveNormal", "High"];

  let rules = $state<Rule[]>([]);
  let procs = $state<string[]>([]);
  let name = $state("");
  let level = $state("Normal");

  onMount(async () => {
    rules = await invoke("list_rules");
    const p: { name: string }[] = await invoke("list_running_processes");
    procs = [...new Set(p.map((x) => x.name))].sort();
  });

  const persist = () => invoke("save_rules", { list: $state.snapshot(rules) });

  async function add() {
    if (!name.trim()) return;
    rules.push({ process_name: name.trim(), priority: level, enabled: true });
    name = "";
    await persist();
  }
  async function remove(i: number) {
    rules.splice(i, 1);
    await persist();
  }
</script>

<main class="min-h-screen bg-zinc-900 p-6 text-zinc-100">
  <h1 class="mb-4 text-2xl font-semibold">Priority Monitor</h1>

  <div class="mb-6 flex gap-2">
    <input list="procs" bind:value={name} placeholder="Process name"
      class="flex-1 rounded-lg bg-zinc-800 px-3 py-2 outline-none" />
    <datalist id="procs">{#each procs as p}<option value={p}></option>{/each}</datalist>
    <select bind:value={level} class="rounded-lg bg-zinc-800 px-3 py-2">
      {#each levels as l}<option>{l}</option>{/each}
    </select>
    <button onclick={add} class="rounded-lg bg-indigo-600 px-4 py-2 hover:bg-indigo-500">Add</button>
  </div>

  <ul class="space-y-2">
    {#each rules as r, i}
      <li class="flex items-center gap-3 rounded-lg bg-zinc-800 px-3 py-2">
        <input type="checkbox" bind:checked={r.enabled} onchange={persist} />
        <span class="flex-1">{r.process_name}</span>
        <select bind:value={r.priority} onchange={persist} class="rounded bg-zinc-700 px-2 py-1">
          {#each levels as l}<option>{l}</option>{/each}
        </select>
        <button onclick={() => remove(i)} class="text-zinc-400 hover:text-red-400">✕</button>
      </li>
    {/each}
  </ul>
</main>